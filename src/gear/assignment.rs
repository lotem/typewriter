use gloo_net::http::Request;
use lazy_regex::regex;
use leptos::logging::log;
use leptos::prelude::*;
use std::borrow::Cow;
use std::cmp::min;

use crate::action::*;
use crate::definition::{碼表格式, 輸入方案定義, 轉寫法定義};
use crate::drill::{練習題, 題目來源};
use crate::gear::{
    caption::字幕格式,
    theory::{方案選項, 輸入方案機關輸出信號},
};
use crate::spelling_algebra::施展拼寫運算;

#[derive(Clone, PartialEq)]
pub struct 作業 {
    pub 科目: 方案選項,
    pub 題號: Option<usize>,
    pub 自訂反查碼: Option<String>,
}

impl 作業 {
    pub fn 練習題(科目: 方案選項, 題號: usize) -> Self {
        Self {
            科目,
            題號: Some(題號),
            自訂反查碼: None,
        }
    }

    pub fn 自訂(科目: 方案選項, 反查碼: String) -> Self {
        Self {
            科目,
            題號: None,
            自訂反查碼: Some(反查碼),
        }
    }

    pub fn 自習(科目: 方案選項) -> Self {
        Self {
            科目,
            題號: None,
            自訂反查碼: None,
        }
    }

    pub fn 是否練習題(&self) -> bool {
        self.題號.is_some()
    }
}

#[derive(Clone)]
pub struct 作業內容<'a> {
    pub 碼表: 碼表定義<'a>,
    pub 字幕: 字幕格式<'a>,
}

fn 解析習題(習題文本: &str) -> 作業內容<'static> {
    match 習題文本.split_once("//") {
        Some((編碼, 字幕)) => {
            let 字幕文本 = 字幕.trim();
            作業內容 {
                碼表: 碼表定義::隨方案(Cow::Owned(編碼.trim().to_string())),
                字幕: if 字幕文本.is_empty() {
                    字幕格式::自動
                } else {
                    字幕格式::詞句(Cow::Owned(字幕文本.to_string()))
                },
            }
        }
        None => 作業內容 {
            碼表: 碼表定義::隨方案(Cow::Owned(習題文本.trim().to_string())),
            字幕: 字幕格式::自動,
        },
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct 對照輸入碼 {
    pub 字根碼原文: Option<String>,
    pub 轉寫碼原文: Option<String>,
}

impl 對照輸入碼 {
    pub fn 反查字根碼<'a>(&'a self, 轉寫法: &轉寫法定義<'a>) -> Option<String> {
        self.字根碼原文.to_owned().or_else(|| {
            self.轉寫碼原文
                .as_deref()
                .filter(|轉寫碼| 轉寫法.驗證拼式(轉寫碼))
                .and_then(|轉寫碼| 轉寫法.拼式拆分爲字根碼(轉寫碼))
                .and_then(|字根碼| 施展拼寫運算(&字根碼, 轉寫法.輸入碼表示).or(Some(字根碼)))
        })
    }

    /// 用於顯示的輸入碼. 優先顯示轉寫碼.
    pub fn 顯示輸入碼(&self) -> Option<&str> {
        self.轉寫碼原文.as_deref().or(self.字根碼原文.as_deref())
    }
}

#[derive(Clone, Copy, Default)]
pub struct 步進法 {
    pub 目標: Option<usize>,
    pub 迴轉: bool,
}

pub type 重置作業進度動作 = impl 動作;
pub type 作業推進動作 = impl 動作給一參數得一結果<步進法>;
pub type 作業回退動作 = impl 動作給一參數得一結果<步進法>;

/// 異步 `LocalResource` 的〇複製同步借閱句柄:
/// 下游僅追蹤棧上輕量的就緒 `世代` 信號, 借閱時以 `with_untracked` 原地讀取 `LocalResource` 內部鎖槽,
/// 既避免複製 `作業內容` 大文本, 又隔離下游同步 `Memo` 的 `Check` 查詢以免清掉異步資源的 `Dirty` 標記.
#[derive(Clone, Copy)]
pub struct 已載入作業 {
    世代: ReadSignal<Option<usize>>,
    資源: LocalResource<Option<作業內容<'static>>>,
}

