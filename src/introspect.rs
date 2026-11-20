//! 框架級信號流 [內觀] 模型: 半自動拓撲構建器, 角色自推導與非侵入式旁路探針.
//!
//! 本模塊僅負責響應式數據流建模與探針織入, **不含任何像素座標, 顏色值或 SVG 幾何邏輯**;
//! 界面自動分層排版與光束渲染全部交由 `crate::view::introspect` 獨立承擔.
//!
//! 水文五象角色:
//! - [源] :: 事件發端與狀態真值 (`ReadSignal` / `RwSignal` / 窗口事件)
//! - [流] :: 無狀態派生變換 (`Signal::derive`)
//! - [澤] :: 等值緩存與異步蓄水 (`Memo` / `LocalResource`, 值變方溢流)
//! - [閘] :: 系統數據出口, 供視圖界面消費
//! - [掣] :: 動作閉包與寫入扳機 (`Fn` / `WriteSignal` / `Callback`), 叩擊上游之 [源]

use std::collections::BTreeSet;
use std::time::Duration;

use leptos::{ev, prelude::*};
use leptos_use::{use_event_listener_with_options, use_window, UseEventListenerOptions};

use crate::definition::{碼表格式, 觸鍵方式, 輸入方案定義, 鍵組};
use crate::engine::{微觀引擎輸出信號, 視圖輸入信號};
use crate::gear::{
    assignment::{作業, 作業內容, 對照輸入碼},
    caption::{字幕段落, 字幕表示},
    chord::並擊狀態,
    input::{檔位, 觸鍵消息},
    key_press::{連擊對照表示, 連擊狀態},
    layout::{盤面選擇碼, 配列},
    mode::工作模式,
    theory::方案選項,
};
use crate::key_code::{網頁鍵值轉換, KeyCode};

/// 拓撲圖中機關節點的強類型索引紐 (`O(1)` 尋址, 免除字符串配對).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct 節點紐(pub usize);

/// 拓撲圖中信號或動作管線的強類型索引紐 (`O(1)` 尋址, 免除字符串配對).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct 管線紐(pub usize);

const 脈衝衰減毫秒: u64 = 440;
const 合詠窗口毫秒: u64 = 680;
const 合詠駐留毫秒: u64 = 920;
const 摘要最大字數: usize = 24;

// ============================================================================
// 一. 純邏輯水文角色與拓撲圖模型 (零像素座標, 零視覺色值)
// ============================================================================

/// 響應式系統中的對象角色: [源] [流] [澤] [閘] [掣].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 響應角色 {
    源,
    流,
    澤,
    閘,
    掣,
}

impl 響應角色 {
    pub const 全部角色: [Self; 5] = [Self::源, Self::流, Self::澤, Self::閘, Self::掣];

    pub const fn 名稱(self) -> &'static str {
        match self {
            Self::源 => "源",
            Self::流 => "流",
            Self::澤 => "澤",
            Self::閘 => "閘",
            Self::掣 => "掣",
        }
    }

    pub const fn 符號(self) -> &'static str {
        match self {
            Self::源 => "◎",
            Self::流 => "〰",
            Self::澤 => "◈",
            Self::閘 => "⫯",
            Self::掣 => "⚡",
        }
    }

    pub const fn 釋義(self) -> &'static str {
        match self {
            Self::源 => "事件發端 · 狀態真值 (ReadSignal / URL / Event)",
            Self::流 => "派生信號 · 即時變換 (Signal::derive)",
            Self::澤 => "蓄水緩存 · 值變方溢 (Memo / LocalResource)",
            Self::閘 => "數據出口 · 視圖消費 (View Props / Effect)",
            Self::掣 => "動作閉包 · 叩擊水源 (Fn / WriteSignal / Callback)",
        }
    }
}

/// 節點演進態勢: 區分實裝模塊與 [七境] 演進藍圖預留位.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum 節點態勢 {
    實裝 {
        模塊: &'static str,
        主角色: 響應角色,
    },
    待鑄 {
        設想職責: &'static str,
    },
}

impl 節點態勢 {
    pub const fn 是否實裝(self) -> bool {
        matches!(self, Self::實裝 { .. })
    }
}

/// 純邏輯機關節點描述 (不含任何畫布像素座標).
#[derive(Clone, Copy, Debug)]
pub struct 機關節點 {
    pub 紐: 節點紐,
    pub 標題: &'static str,
    pub 態勢: 節點態勢,
}

/// 九脈語義分組 (視覺配色由 `view::introspect` 決定).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 九脈 {
    觸鍵消息脈,
    方案法度脈,
    盤面經緯脈,
    並擊和絃脈,
    連擊字根脈,
    習題碼表脈,
    反查指路脈,
    批閱步進脈,
    走紙回顯脈,
    待鑄藍圖脈,
}

impl 九脈 {
    pub const 實裝九脈: [Self; 9] = [
        Self::觸鍵消息脈,
        Self::方案法度脈,
        Self::盤面經緯脈,
        Self::並擊和絃脈,
        Self::連擊字根脈,
        Self::習題碼表脈,
        Self::反查指路脈,
        Self::批閱步進脈,
        Self::走紙回顯脈,
    ];

    pub const fn 名稱(self) -> &'static str {
        match self {
            Self::觸鍵消息脈 => "一 · 觸鍵消息脈",
            Self::方案法度脈 => "二 · 方案法度脈",
            Self::盤面經緯脈 => "三 · 盤面經緯脈",
            Self::並擊和絃脈 => "四 · 並擊和絃脈",
            Self::連擊字根脈 => "五 · 連擊字根脈",
            Self::習題碼表脈 => "六 · 習題碼表脈",
            Self::反查指路脈 => "七 · 反查指路脈",
            Self::批閱步進脈 => "八 · 批閱步進脈",
            Self::走紙回顯脈 => "九 · 走紙回顯脈",
            Self::待鑄藍圖脈 => "預 · 待鑄藍圖脈",
        }
    }
}

/// 自適應 [合詠] 閉環的四個實裝階段.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum 合詠階段 {
    無,
    一擊觸發,
    指法成碼,
    作業推進,
    走紙回顯,
}

