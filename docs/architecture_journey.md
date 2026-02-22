# 實戰 Leptos + Rust：從狀態混亂到架構覺醒的打字機開發紀實

> **觀看視頻版本：**
> 📺 [去嗶哩嗶哩看](https://www.bilibili.com/video/BV12KfFBsEtv/)

最近在爲 [Rime 打字機](https://github.com/lotem/typewriter)（一個基於 Rust 和 Leptos 框架開發的 CSR 網頁應用）添加 URL 狀態同步功能。原本以爲只是簡單地讀取幾個 Query 參數，沒想到卻引發了一場關於**前端狀態架構、領域驅動設計（DDD）、編譯器搏鬥，以及 Cloudflare 部署玄學**的全面重構。

這篇文章記錄了這次「折騰」的完整過程。如果你也在用 Rust 寫前端，或者對 SPA 的狀態管理與部署有疑惑，這篇覆盤或許能幫你少走幾個坑。

---

## 起：架構的分水嶺——當「下拉選單」遇上「URL」

一開始的需求很單純：
希望用戶能透過 URL 分享特定的輸入方案和佈局（例如 `/?theory=combo_pinyin&layout=ortho`），打開網頁就能直接進入對應狀態。

直覺的做法是：在 `App` 初始化時讀取 URL，然後寫入本地的 `WriteSignal`。
但這種做法很快就暴露出問題——**狀態乒乓（State Ping-Pong）**。
如果 URL 變了，本地信號沒跟上怎麼辦？如果用戶點了下拉選單，還要手動去同步 URL？

**解法：讓 URL 成爲「單一真值來源」（Single Source of Truth）**

某徹底拋棄了獨立的本地信號，全面改用 Derived Signal（衍生信號）。UI 不再擁有狀態的決定權，它只是一個「遙控器」：
* 下拉選單的切換，本質上是觸發了 `Maps` 改變 URL。
* URL 改變後，Leptos Router 驅動 `use_query` 和 `use_params` 更新。
* 衍生信號自動重新計算，UI 隨之刷新。

**RESTful 實踐**：
某將核心資源（方案）放在 Path 中（`/:theory`），將附加選項（題號、佈局）放在 Query 中（`?drill=...&layout=...`）。邏輯瞬間清晰。

---

## 承：領域驅動設計的覺醒

隨著 URL 接管了越來越多的狀態，一個架構陷阱出現了：**是不是所有的狀態都要放進 URL？**

打字機的「作業（drill）」有著複雜的內部流轉：打字中 -> 完成短文本 -> 退格重練 -> 進入無題號的「自習」模式。
如果讓 URL 來驅動這些，核心的「作業機關」就會被前端 Router 綁架。

**控制反轉（IoC）的高光時刻：**我守住了業務領域的邊界。

1. **依賴注入**：`輸入方案機關` 和 `佈局機關` 不再內部維護狀態，而是直接將 URL 派生出的 Signal 作爲參數「注入」進來。
2. **單向數據流 + 反向同步**：URL 下達初始指令 -> 機關引擎執行複雜的打字邏輯。\
   當用戶打完短文本，引擎內部切換爲「自習」狀態時，外層的 Effect 監聽到了引擎的變化，**反向**去清空 URL 上的 `?drill=` 參數。

這就是完美的主從同步：URL 是初始化的遙控器，Engine 處理複雜交互，Effect 負責善後同步。

---

## 轉：與編譯器近身肉搏

在實作 URL 驅動的過程中，我迎頭撞上了 Rust 強大的編譯器，經歷了一番極限拉扯。

### 1. Leptos 0.8 的破壞性更新

新版 Leptos Router 引入了強型別路由，`<Route>` 的 `path` 不能再傳入字串，必須使用 `path!()` 宏包裹；
同時 `<Routes>` 強制要求提供 `fallback`。
這些都是爲了在編譯期消滅 404 錯誤，強迫你寫出更嚴謹的路由代碼。

### 2. The Hacker Moment：優雅破解 Copy Bound

爲了封裝複雜的導航邏輯，我試圖將設定狀態的函數定義爲 `pub type 選用方案動作 = impl 動作給一參數<方案選項>;`。
然而，`use_navigate()` 返回的函數並不實作 `Copy`，這導致我的閉包在捕獲它時，觸發了嚴厲的 `E0277` 報錯。

機器人建議的解法是使用 Leptos 的 `store_value` 滿足 `Copy` 特性，或是改用 `Callback` 回傳函數閉包，但這會在調用處產生額外的噪音。
某最終的解法是：直接被多個閉包復用的 `let navigate = use_navigate()` 移入 closure 內部。
因爲 `use_navigate` 本質上是從 Context 撈取 Router，它不需要被外部捕獲，直接在執行當下讀取即可。
這完美避開了所有權與 Copy 的坑，沒有多餘的 clone。

---

## 合：決戰 Cloudflare——部署的終極玄學

本地跑得飛起，推送到 Cloudflare Pages 上線後，直接訪問 `/typewriter/combo_pinyin` 卻迎來了無情的 404。

### 1. SPA 的宿命與 200 重寫

因爲是 CSR 應用，伺服器上根本沒有 `combo_pinyin` 這個實體檔案。

解決辦法是在打包產物中加入 `_redirects` 檔案，配置 SPA 的 Fallback：

```text
/ /typewriter/ 301
/typewriter/* /typewriter/index.html 200

```

這招「狸貓換太子」，讓伺服器默默回傳 `index.html` 而不改變網址列，成功把路由解析權交還給前端的 Leptos。

### 2. Wasm 的 Magic Word 慘案

404 解決了，但頁面卻一片空白。控制台報錯：
`Uncaught CompileError: expected magic word 00 61 73 6d, found 3c 21 44 4f`

排查後發現了 Cloudflare 通配符的陷阱：
`/typewriter/*` 把對 `.wasm` 資源的請求也攔截了，導致 Wasm 引擎讀到的是 HTML 的開頭（`<!DO` 轉換爲十六進制正好是 `3c 21 44 4f`）。

**最終防線**：
不能偷懶，必須爲靜態資源開闢專屬的「綠色通道」。

最終的 `_redirects` 配置如下，這也是子目錄 SPA 部署的最穩健寫法：

```text
# 靜態資源綠色通道
/typewriter/*.js /typewriter/:splat.js 200
/typewriter/*.wasm /typewriter/:splat.wasm 200
/typewriter/*.css /typewriter/:splat.css 200

# 終極 SPA 兜底
/typewriter/* /typewriter/index.html 200
```

---

## 結語

用 Rust 和 Leptos 寫前端，是一場痛並快樂着的旅程。
初期你要和借用檢查器打架，要思考狀態的流動方向，甚至還要親手處理底層的 Web 伺服器配置。

但一旦編譯通過、成功上線，那種「狀態絕對安全」「邏輯極度解耦」的工程掌控感，是傳統 JS 框架很難給予的。代碼就在那裏，穩若泰山。

歡迎來 GitHub 交流指教：[lotem/typewriter](https://github.com/lotem/typewriter)