impl 已載入作業 {
    pub fn 借閱<U>(&self, 讀取: impl FnOnce(Option<&作業內容<'static>>) -> U) -> U {
        if self.世代.get().is_none() {
            return 讀取(None);
        }
        self.資源
            .with_untracked(|槽| 讀取(槽.as_ref().and_then(Option::as_ref)))
    }
}

#[derive(Clone)]
pub struct 作業機關輸出信號 {
    pub 當前作業: ReadSignal<作業>,
    pub 佈置作業: WriteSignal<作業>,
    pub 作業進度: ReadSignal<usize>,
    pub 重置作業進度: 重置作業進度動作,
    pub 目標作業內容: LocalResource<Option<作業內容<'static>>>,
    pub 已載入作業: 已載入作業,
    pub 目標碼表格式: Signal<Option<碼表格式>>,
    pub 目標輸入碼序列: Memo<Box<[對照輸入碼]>>,
    pub 目標輸入碼片段: Signal<Option<對照輸入碼>>,
    pub 作業推進: 作業推進動作,
    pub 作業回退: 作業回退動作,
    pub 有無作業: Signal<bool>,
    pub 作業進度完成: Signal<bool>,
}

#[define_opaque(重置作業進度動作, 作業推進動作, 作業回退動作)]
pub fn 作業機關(方案: &輸入方案機關輸出信號) -> 作業機關輸出信號 {
    let 現行方案 = 方案.現行方案;
    let 方案定義 = 方案.方案定義;
    let 初始方案 = 現行方案.get_untracked();
    let (當前作業, 佈置作業) = signal(作業::練習題(初始方案, 0));
    let (作業就緒世代, 更新作業就緒世代) = signal(None::<usize>);

    let _ = Effect::watch(
        現行方案,
        move |&方案, 舊方案, _| {
            if 舊方案 != Some(&方案) {
                更新作業就緒世代(None);
                佈置作業(作業::練習題(方案, 0));
            }
        },
        false,
    );

    let (作業進度, 更新作業進度) = signal(0);

    let 重置作業進度 = move || {
        更新作業進度(0);
    };

    let 目標作業內容 = LocalResource::new(move || {
        let 作業 = 當前作業.get();
        let 選題 = 作業
            .科目
            .配套練習題()
            .and_then(|練習題| 作業.題號.and_then(|題號| 練習題.get(題號)));
        async move {
            match 選題 {
                Some(練習題 {
                    標題: _,
                    題目: 題目來源::求取 { 網址 },
                }) => {
                    let 習題文本 = Request::get(網址).send().await.ok()?.text().await.ok()?;
                    Some(解析習題(&習題文本))
                }
                Some(練習題 {
                    標題: _,
                    題目: 題目來源::內建 { 編碼, 字幕 },
                }) => Some(作業內容 {
                    碼表: 編碼.clone(),
                    字幕: 字幕.clone(),
                }),

                None => 作業.自訂反查碼.as_deref().map(解析習題),
            }
        }
    });

    // 由單一專職 Effect 銜接異步 LocalResource 與同步信號圖, 僅遞增就緒世代而不複製作業內容
    Effect::new(move |世代: Option<usize>| {
        let 已載入 = 目標作業內容.with(|槽| 槽.is_some());
        let 新世代 = 世代.unwrap_or(0).wrapping_add(1);
        更新作業就緒世代(已載入.then_some(新世代));
        新世代
    });

    let 已載入作業 = 已載入作業 {
        世代: 作業就緒世代,
        資源: 目標作業內容,
    };

    let 目標碼表格式 = Signal::derive(move || {
        已載入作業.借閱(|作業| 作業.and_then(|作業| 作業.碼表.指定格式))
    });

    let 目標輸入碼序列 = Memo::new(move |_| {
        已載入作業.借閱(|作業| {
            作業.map(|作業| 解析碼表(&作業.碼表, &方案定義.read()))
                .unwrap_or(Box::new([]))
        })
    });

    let _ = Effect::watch(
        目標輸入碼序列,
        move |輸入碼, _, _| {
            log!("更新了目標輸入碼: {}", 輸入碼.len());
            重置作業進度();
        },
        false,
    );

    let 目標輸入碼片段 = Signal::derive(move || {
        目標輸入碼序列.with(|輸入碼| {
            if 輸入碼.is_empty() {
                None
            } else {
                輸入碼.get(min(作業進度(), 輸入碼.len() - 1)).cloned()
            }
        })
    });

    let 作業推進 = move |步進: 步進法| {
        let 當前進度 = 作業進度();
        let 全文長度 = 目標輸入碼序列.read().len();
        let 目標進度 = 步進.目標.unwrap_or(當前進度 + 1);
        if 步進.迴轉 && 目標進度 >= 全文長度 {
            重置作業進度();
            Ok(())
        }
        // 非迴轉態可推進至全文結束位置
        else if 目標進度 <= 全文長度 {
            更新作業進度(目標進度);
            Ok(())
        } else {
            Err(未有())
        }
    };

    let 作業回退 = move |步進: 步進法| {
        let 當前進度 = 作業進度();
        let 全文長度 = 目標輸入碼序列.read().len();
        match 步進.目標 {
            Some(目標進度) if 步進.迴轉 || 當前進度 > 目標進度 => {
                更新作業進度(目標進度);
                Ok(())
            }
            None if 步進.迴轉 && 當前進度 == 0 && 全文長度 > 0 => {
                更新作業進度(全文長度 - 1);
                Ok(())
            }
            None if 當前進度 > 0 => {
                更新作業進度(當前進度 - 1);
                Ok(())
            }
            _ => Err(未有()),
        }
    };

    let 輸入碼總數 = move || 目標輸入碼序列.read().len();

    let 有無作業 = Signal::derive(move || 輸入碼總數() > 0);

    let 作業進度完成 = Signal::derive(move || 有無作業() && 作業進度() == 輸入碼總數());

    作業機關輸出信號 {
        當前作業,
        佈置作業,
        作業進度,
        重置作業進度,
        目標作業內容,
        已載入作業,
        目標碼表格式,
        目標輸入碼序列,
        目標輸入碼片段,
        作業推進,
        作業回退,
        有無作業,
        作業進度完成,
    }
}

/// 練習題碼表定義: 保存碼表原文與可選的格式覆寫 (`None` 表示跟隨當前方案默認 `編碼法`).
#[derive(Clone, Debug, PartialEq)]
pub struct 碼表定義<'a> {
    pub 原文: Cow<'a, str>,
    pub 指定格式: Option<碼表格式>,
}