impl 合詠階段 {
    pub const fn 階段位序(self) -> Option<usize> {
        match self {
            Self::無 => None,
            Self::一擊觸發 => Some(0),
            Self::指法成碼 => Some(1),
            Self::作業推進 => Some(2),
            Self::走紙回顯 => Some(3),
        }
    }
}

/// 由脈絡歸屬, 信號角色與源節點態勢自動推導默認 `[合詠階段]`, 免除逐條手工標註.
const fn 推導合詠階段(
    分組: 九脈, 角色: 響應角色, 源態勢: 節點態勢
) -> 合詠階段 {
    match 分組 {
        九脈::觸鍵消息脈 => 合詠階段::一擊觸發,
        九脈::並擊和絃脈 | 九脈::連擊字根脈 => match 角色 {
            響應角色::流 | 響應角色::澤 => 合詠階段::指法成碼,
            _ => 合詠階段::無,
        },
        九脈::批閱步進脈 => 合詠階段::作業推進,
        九脈::走紙回顯脈 => match 源態勢 {
            節點態勢::實裝 {
                主角色: 響應角色::流,
                ..
            } => 合詠階段::走紙回顯,
            _ => 合詠階段::無,
        },
        _ => 合詠階段::無,
    }
}

/// 純邏輯有向管線描述 (不含任何曲率像素魔數).
#[derive(Clone, Copy, Debug)]
pub struct 信號管線 {
    pub 紐: 管線紐,
    pub 信號名: &'static str,
    pub 源節點: 節點紐,
    pub 目標節點: 節點紐,
    pub 角色: 響應角色,
    pub 分組: 九脈,
    pub 階段: 合詠階段,
}

// ============================================================================
// 二. 領域載荷濃縮摘要特徵 `內觀摘要`
// ============================================================================

/// 按字符數安全截斷 UTF-8 字符串.
pub fn 截斷字符(文本: &str, 上限: usize) -> String {
    let mut 字符流 = 文本.chars();
    let 前段: String = 字符流.by_ref().take(上限).collect();
    if 字符流.next().is_some() {
        format!("{前段}...")
    } else {
        前段
    }
}

pub trait 內觀摘要 {
    fn 濃縮摘要(&self) -> String;
}

macro_rules! 摘要同Display {
    ($($類型:ty),+ $(,)?) => {
        $(
            impl 內觀摘要 for $類型 {
                fn 濃縮摘要(&self) -> String {
                    self.to_string()
                }
            }
        )+
    };
}

摘要同Display!(方案選項, 配列);

impl 內觀摘要 for bool {
    fn 濃縮摘要(&self) -> String {
        if *self { "✓ 真" } else { "· 假" }.into()
    }
}

impl 內觀摘要 for usize {
    fn 濃縮摘要(&self) -> String {
        format!("#{self}")
    }
}

impl 內觀摘要 for str {
    fn 濃縮摘要(&self) -> String {
        if self.is_empty() {
            "∅".into()
        } else {
            截斷字符(self, 摘要最大字數)
        }
    }
}

impl 內觀摘要 for String {
    fn 濃縮摘要(&self) -> String {
        self.as_str().濃縮摘要()
    }
}

impl<T: 內觀摘要> 內觀摘要 for Option<T> {
    fn 濃縮摘要(&self) -> String {
        self.as_ref().map_or_else(|| "∅".into(), T::濃縮摘要)
    }
}

impl<T: ?Sized + 內觀摘要> 內觀摘要 for Box<T> {
    fn 濃縮摘要(&self) -> String {
        self.as_ref().濃縮摘要()
    }
}

impl 內觀摘要 for KeyCode {
    fn 濃縮摘要(&self) -> String {
        format!("{self:?}")
    }
}

impl 內觀摘要 for 觸鍵消息 {
    fn 濃縮摘要(&self) -> String {
        let mut 修飾 = String::new();
        for (按下, 名) in [
            (self.檔位.ctrl, "Ctrl+"),
            (self.檔位.alt, "Alt+"),
            (self.檔位.shift, "Shift+"),
            (self.檔位.meta, "Meta+"),
        ] {
            if 按下 {
                修飾.push_str(名);
            }
        }
        截斷字符(&format!("{修飾}{:?}", self.鍵碼), 摘要最大字數)
    }
}

impl 內觀摘要 for 輸入方案定義<'_> {
    fn 濃縮摘要(&self) -> String {
        截斷字符(
            &format!("{} · {}", self.名稱, self.指法.濃縮摘要()),
            摘要最大字數,
        )
    }
}

impl 內觀摘要 for 觸鍵方式 {
    fn 濃縮摘要(&self) -> String {
        match self {
            Self::連擊 => "連擊",
            Self::並擊 => "並擊",
        }
        .into()
    }
}

impl 內觀摘要 for 盤面選擇碼 {
    fn 濃縮摘要(&self) -> String {
        format!("層#{} ({:#x})", self.頂層盤面(), self.0)
    }
}

impl 內觀摘要 for 作業 {
    fn 濃縮摘要(&self) -> String {
        match (self.題號, self.自訂反查碼.as_deref()) {
            (Some(題號), _) => format!("{} · 題#{題號}", self.科目),
            (None, Some(反查碼)) => 截斷字符(&format!("反查: {反查碼}"), 摘要最大字數),
            (None, None) => format!("{} · 自習", self.科目),
        }
    }
}

impl 內觀摘要 for 碼表格式 {
    fn 濃縮摘要(&self) -> String {
        match self {
            Self::逐鍵 => "逐鍵碼表",
            Self::連擊 => "連擊碼表",
            Self::並擊 => "並擊碼表",
        }
        .into()
    }
}

impl 內觀摘要 for 作業內容<'_> {
    fn 濃縮摘要(&self) -> String {
        let 格式名 = self
            .碼表
            .指定格式
            .map_or_else(|| "隨方案碼表".into(), |格式| 格式.濃縮摘要());
        format!("{格式名} ({}B)", self.碼表.原文.len())
    }
}

impl 內觀摘要 for 對照輸入碼 {
    fn 濃縮摘要(&self) -> String {
        self.顯示輸入碼()
            .map_or_else(|| "∅".into(), |文本| 截斷字符(文本, 摘要最大字數))
    }
}

impl 內觀摘要 for [對照輸入碼] {
    fn 濃縮摘要(&self) -> String {
        format!("共 {} 碼段", self.len())
    }
}

