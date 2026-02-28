# Rime 打字機 (Typewriter)

[Rime 打字機](https://github.com/lotem/typewriter) 是一個基於 Rust 和 Leptos 框架開發的現代 CSR (客戶端渲染) 網頁應用。
它內置了輕量級的開發版 Rime 輸入法微觀引擎，專為打字練習、方案驗證與鍵盤配列展示而設計。

👉 **[在線體驗 (Live Demo) 點擊這裏](https://rime.io/typewriter)**

## ✨ 核心特性

* **內置 Rime 微觀引擎**：在瀏覽器端完全用 Rust 實現的輸入狀態機，支持單字、詞句的編碼解析與狀態流轉。
* **豐富的觸鍵方式**：完美支持**並擊 (Chord)** 和**連擊**，精確還原實體鍵盤的輸入反饋。
* **多種鍵盤佈局支持**：支持實時切換虛擬鍵盤形態，並適配屏幕等比縮放。包含：
* 傳統主鍵盤區 (`qwerty`)
* 緊湊的字母鍵盤 (`alphabet`)
* 正交直列 (`ortho`)
* 直列分體 (`ortho_split`)
* 縱向錯列分體（`columnar_split`）
* 各種帶數字行的進階佈局

* **多維度練習模式**：
* **預設練習題**：依據不同方案定製的循序漸進練習（如：聲母韻母練習、單字、詞句、洋文金句等）。
* **自訂文本**：支持粘貼任意文本生成即時反查碼與字幕，進行專項練習。
* **自習模式**：自由擊鍵，實時回顯輸入碼與虛擬鍵盤按鍵狀態。

* **現代 Web 體驗**：
* 基於 Leptos 0.8，極致的響應式性能。
* 完整的 URL 狀態同步：可透過 URL 參數（如 `/typewriter/combo_pinyin/?drill=1&layout=ortho`）直接分享特定的方案、題號與佈局。
* 深色/淺色主題自動適配。

## ⌨️ 支持的輸入方案

打字機目前內置並支持以下輸入方案的演示與練習：

**現代漢語與方言：**

* **宮保拼音** (默認) - 高效的拼音並擊方案 [🏠 專案主頁](http://github.com/rime/rime-combo-pinyin) | [📺 基礎教程視頻](https://www.bilibili.com/video/BV1oRMbz2ET6/)
* **宮保粵拼** - 粵語並擊方案 [🏠 專案主頁](https://github.com/lotem/rime-combo-jyutping) | [📺 鍵盤演示](https://www.bilibili.com/video/BV1UfY3z7EUf/)
* **粵語** - 根據《[分韻撮要](https://ytenx.org/pyonh/)》記載的粵語聲紐和韻部設計並擊鍵盤 [📺 方案出鞘視頻](https://www.bilibili.com/video/BV1R6aczdETX/)
* **宮保注音** - 以注音字母標註的宮保並擊鍵盤
* **注音** - 傳統大千式注音
* **動態能力注音** - 支持零聲母鍵〇與動態盤面切換 [📺 東風破·注音緣](https://www.bilibili.com/video/BV13HZKB1EJt/)

**歷史語音：**

欲知古人如何打字？

以下歷史語音方案的並擊錄入演示，可觀看視頻：[📺 詩經「隰有萇楚」多音系並擊演示](https://www.bilibili.com/video/BV12attzGEht/)

* **上古漢語**
* **早期中古漢語**
* **晚期中古漢語**
* **近古漢語**
* **現代漢語**

**基礎方案：**

* **拉丁字母** - 用於英打與基礎鍵盤測試

## 🛠️ 本地開發指南

本專案使用 Rust 編寫，並透過 `trunk` 構建為 WebAssembly (Wasm) 應用。

### 環境準備

1. 安裝 Rust 工具鏈 (建議使用 nightly)：
```sh
rustup toolchain install nightly
rustup default nightly

```

2. 添加 Wasm 目標平台：
```sh
rustup target add wasm32-unknown-unknown

```

3. 安裝打包工具 Trunk：
```sh
cargo install trunk

```

### 運行與構建

啟動本地開發服務器 (帶熱重載功能)：

```sh
trunk serve --open

```

構建生產環境優化版本：

```sh
trunk build --release

```

## 📖 架構與開發紀實

想了解這個純前端打字機是如何構建出來的嗎？
某記錄了使用 Rust + Leptos 進行開發的極客之旅。
內容涵蓋前端狀態管理架構、依賴注入 (IoC)、編譯器借用檢查的極限拉扯，以及 Cloudflare SPA 部署玄學。

👉 **[閱讀開發紀實：從狀態混亂到架構覺醒](./docs/architecture_journey.md)**
👉 **[閱讀開發紀實：Leptos 異步資源加載](./docs/leptos_async_resource_loading.md)**

## 📜 授權協議

本專案源代碼遵循開源協議。詳見代碼倉庫相關說明。
