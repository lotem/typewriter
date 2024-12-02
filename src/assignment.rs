use lazy_regex::regex;
use leptos::*;
use std::cmp::min;

use crate::action::*;
use crate::engine::{對照輸入碼, 觸鍵方式, 輸入方案定義};
use crate::theory::方案選項;

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

    pub fn 反查碼(&self) -> &str {
        self.科目
            .配套練習題()
            .and_then(|練習題| self.題號.and_then(|題號| 練習題.get(題號)))
            .map(|題| 題.編碼)
            .or(self.自訂反查碼.as_deref())
            .unwrap_or("")
    }

    pub fn 字幕(&self) -> Option<&'static str> {
        self.科目
            .配套練習題()
            .and_then(|練習題| self.題號.and_then(|題號| 練習題.get(題號)))
            .and_then(|題| 題.字幕)
    }

    pub fn 是否練習題(&self) -> bool {
        self.題號.is_some()
    }
}

pub struct 作業推進參數 {
    pub 段落: Option<(usize, usize)>,
    pub 迴轉: bool,
}

impl 作業推進參數 {
    pub fn 步進() -> Self {
        Self {
            段落: None,
            迴轉: false,
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn 作業機關(
    現行方案: ReadSignal<方案選項>,
    方案定義: Signal<輸入方案定義<'static>>,
) -> (
    // 當前作業
    ReadSignal<作業>,
    // 佈置作業
    WriteSignal<作業>,
    // 作業進度
    ReadSignal<usize>,
    // 作業進度完成
    Signal<bool>,
    // 目標輸入碼
    Signal<Option<對照輸入碼>>,
    // 重置作業進度
    impl 動作,
    // 作業推進
    impl 動作給一參數得一結果<作業推進參數>,
    // 作業回退
    impl 動作得一結果,
) {
    let 初始方案 = 現行方案.get_untracked();
    let (當前作業, 佈置作業) = create_signal(作業::練習題(初始方案, 0));

    let _ = watch(
        現行方案,
        move |&方案, _, _| {
            佈置作業(作業::練習題(方案, 0));
        },
        false,
    );

    let (作業進度, 更新作業進度) = create_signal(0);

    let 反查拼音組 = create_memo(move |_| {
        with!(|當前作業, 方案定義| 解析輸入碼序列(當前作業.反查碼(), 方案定義))
    });

    let 重置作業進度 = move || 更新作業進度(0);

    let _ = watch(
        反查拼音組,
        move |_, _, _| {
            重置作業進度();
        },
        false,
    );

    let 作業推進 = move |參數: 作業推進參數| {
        let 全文結束 = 反查拼音組.with(Vec::len);
        let 推進目標位置 = match 參數.段落 {
            Some((起, 止)) => {
                if 作業進度() < 起 {
                    起
                } else {
                    止
                }
            }
            None => 作業進度() + 1,
        };
        if 參數.迴轉 && 推進目標位置 >= 全文結束 {
            重置作業進度();
            Ok(())
        }
        // 非迴轉態可推進至全文結束位置
        else if 推進目標位置 <= 全文結束 {
            更新作業進度(推進目標位置);
            Ok(())
        } else {
            Err(未有())
        }
    };

    let 作業回退 = move || {
        if 作業進度() > 0 {
            更新作業進度(作業進度() - 1);
            Ok(())
        } else {
            Err(未有())
        }
    };
    let 拼音數 = move || 反查拼音組.with(Vec::len);
    let 作業進度完成 = Signal::derive(move || 作業進度() == 拼音數());

    let 目標輸入碼 = Signal::derive(move || {
        反查拼音組.with(|拼音組| {
            if 拼音組.is_empty() {
                None
            } else {
                拼音組.get(min(作業進度(), 拼音組.len() - 1)).cloned()
            }
        })
    });

    (
        當前作業,
        佈置作業,
        作業進度,
        作業進度完成,
        目標輸入碼,
        重置作業進度,
        作業推進,
        作業回退,
    )
}

fn 解析輸入碼序列(
    輸入碼序列: &str, 方案: &輸入方案定義
) -> Vec<對照輸入碼> {
    match 方案.指法 {
        觸鍵方式::連擊 => 解析連擊輸入碼序列(輸入碼序列, 方案),
        觸鍵方式::並擊 => 解析並擊輸入碼序列(輸入碼序列),
    }
}

fn 解析連擊輸入碼序列(
    輸入碼序列: &str, 方案: &輸入方案定義
) -> Vec<對照輸入碼> {
    輸入碼序列
        .split_whitespace()
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
/// 輸入碼通常是拼音音節的序列, 音節之間用空白或隔音符號 `'` 分開.
/// 特殊形式的拼音寫在尖括號中, 如: `<'a>`。
///
/// 輸入碼片段也可以是以下形式:
///
/// - 用大寫字母連書並擊碼, 如 `ZFURO`
/// - 寫明並擊碼和對應的拼音, 如 `SHGUA=shu'ru'fa`
/// - 寫明並擊碼並將對應的拼音寫在尖括號中, 如 `SHGUA=<shu ru fa>`
fn 解析並擊輸入碼序列(輸入碼序列: &str) -> Vec<對照輸入碼> {
    let 輸入碼片段模式 = regex!(
        r"(?x)
        (?P<chord> \p{Uppercase}+ )
        (?:
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
            let 並擊碼原文 = 片段.name("chord").map(|m| m.as_str().to_owned());
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