impl 內觀摘要 for 字幕段落<'_> {
    fn 濃縮摘要(&self) -> String {
        let 字幕段落(起, 止, ref 內容) = *self;
        let 文本 = 內容.拼接全文();
        format!("[{起}..{止}) {}", 截斷字符(文本.trim(), 8))
    }
}

impl 內觀摘要 for [字幕段落<'_>] {
    fn 濃縮摘要(&self) -> String {
        format!("共 {} 段", self.len())
    }
}

impl 內觀摘要 for 字幕表示 {
    fn 濃縮摘要(&self) -> String {
        if self.指標文字.is_empty() {
            "[完]".into()
        } else {
            截斷字符(&format!("[{}]", self.指標文字), 摘要最大字數)
        }
    }
}

impl 內觀摘要 for 鍵組 {
    fn 濃縮摘要(&self) -> String {
        if self.0.is_empty() {
            return "∅".into();
        }
        let 衆鍵: Vec<String> = self.0.iter().take(5).map(|鍵| format!("{鍵:?}")).collect();
        let 尾綴 = if self.0.len() > 5 { "+..." } else { "" };
        截斷字符(&format!("[{}{尾綴}]", 衆鍵.join(",")), 摘要最大字數)
    }
}

impl 內觀摘要 for 並擊狀態 {
    fn 濃縮摘要(&self) -> String {
        if self.實時落鍵.0.is_empty() && self.累計擊鍵.0.is_empty() {
            "靜默 ∅".into()
        } else {
            截斷字符(
                &format!(
                    "落:{} 累:{}",
                    self.實時落鍵.0.len(),
                    self.累計擊鍵.濃縮摘要()
                ),
                摘要最大字數,
            )
        }
    }
}

impl 內觀摘要 for 連擊狀態 {
    fn 濃縮摘要(&self) -> String {
        if self.鍵碼 == KeyCode::No {
            "∅".into()
        } else {
            format!("{:?}×{}", self.鍵碼, self.連擊次數)
        }
    }
}

impl 內觀摘要 for 連擊對照表示 {
    fn 濃縮摘要(&self) -> String {
        let 餘數 = self.待輸後綴.chars().count();
        if !self.錯碼段.is_empty() {
            截斷字符(
                &format!("{} ✗{} · 餘{餘數}", self.已對前綴, self.錯碼段),
                摘要最大字數,
            )
        } else if !self.顯示光標 && !self.已對前綴.is_empty() && 餘數 == 0 {
            截斷字符(&format!("✓ {}", self.已對前綴), 摘要最大字數)
        } else if self.已對前綴.is_empty() {
            if 餘數 > 0 {
                format!("待輸 {餘數} 碼")
            } else {
                "∅".into()
            }
        } else if 餘數 > 0 {
            截斷字符(&format!("{}‸ · 餘{餘數}", self.已對前綴), 摘要最大字數)
        } else if self.顯示光標 {
            截斷字符(&format!("{}‸", self.已對前綴), 摘要最大字數)
        } else {
            截斷字符(&self.已對前綴, 摘要最大字數)
        }
    }
}

impl 內觀摘要 for 工作模式 {
    fn 濃縮摘要(&self) -> String {
        match self {
            Self::錄入 => "錄入態",
            Self::輸入反查碼 => "輸入反查碼",
            Self::選取練習題 => "選取練習題",
            Self::選擇輸入方案 => "選擇方案",
            Self::選擇配列 => "選擇配列",
        }
        .into()
    }
}

// ============================================================================
// 三. 信號角色自推導特徵 `可內觀信號` 與旁路探針掛載
// ============================================================================

/// 框架級可觀測信號特徵: 根據 Leptos 信號具體類型自動推導固有水文角色 ([源] / [流] / [澤]) 並掛載旁路探針.
pub trait 可內觀信號: Copy + 'static {
    const 固有角色: 響應角色;
    fn 掛載探針(self, 總樞: 內觀總樞, 目標紐: 管線紐);
}

fn 掛載同步信號探針<S, T>(總樞: 內觀總樞, 目標紐: 管線紐, 信號: S)
where
    S: With<Value = T> + WithUntracked<Value = T> + Track + Copy + 'static,
    T: 內觀摘要 + ?Sized + 'static,
{
    let 初始值 = 信號.try_with_untracked(T::濃縮摘要);
    總樞.標記已接線(目標紐, 初始值);

    Effect::new(move |已運行: Option<()>| {
        if 已運行.is_none() || !總樞.是否開啓_不追蹤() {
            // 首幀或 [內觀視圖] 關閉時僅維持依賴追蹤, 零字符串分配, 零定時器開銷
            信號.track();
            return;
        }
        if let Some(摘要) = 信號.try_with(T::濃縮摘要) {
            總樞.發射脈衝(目標紐, 摘要);
        }
    });

    Effect::new(move |_| {
        if 總樞.開啓.get() {
            if let Some(摘要) = 信號.try_with_untracked(T::濃縮摘要) {
                總樞.靜默刷新快照(目標紐, 摘要);
            }
        }
    });
}

impl<T: 內觀摘要 + Send + Sync + 'static> 可內觀信號 for ReadSignal<T> {
    const 固有角色: 響應角色 = 響應角色::源;
    fn 掛載探針(self, 總樞: 內觀總樞, 目標紐: 管線紐) {
        掛載同步信號探針(總樞, 目標紐, self);
    }
}

impl<T: 內觀摘要 + Send + Sync + 'static> 可內觀信號 for RwSignal<T> {
    const 固有角色: 響應角色 = 響應角色::源;
    fn 掛載探針(self, 總樞: 內觀總樞, 目標紐: 管線紐) {
        掛載同步信號探針(總樞, 目標紐, self);
    }
}

impl<T: 內觀摘要 + Send + Sync + 'static> 可內觀信號 for Signal<T> {
    const 固有角色: 響應角色 = 響應角色::流;
    fn 掛載探針(self, 總樞: 內觀總樞, 目標紐: 管線紐) {
        掛載同步信號探針(總樞, 目標紐, self);
    }
}