impl<'a> 碼表定義<'a> {
    pub const fn 靜態隨方案(原文: &'a str) -> Self {
        Self {
            原文: Cow::Borrowed(原文),
            指定格式: None,
        }
    }

    pub const fn 靜態指定(格式: 碼表格式, 原文: &'a str) -> Self {
        Self {
            原文: Cow::Borrowed(原文),
            指定格式: Some(格式),
        }
    }

    pub fn 隨方案(原文: Cow<'a, str>) -> Self {
        Self {
            原文,
            指定格式: None,
        }
    }

    pub fn 實際格式(&self, 方案: &輸入方案定義) -> 碼表格式 {
        self.指定格式.unwrap_or(方案.編碼法)
    }
}

pub(crate) fn 按行解析碼表(
    碼表: &碼表定義,
    方案: &輸入方案定義,
) -> Vec<Vec<對照輸入碼>> {
    let 格式 = 碼表.實際格式(方案);
    碼表.原文
        .lines()
        .map(|行| match 格式 {
            碼表格式::逐鍵 => 解析逐鍵單行(行, 方案),
            碼表格式::連擊 => 解析連擊單行(行),
            碼表格式::並擊 => 解析並擊單行(行),
        })
        .filter(|行碼組| !行碼組.is_empty())
        .collect()
}

pub(crate) fn 解析碼表(碼表: &碼表定義, 方案: &輸入方案定義) -> Box<[對照輸入碼]> {
    按行解析碼表(碼表, 方案)
        .into_iter()
        .flatten()
        .collect()
}

/// 將單行輸入碼逐字符分段, 保留行內空白字符作獨立鍵位步進.
fn 解析逐鍵單行(單行文本: &str, 方案: &輸入方案定義) -> Vec<對照輸入碼> {
    單行文本
        .trim()
        .chars()
        .map(|字符| {
            let 輸入碼原文 = 字符.to_string();
            if 方案.尋得字根(&輸入碼原文).is_some() {
                對照輸入碼 {
                    字根碼原文: Some(輸入碼原文),
                    轉寫碼原文: None,
                }
            } else {
                對照輸入碼 {
                    字根碼原文: None,
                    轉寫碼原文: Some(輸入碼原文),
                }
            }
        })
        .collect()
}

/// 將連擊輸入碼單行按空白切分爲碼段; 方括號 `[code]` 直接作字根碼原文.
fn 解析連擊單行(單行文本: &str) -> Vec<對照輸入碼> {
    let 字根碼模式 = regex!(r"^\[(?P<code>[^\]]+)\]$");
    單行文本
        .split_whitespace()
        .map(|片段| {
            if let Some(捕獲) = 字根碼模式.captures(片段) {
                對照輸入碼 {
                    字根碼原文: Some(捕獲["code"].to_string()),
                    轉寫碼原文: None,
                }
            } else {
                對照輸入碼 {
                    字根碼原文: None,
                    轉寫碼原文: Some(片段.to_string()),
                }
            }
        })
        .collect()
}

/// 將並擊輸入碼單行解析爲輸入碼片段:
/// - 純轉寫音節 (空白或 `'` 分隔), 如 `zhong` 或尖括號 `<'a>`
/// - 大寫並擊碼或對照式, 如 `ZFURO`, `SHGUA=shu'ru'fa`, `SHGUA=<shu ru fa>`
/// - 方括號漢字並擊碼, 如 `[端定]=<泥>`
fn 解析並擊單行(單行文本: &str) -> Vec<對照輸入碼> {
    let 輸入碼片段模式 = regex!(
        r"(?x)
        (?:
            (?P<chord> \p{Uppercase}+ ) |
            \[ (?P<non_ascii_chord> [^\]]+ ) \]
        )(?:
            = (?P<eq_code> [\w']+ ) |
            =< (?P<eq_quoted_code> [^<>]* ) >
        )? |
        (?P<code> \w+ ) |
        <(?P<quoted_code> [^<>]* )>
    "
    );
    輸入碼片段模式
        .captures_iter(單行文本)
        .map(|片段| {
            let 並擊碼原文 = 片段
                .name("chord")
                .or_else(|| 片段.name("non_ascii_chord"))
                .map(|m| m.as_str().to_owned());
            let 轉寫碼原文 = 片段
                .name("code")
                .or_else(|| 片段.name("quoted_code"))
                .or_else(|| 片段.name("eq_code"))
                .or_else(|| 片段.name("eq_quoted_code"))
                .map(|m| m.as_str().to_owned());
            對照輸入碼 {
                字根碼原文: 並擊碼原文,
                轉寫碼原文,
            }
        })
        .collect()
}
