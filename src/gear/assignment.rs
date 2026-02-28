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
        Some((編碼, 字幕)) => 作業內容 {
            碼表: 碼表定義::自訂(Cow::Owned(編碼.trim().to_string())),
            字幕: 字幕格式::自訂(Cow::Owned(字幕.trim().to_string())),
        },
        None => 作業內容 {
            碼表: 碼表定義::自訂(Cow::Owned(習題文本.trim().to_string())),
            字幕: 字幕格式::自動生成,
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

#[derive(Clone)]
pub struct 作業機關輸出信號 {
    pub 當前作業: ReadSignal<作業>,
    pub 佈置作業: WriteSignal<作業>,
    pub 作業進度: ReadSignal<usize>,
    pub 重置作業進度: 重置作業進度動作,
    pub 目標作業內容: LocalResource<Option<作業內容<'static>>>,
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

    let _ = Effect::watch(
        現行方案,
        move |&方案, _, _| {
            佈置作業(作業::練習題(方案, 0));
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

    let 目標碼表格式 = Signal::derive(move || {
        目標作業內容
            .read()
            .as_ref()
            .flatten()
            .and_then(|作業| 作業.碼表.碼表格式())
    });

    let 目標輸入碼序列 = Memo::new(move |_| {
        目標作業內容
            .read()
            .as_ref()
            .flatten()
            .map(|作業| 解析碼表(&作業.碼表, &方案定義.read()))
            .unwrap_or(Box::new([]))
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
        目標碼表格式,
        目標輸入碼序列,
        目標輸入碼片段,
        作業推進,
        作業回退,
        有無作業,
        作業進度完成,
    }
}

#[derive(Clone)]
pub enum 碼表定義<'a> {
    逐鍵(&'a str),
    連擊(&'a str),
    並擊(&'a str),
    自訂(Cow<'a, str>),
}

impl 碼表定義<'_> {
    pub fn 碼表原文(&self) -> &str {
        match &self {
            Self::逐鍵(s) => s,
            Self::連擊(s) => s,
            Self::並擊(s) => s,
            Self::自訂(s) => s,
        }
    }

    pub fn 碼表格式(&self) -> Option<碼表格式> {
        match &self {
            Self::逐鍵(_) => Some(碼表格式::逐鍵),
            Self::連擊(_) => Some(碼表格式::連擊),
            Self::並擊(_) => Some(碼表格式::並擊),
            Self::自訂(_) => None,
        }
    }
}

fn 解析碼表(碼表: &碼表定義, 方案: &輸入方案定義) -> Box<[對照輸入碼]> {
    match 碼表 {
        碼表定義::逐鍵(輸入碼序列) => 解析逐鍵輸入碼序列(輸入碼序列, 方案),
        碼表定義::連擊(輸入碼序列) => 解析連擊輸入碼序列(輸入碼序列),
        碼表定義::並擊(輸入碼序列) => 解析並擊輸入碼序列(輸入碼序列),
        碼表定義::自訂(輸入碼序列) => match 方案.編碼法 {
            碼表格式::逐鍵 => 解析逐鍵輸入碼序列(輸入碼序列, 方案),
            碼表格式::連擊 => 解析連擊輸入碼序列(輸入碼序列),
            碼表格式::並擊 => 解析並擊輸入碼序列(輸入碼序列),
        },
    }
}

/// 將輸入碼逐鍵/逐字分段, 包括行內的空白文字.
fn 解析逐鍵輸入碼序列(
    輸入碼序列: &str, 方案: &輸入方案定義
) -> Box<[對照輸入碼]> {
    輸入碼序列
        .lines()
        .map(str::trim)
        .flat_map(|片段| 片段.chars())
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

/// 將並擊輸入碼序列解析爲輸入碼片段.
///
/// 輸入碼通常是音節序列, 音節之間用空白分開.
/// 音節用習慣的轉寫形式書寫, 如拼音 `zhong`, 注音 `ㄓㄨㄥ`;
/// 若用字根碼拼寫, 須寫在方括號中，如四通碼 `[ay]`.
fn 解析連擊輸入碼序列(輸入碼序列: &str) -> Box<[對照輸入碼]> {
    let 字根碼模式 = regex!(r"^\[(?P<code>[^\]]+)\]$");
    輸入碼序列
        .split_whitespace()
        .map(|片段| {
            let 輸入碼原文 = 片段.to_string();
            if 字根碼模式.is_match(&輸入碼原文) {
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

/// 將並擊輸入碼序列解析爲輸入碼片段.
///
/// 輸入碼通常是拼音音節的序列, 音節之間用空白或隔音符號 `'` 分開.
/// 特殊形式的拼音寫在尖括號中, 如: `<'a>`。
///
/// 輸入碼片段也可以是以下形式:
///
/// - 用大寫字母連書並擊碼, 如 `ZFURO`
/// - 寫明並擊碼和對應的拼音, 如 `SHGUA=shu'ru'fa`
/// - 寫明並擊碼並將對應的拼音寫在尖括號中, 如 `SHGUA=<shu ru fa>`
/// - 非大寫字母的並擊碼，寫在方括號中，如 `[端定]=<泥>`
fn 解析並擊輸入碼序列(輸入碼序列: &str) -> Box<[對照輸入碼]> {
    let 輸入碼片段模式 = regex!(
        r"(?x)
        (?:
            (?P<chord> \p{Uppercase}+ ) |
            \[ (?P<non_ascii_chord> [^\]]+ ) \]
        )(?:
            = (?P<eq_code> [\w'] )+ |
            =< (?P<eq_quoted_code> [^<>]* ) >
        )? |
        (?P<code> \w+ ) |
        <(?P<quoted_code> [^<>]* )>
    "
    );
    輸入碼片段模式
        .captures_iter(輸入碼序列)
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