impl<T: 內觀摘要 + Send + Sync + 'static> 可內觀信號 for Memo<T> {
    const 固有角色: 響應角色 = 響應角色::澤;
    fn 掛載探針(self, 總樞: 內觀總樞, 目標紐: 管線紐) {
        掛載同步信號探針(總樞, 目標紐, self);
    }
}

impl<T: 內觀摘要 + 'static> 可內觀信號 for LocalResource<Option<T>> {
    const 固有角色: 響應角色 = 響應角色::澤;
    fn 掛載探針(self, 總樞: 內觀總樞, 目標紐: 管線紐) {
        總樞.標記已接線(目標紐, Some("待載入...".into()));
        // 專職單源 Effect: 僅追蹤 LocalResource 自身, 絕不混訂下游 Memo
        Effect::new(move |已運行: Option<()>| {
            let 守衛 = self.read();
            if 已運行.is_none() || !總樞.是否開啓_不追蹤() {
                return;
            }
            let 摘要 = match 守衛.as_ref() {
                None => "汲水中...".into(),
                Some(Some(值)) => 值.濃縮摘要(),
                Some(None) => "∅".into(),
            };
            總樞.發射脈衝(目標紐, 摘要);
        });
    }
}

// ============================================================================
// 四. 運行時狀態機與半自動拓撲構建器 `內觀總樞`
// ============================================================================

#[derive(Clone, Debug, Default)]
pub struct 管線脈衝態 {
    pub 已接線: bool,
    pub 活躍: bool,
    pub 世代: u64,
    pub 摘要: String,
}

#[derive(Clone, Debug, Default)]
pub struct 節點脈衝態 {
    pub 已接管線數: usize,
    pub 活躍: bool,
    pub 世代: u64,
    pub 快照: String,
}

struct 管線上下文 {
    管線: 信號管線,
    管線態: RwSignal<管線脈衝態>,
    源態: RwSignal<節點脈衝態>,
    匯態: RwSignal<節點脈衝態>,
}

#[derive(Clone, Debug)]
pub struct 脈衝記錄 {
    pub 信號名: &'static str,
    pub 角色: 響應角色,
    pub 摘要: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum 內觀附欄 {
    同屏對照,
    待鑄藍圖,
    純淨光束,
}

#[derive(Default)]
struct 拓撲名冊 {
    衆節點: Vec<機關節點>,
    衆管線: Vec<信號管線>,
    節點態表: Vec<RwSignal<節點脈衝態>>,
    管線態表: Vec<RwSignal<管線脈衝態>>,
}

/// 全局響應式內觀總樞 (`Copy` 句柄): 集拓撲登記, 探針引線與脈衝狀態機於一體.
#[derive(Clone, Copy)]
pub struct 內觀總樞 {
    pub 開啓: RwSignal<bool>,
    pub 錄屏模式: RwSignal<bool>,
    pub 附欄: RwSignal<內觀附欄>,
    pub 聚光九脈: RwSignal<Option<九脈>>,
    pub 聚光角色: RwSignal<Option<響應角色>>,
    名冊: StoredValue<拓撲名冊>,
    pub 近期脈衝: RwSignal<Vec<脈衝記錄>>,
    pub 累計脈衝數: RwSignal<u64>,
    合詠掩碼: RwSignal<u8>,
    各階世代: RwSignal<[u64; 4]>,
    pub 合詠活躍: RwSignal<bool>,
    pub 合詠世代: RwSignal<u64>,
}

impl 內觀總樞 {
    pub fn new() -> Self {
        Self {
            開啓: RwSignal::new(false),
            錄屏模式: RwSignal::new(false),
            附欄: RwSignal::new(內觀附欄::同屏對照),
            聚光九脈: RwSignal::new(None),
            聚光角色: RwSignal::new(None),
            名冊: StoredValue::new(拓撲名冊::default()),
            近期脈衝: RwSignal::new(Vec::with_capacity(8)),
            累計脈衝數: RwSignal::new(0),
            合詠掩碼: RwSignal::new(0),
            各階世代: RwSignal::new([0; 4]),
            合詠活躍: RwSignal::new(false),
            合詠世代: RwSignal::new(0),
        }
    }

    /// 登記一個已實裝 `[機關]` 節點, 返回其 `O(1)` 索引紐.
    pub fn 登記機關(
        &self,
        標題: &'static str,
        模塊: &'static str,
        主角色: 響應角色,
    ) -> 節點紐 {
        self.推入節點(標題, 節點態勢::實裝 { 模塊, 主角色 })
    }

    /// 登記一個尚未實裝的 `[七境]` 演進藍圖節點.
    pub fn 登記待鑄(&self, 標題: &'static str, 設想職責: &'static str) -> 節點紐 {
        self.推入節點(標題, 節點態勢::待鑄 { 設想職責 })
    }

    fn 推入節點(&self, 標題: &'static str, 態勢: 節點態勢) -> 節點紐 {
        let mut 編號 = 0;
        self.名冊.update_value(|名冊| {
            編號 = 名冊.衆節點.len();
            let 紐 = 節點紐(編號);
            名冊.衆節點.push(機關節點 {
                紐, 標題, 態勢
            });
            名冊.節點態表.push(RwSignal::new(節點脈衝態::default()));
        });
        節點紐(編號)
    }

    fn 推入管線(
        &self,
        源節點: 節點紐,
        目標節點: 節點紐,
        角色: 響應角色,
        分組: 九脈,
        信號名: &'static str,
        階段覆寫: Option<合詠階段>,
    ) -> 管線紐 {
        let mut 編號 = 0;
        self.名冊.update_value(|名冊| {
            編號 = 名冊.衆管線.len();
            let 紐 = 管線紐(編號);
            let 源態勢 = 名冊.衆節點[源節點.0].態勢;
            let 階段 = 階段覆寫.unwrap_or_else(|| 推導合詠階段(分組, 角色, 源態勢));
            名冊.衆管線.push(信號管線 {
                紐,
                信號名,
                源節點,
                目標節點,
                角色,
                分組,
                階段,
            });
            名冊.管線態表.push(RwSignal::new(管線脈衝態::default()));
        });
        管線紐(編號)
    }

    /// 半自動穿針引線: 根據信號具體類型自動推導 `[源 / 流 / 澤]` 角色, 註冊拓撲邊並掛載旁路探針.
    pub fn 引線<S: 可內觀信號>(
        &self,
        源節點: 節點紐,
        目標節點: 節點紐,
        分組: 九脈,
        信號名: &'static str,
        信號: S,
    ) -> 管線紐 {
        let 紐 = self.推入管線(源節點, 目標節點, S::固有角色, 分組, 信號名, None);
        信號.掛載探針(*self, 紐);
        紐
    }

    /// 將出口信號引入視圖 `[閘]` 節點: 標記端點角色爲 `[閘]` 並掛載旁路探針.
    pub fn 引閘<S: 可內觀信號>(
        &self,
        源節點: 節點紐,
        目標閘: 節點紐,
        分組: 九脈,
        信號名: &'static str,
        信號: S,
    ) -> 管線紐 {
        let 紐 = self.推入管線(源節點, 目標閘, 響應角色::閘, 分組, 信號名, None);
        信號.掛載探針(*self, 紐);
        紐
    }

    /// 登記一條 `[掣]` 控制/動作管線並標記已接線, 返回 `管線紐` 供動作或邊沿觀測器發射脈衝.
    pub fn 引掣(
        &self,
        源節點: 節點紐,
        目標節點: 節點紐,
        分組: 九脈,
        信號名: &'static str,
        階段覆寫: Option<合詠階段>,
    ) -> 管線紐 {
        let 紐 = self.推入管線(源節點, 目標節點, 響應角色::掣, 分組, 信號名, 階段覆寫);
        self.標記已接線(紐, None);
        紐
    }

    /// 登記一條尚未接線的 `[待鑄藍圖脈]` 演進預留邊.
    pub fn 預留藍圖線(
        &self,
        源節點: 節點紐,
        目標節點: 節點紐,
        角色: 響應角色,
        信號名: &'static str,
    ) -> 管線紐 {
        self.推入管線(
            源節點,
            目標節點,
            角色,
            九脈::待鑄藍圖脈,
            信號名,
            Some(合詠階段::無),
        )
    }

    /// 零拷貝借閱當前已登記的完整邏輯拓撲 (`衆節點`, `衆管線`).
    pub fn 借閱拓撲<U>(&self, 讀取: impl FnOnce(&[機關節點], &[信號管線]) -> U) -> U {
        self.名冊
            .with_value(|名冊| 讀取(&名冊.衆節點, &名冊.衆管線))
    }

    /// `O(1)` 讀取管線脈衝信號.
    pub fn 管線信號(&self, 目標紐: 管線紐) -> Option<RwSignal<管線脈衝態>> {
        self.名冊
            .with_value(|名冊| 名冊.管線態表.get(目標紐.0).copied())
    }

    /// `O(1)` 讀取節點脈衝信號.
    pub fn 節點信號(&self, 目標紐: 節點紐) -> Option<RwSignal<節點脈衝態>> {
        self.名冊
            .with_value(|名冊| 名冊.節點態表.get(目標紐.0).copied())
    }

    pub fn 是否顯示藍圖(&self) -> bool {
        self.附欄.get() == 內觀附欄::待鑄藍圖
    }

    pub fn 是否顯示藍圖_不追蹤(&self) -> bool {
        self.附欄.get_untracked() == 內觀附欄::待鑄藍圖
    }

    pub fn 是否顯示對照鍵盤(&self) -> bool {
        self.附欄.get() == 內觀附欄::同屏對照
    }

    pub fn 切換附欄(&self, 目標: 內觀附欄) {
        self.附欄.update(|當前| {
            *當前 = if *當前 == 目標 {
                內觀附欄::純淨光束
            } else {
                目標
            };
        });
    }

    pub fn 是否開啓_不追蹤(&self) -> bool {
        self.開啓.get_untracked()
    }

    pub fn 切換視圖(&self) {
        self.開啓.update(|開| *開 = !*開);
    }

    pub fn 切換錄屏(&self) {
        let 下態 = !self.錄屏模式.get_untracked();
        self.錄屏模式.set(下態);
        if 下態 && !self.開啓.get_untracked() {
            self.開啓.set(true);
        }
    }

    fn 查閱管線上下文(&self, 目標紐: 管線紐) -> Option<管線上下文> {
        self.名冊.with_value(|名冊| {
            let 管線 = *名冊.衆管線.get(目標紐.0)?;
            let 管線態 = *名冊.管線態表.get(目標紐.0)?;
            let 源態 = *名冊.節點態表.get(管線.源節點.0)?;
            let 匯態 = *名冊.節點態表.get(管線.目標節點.0)?;
            Some(管線上下文 {
                管線,
                管線態,
                源態,
                匯態,
            })
        })
    }

    pub fn 標記已接線(&self, 目標紐: 管線紐, 初始摘要: Option<String>) {
        let Some(管線上下文 {
            管線,
            管線態,
            源態,
            匯態,
        }) = self.查閱管線上下文(目標紐)
        else {
            return;
        };

        管線態.update(|狀態| {
            狀態.已接線 = true;
            if let Some(ref 文本) = 初始摘要 {
                狀態.摘要.clone_from(文本);
            }
        });
        for (端點紐, 節點態) in [(管線.源節點, 源態), (管線.目標節點, 匯態)] {
            節點態.update(|狀態| {
                狀態.已接管線數 += 1;
                if 端點紐 == 管線.源節點 && 狀態.快照.is_empty() {
                    if let Some(文本) = 初始摘要.as_deref().filter(|文本| *文本 != "∅")
                    {
                        狀態.快照.push_str(文本);
                    }
                }
            });
        }
    }

    pub fn 靜默刷新快照(&self, 目標紐: 管線紐, 摘要: String) {
        let Some(管線上下文 {
            管線態, 源態, ..
        }) = self.查閱管線上下文(目標紐)
        else {
            return;
        };
        管線態.update(|狀態| 狀態.摘要.clone_from(&摘要));
        if 摘要 != "∅" && !摘要.is_empty() {
            源態.update(|狀態| {
                if !狀態.活躍 {
                    狀態.快照 = 摘要;
                }
            });
        }
    }

    pub fn 發射脈衝(&self, 目標紐: 管線紐, 摘要: String) {
        if !self.是否開啓_不追蹤() {
            return;
        }
        let Some(管線上下文 {
            管線,
            管線態,
            源態,
            匯態,
        }) = self.查閱管線上下文(目標紐)
        else {
            return;
        };

        self.累計脈衝數.update(|計數| *計數 += 1);

        let mut 本次世代 = 0;
        管線態.update(|狀態| {
            狀態.已接線 = true;
            狀態.活躍 = true;
            狀態.世代 += 1;
            本次世代 = 狀態.世代;
            狀態.摘要.clone_from(&摘要);
        });
        set_timeout(
            move || {
                管線態.try_update(|狀態| {
                    if 狀態.世代 == 本次世代 {
                        狀態.活躍 = false;
                    }
                });
            },
            Duration::from_millis(脈衝衰減毫秒),
        );

        for 節點態 in [源態, 匯態] {
            let mut 節點世代 = 0;
            let 快照副本 = 摘要.clone();
            節點態.update(|狀態| {
                狀態.活躍 = true;
                狀態.世代 += 1;
                節點世代 = 狀態.世代;
                狀態.快照 = 快照副本;
            });
            set_timeout(
                move || {
                    節點態.try_update(|狀態| {
                        if 狀態.世代 == 節點世代 {
                            狀態.活躍 = false;
                        }
                    });
                },
                Duration::from_millis(脈衝衰減毫秒 + 60),
            );
        }

        self.近期脈衝.update(|列表| {
            列表.insert(
                0,
                脈衝記錄 {
                    信號名: 管線.信號名,
                    角色: 管線.角色,
                    摘要,
                },
            );
            列表.truncate(8);
        });

        if let Some(階位) = 管線.階段.階段位序() {
            self.推進合詠階段(階位);
        }
    }

    fn 推進合詠階段(&self, 階位: usize) {
        let 位掩碼 = 1u8 << 階位;
        let mut 本階世代 = 0u64;
        self.各階世代.update(|世代組| {
            世代組[階位] += 1;
            本階世代 = 世代組[階位];
        });

        let mut 完成閉環 = false;
        self.合詠掩碼.update(|掩碼| {
            *掩碼 |= 位掩碼;
            if (*掩碼 & 0b1111) == 0b1111 {
                完成閉環 = true;
                *掩碼 = 0;
            }
        });

        if 完成閉環 {
            let mut 共振序號 = 0;
            self.合詠世代.update(|世代| {
                *世代 += 1;
                共振序號 = *世代;
            });
            self.合詠活躍.set(true);
            let 合詠活躍 = self.合詠活躍;
            let 合詠世代 = self.合詠世代;
            set_timeout(
                move || {
                    if 合詠世代.try_get_untracked() == Some(共振序號) {
                        let _ = 合詠活躍.try_set(false);
                    }
                },
                Duration::from_millis(合詠駐留毫秒),
            );
        } else {
            let 合詠掩碼 = self.合詠掩碼;
            let 各階世代 = self.各階世代;
            set_timeout(
                move || {
                    if let Some(世代組) = 各階世代.try_get_untracked() {
                        if 世代組[階位] == 本階世代 {
                            合詠掩碼.try_update(|掩碼| *掩碼 &= !位掩碼);
                        }
                    }
                },
                Duration::from_millis(合詠窗口毫秒),
            );
        }
    }

    /// 包裝 Leptos `Callback<P>`, 在觸發時向指定管線發射脈衝.
    pub fn 包裝回調<P>(
        &self,
        目標紐: 管線紐,
        前綴: &'static str,
        回調: Callback<P>,
    ) -> Callback<P>
    where
        P: 內觀摘要 + Send + Sync + 'static,
    {
        let 總樞 = *self;
        Callback::new(move |參數: P| {
            if 總樞.是否開啓_不追蹤() {
                let 摘要 = 截斷字符(&format!("{前綴} {}", 參數.濃縮摘要()), 摘要最大字數);
                總樞.發射脈衝(目標紐, 摘要);
            }
            回調.run(參數);
        })
    }
}

// ============================================================================
// 五. 應用層非侵入式裝配器: 在 `engine.rs` 外部爲打字機引擎織入內觀拓撲與視圖熱鍵
// ============================================================================

pub fn 裝配打字機內觀(
    引擎: &微觀引擎輸出信號
) -> (內觀總樞, 視圖輸入信號) {
    let 內觀 = 內觀總樞::new();
    provide_context(內觀);

    // 1. 登記九大實裝機關與三枚待鑄演進節點 (純邏輯身份, 無任何座標參數)
    let 方案樞 = 內觀.登記機關(
        "方案樞 · 輸入方案",
        "gear::theory + app_state",
        響應角色::源,
    );
    let 觸鍵源 = 內觀.登記機關("觸鍵源 · 輸入事件", "gear::input", 響應角色::源);
    let 模式闕 = 內觀.登記機關("模式闕 · 工作模式", "gear::mode + engine", 響應角色::源);
    let 盤面陣 = 內觀.登記機關("盤面陣 · 鍵盤佈局", "gear::layout", 響應角色::流);
    let 作業卷 = 內觀.登記機關("作業卷 · 習題進度", "gear::assignment", 響應角色::澤);
    let 並擊池 = 內觀.登記機關("並擊池 · 並擊機關", "gear::chord", 響應角色::澤);
    let 連擊機 = 內觀.登記機關("連擊機 · 連擊機關", "gear::key_press", 響應角色::澤);
    let 字幕軸 = 內觀.登記機關("字幕軸 · 字幕機關", "gear::caption", 響應角色::流);
    let 走紙屏 = 內觀.登記機關("走紙屏 · 打字機視圖", "app + view::*", 響應角色::閘);

    let 韻書探幽 = 內觀.登記待鑄("韻書探幽", "[韻書] 與音韻碼表檢索之 [澤]");
    let 構詞圖 = 內觀.登記待鑄("DAG 構詞", "有向無環圖切分構詞之 [流/澤]");
    let 機關排序 = 內觀.登記待鑄("機關排序", "候選詞排序輸出之 [流]");

    let 微觀引擎輸出信號 {
        方案,
        模式,
        佈局,
        作業,
        字幕,
        連擊,
        並擊,
        視圖,
    } = 引擎;

    // 2. 半自動引線: 一行同時完成拓撲邊註冊, 角色自推導與響應式探針掛載
    // -- 一 · 觸鍵消息脈 --
    let 落鍵入並擊 = 內觀.引掣(
        觸鍵源,
        並擊池,
        九脈::觸鍵消息脈,
        "既然落鍵 → 並擊狀態變更.落鍵",
        None,
    );
    let 抬鍵入並擊 = 內觀.引掣(
        觸鍵源,
        並擊池,
        九脈::觸鍵消息脈,
        "既然抬鍵 → 並擊狀態變更.抬鍵",
        None,
    );
    let 擊鍵入連擊 = 內觀.引掣(
        觸鍵源,
        連擊機,
        九脈::觸鍵消息脈,
        "編輯連擊輸入碼(KeyCode)",
        None,
    );
    let 功能鍵攔截 = 內觀.引掣(
        觸鍵源,
        模式闕,
        九脈::觸鍵消息脈,
        "處理功能鍵(觸鍵消息)",
        Some(合詠階段::無),
    );
    let 觸屏回傳源 = 內觀.引掣(
        走紙屏,
        觸鍵源,
        九脈::觸鍵消息脈,
        "視圖落鍵動作 / 視圖抬鍵動作",
        None,
    );

    // -- 二 · 方案法度脈 --
    內觀.引線(方案樞, 作業卷, 九脈::方案法度脈, "現行方案", 方案.現行方案);
    內觀.引線(
        方案樞,
        盤面陣,
        九脈::方案法度脈,
        "方案定義 (佈局/默認盤面)",
        方案.方案定義,
    );
    內觀.引線(
        方案樞,
        並擊池,
        九脈::方案法度脈,
        "方案定義 (字根表/轉寫法)",
        方案.方案定義,
    );
    內觀.引線(
        方案樞,
        連擊機,
        九脈::方案法度脈,
        "方案定義 (字根表/動態切換)",
        方案.方案定義,
    );
    內觀.引線(
        方案樞,
        字幕軸,
        九脈::方案法度脈,
        "方案定義 (編碼法)",
        方案.方案定義,
    );

    // -- 三 · 盤面經緯脈 --
    內觀.引閘(盤面陣, 走紙屏, 九脈::盤面經緯脈, "實際配列", 佈局.實際配列);
    內觀.引線(盤面陣, 連擊機, 九脈::盤面經緯脈, "當選盤面", 佈局.當選盤面);
    內觀.引閘(盤面陣, 走紙屏, 九脈::盤面經緯脈, "當選盤面", 佈局.當選盤面);
    let 自動切盤脈 = 內觀.引掣(
        連擊機,
        盤面陣,
        九脈::盤面經緯脈,
        "自動切換盤面 → 選擇盤面",
        None,
    );

    // -- 四 · 並擊和絃脈 --
    內觀.引線(
        並擊池,
        走紙屏,
        九脈::並擊和絃脈,
        "並擊狀態流",
        並擊.並擊狀態流,
    );
    內觀.引線(
        並擊池,
        走紙屏,
        九脈::並擊和絃脈,
        "實況並擊碼",
        並擊.實況並擊碼,
    );
    內觀.引線(
        並擊池,
        走紙屏,
        九脈::並擊和絃脈,
        "並擊所得拼音",
        並擊.並擊所得拼音,
    );

    // -- 五 · 連擊字根脈 --
    內觀.引線(
        連擊機,
        走紙屏,
        九脈::連擊字根脈,
        "實況字根碼",
        連擊.實況字根碼,
    );
    內觀.引線(連擊機, 走紙屏, 九脈::連擊字根脈, "連擊對照", 連擊.連擊對照);

    // -- 六 · 習題碼表脈 --
    內觀.引線(
        作業卷,
        字幕軸,
        九脈::習題碼表脈,
        "目標作業內容 (LocalResource)",
        作業.目標作業內容,
    );
    內觀.引線(
        作業卷,
        字幕軸,
        九脈::習題碼表脈,
        "目標輸入碼序列",
        作業.目標輸入碼序列,
    );
    內觀.引線(
        作業卷,
        並擊池,
        九脈::習題碼表脈,
        "目標輸入碼片段",
        作業.目標輸入碼片段,
    );
    內觀.引線(
        作業卷,
        連擊機,
        九脈::習題碼表脈,
        "目標輸入碼片段",
        作業.目標輸入碼片段,
    );

    // -- 七 · 反查指路脈 --
    內觀.引閘(並擊池, 走紙屏, 九脈::反查指路脈, "反查鍵位", 並擊.反查鍵位);
    內觀.引閘(連擊機, 走紙屏, 九脈::反查指路脈, "逐鍵提示", 連擊.逐鍵提示);

    // -- 八 · 批閱步進脈 --
    let 並擊批閱推進 = 內觀.引掣(
        並擊池,
        作業卷,
        九脈::批閱步進脈,
        "並擊成功 → 作業推進",
        None,
    );
    let 連擊批閱推進 = 內觀.引掣(
        連擊機,
        作業卷,
        九脈::批閱步進脈,
        "連擊比對成功 → 作業推進",
        None,
    );
    內觀.引線(作業卷, 字幕軸, 九脈::批閱步進脈, "作業進度", 作業.作業進度);

    // -- 九 · 走紙回顯脈 --
    內觀.引閘(字幕軸, 走紙屏, 九脈::走紙回顯脈, "段落表示", 字幕.段落表示);
    內觀.引閘(
        模式闕,
        走紙屏,
        九脈::走紙回顯脈,
        "現行工作模式",
        模式.現行工作模式,
    );
    內觀.引線(
        模式闕,
        作業卷,
        九脈::走紙回顯脈,
        "當前作業 / 佈置作業",
        作業.當前作業,
    );

    // -- 預 · 待鑄藍圖脈 --
    內觀.預留藍圖線(並擊池, 韻書探幽, 響應角色::流, "待鑄: 音碼檢索");
    內觀.預留藍圖線(連擊機, 韻書探幽, 響應角色::流, "待鑄: 字根檢索");
    內觀.預留藍圖線(韻書探幽, 構詞圖, 響應角色::澤, "待鑄: 詞元切分");
    內觀.預留藍圖線(構詞圖, 機關排序, 響應角色::流, "待鑄: 候選排序");
    內觀.預留藍圖線(機關排序, 走紙屏, 響應角色::閘, "待鑄: 候選詞閘");

    // 3. 外部旁路邊沿觀測器 (無須侵入 engine.rs 內部閉包即可還原全部 [掣] 動作脈衝)
    let 並擊狀態流 = 並擊.並擊狀態流;
    Effect::new(move |舊落鍵: Option<BTreeSet<KeyCode>>| {
        let 新落鍵 = 並擊狀態流.with(|狀態| 狀態.實時落鍵.0.clone());
        if let Some(舊) = 舊落鍵 {
            if 內觀.是否開啓_不追蹤() {
                for 鍵碼 in 新落鍵.difference(&舊) {
                    內觀.發射脈衝(落鍵入並擊, format!("↓ {鍵碼:?}"));
                }
                for 鍵碼 in 舊.difference(&新落鍵) {
                    內觀.發射脈衝(抬鍵入並擊, format!("↑ {鍵碼:?}"));
                }
            }
        }
        新落鍵
    });

    let 連擊狀態流 = 連擊.連擊狀態流;
    Effect::new(move |已初始化: Option<連擊狀態>| {
        let 當前 = 連擊狀態流.get();
        if 已初始化.is_some() && 當前.鍵碼 != KeyCode::No && 內觀.是否開啓_不追蹤()
        {
            內觀.發射脈衝(擊鍵入連擊, format!("擊 {:?}", 當前.鍵碼));
        }
        當前
    });

    let 作業進度 = 作業.作業進度;
    let 目標輸入碼序列 = 作業.目標輸入碼序列;
    let 分段字幕 = 字幕.分段字幕;
    let 指法 = 方案.指法;
    let 現行工作模式 = 模式.現行工作模式;
    Effect::new(move |舊進度: Option<usize>| {
        let 新進度 = 作業進度.get();
        if let Some(舊) = 舊進度 {
            if 內觀.是否開啓_不追蹤() && 現行工作模式.get_untracked() == 工作模式::錄入
            {
                let 全長 = 目標輸入碼序列.with_untracked(|序列| 序列.len());
                let 多段迴轉 = 新進度 == 0
                    && 舊 > 0
                    && 舊 + 1 >= 全長
                    && 分段字幕.with_untracked(|衆段| 衆段.len() > 1);
                if 新進度 == 舊 + 1 || 多段迴轉 {
                    let 快照 = format!("✓ 進 #{新進度}");
                    match 指法.get_untracked() {
                        觸鍵方式::並擊 => 內觀.發射脈衝(並擊批閱推進, 快照),
                        觸鍵方式::連擊 => 內觀.發射脈衝(連擊批閱推進, 快照),
                    }
                }
            }
        }
        新進度
    });

    let 當選盤面 = 佈局.當選盤面;
    let 現行方案 = 方案.現行方案;
    Effect::new(move |舊態: Option<(盤面選擇碼, 方案選項)>| {
        let 新盤面 = 當選盤面.get();
        let 新方案 = 現行方案.get();
        if let Some((舊盤面, 舊方案)) = 舊態 {
            if 新盤面 != 舊盤面
                && 新方案 == 舊方案
                && 指法.get_untracked() == 觸鍵方式::連擊
                && 內觀.是否開啓_不追蹤()
            {
                內觀.發射脈衝(自動切盤脈, 新盤面.濃縮摘要());
            }
        }
        (新盤面, 新方案)
    });

    // 4. 捕獲階段鍵盤監聽: 接管視圖層熱鍵 (`Ctrl+\`, `Ctrl+|`, `Alt+X`, 錄屏態 `Esc`), 並旁路觀測功能鍵脈衝
    let _ = use_event_listener_with_options(
        use_window(),
        ev::keydown,
        move |ev| {
            if ev.is_composing() {
                return;
            }
            let 鍵碼 = 網頁鍵值轉換(&ev.code());
            let 檔位 = 檔位 {
                shift: ev.shift_key(),
                ctrl: ev.ctrl_key(),
                alt: ev.alt_key(),
                meta: ev.meta_key(),
            };
            let 消息 = 觸鍵消息 { 鍵碼, 檔位 };

            let 觸發內觀開關 =
                (消息.檔位.ctrl && !消息.檔位.alt && !消息.檔位.meta && 鍵碼 == KeyCode::Backslash)
                    || (消息.檔位.alt && !消息.檔位.ctrl && !消息.檔位.meta && 鍵碼 == KeyCode::X);
            if 觸發內觀開關 {
                if 消息.檔位.shift {
                    內觀.切換錄屏();
                } else {
                    if 內觀.錄屏模式.get() {
                        內觀.錄屏模式.set(false);
                    }
                    內觀.切換視圖();
                }
                內觀.發射脈衝(功能鍵攔截, 消息.濃縮摘要());
                ev.prevent_default();
                ev.stop_immediate_propagation();
                return;
            }

            if 鍵碼 == KeyCode::Escape
                && 內觀.錄屏模式.get()
                && 現行工作模式() == 工作模式::錄入
                && 作業進度() == 0
            {
                內觀.錄屏模式.set(false);
                內觀.發射脈衝(功能鍵攔截, 消息.濃縮摘要());
                ev.prevent_default();
                ev.stop_immediate_propagation();
                return;
            }

            if 內觀.是否開啓_不追蹤() {
                let 是功能鍵 =
                    (消息.檔位.ctrl && !消息.檔位.alt && !消息.檔位.meta && 鍵碼 == KeyCode::Grave)
                        || matches!(鍵碼, KeyCode::Escape | KeyCode::Tab | KeyCode::Enter)
                        || (鍵碼 == KeyCode::Backspace && 現行工作模式() == 工作模式::錄入);
                if 是功能鍵 {
                    內觀.發射脈衝(功能鍵攔截, 消息.濃縮摘要());
                }
            }
        },
        UseEventListenerOptions::default().capture(true),
    );

    let 裝配視圖 = 視圖輸入信號 {
        視圖落鍵動作: 內觀.包裝回調(觸屏回傳源, "觸↓", 視圖.視圖落鍵動作),
        視圖抬鍵動作: 內觀.包裝回調(觸屏回傳源, "觸↑", 視圖.視圖抬鍵動作),
    };

    (內觀, 裝配視圖)
}
