use leptos::*;
use std::cmp::min;

use crate::action::*;
use crate::engine::{對照輸入碼, 解析輸入碼序列};
use crate::theory::方案選單;

#[derive(Clone, PartialEq)]
pub struct 作業 {
    pub 科目: 方案選單,
    pub 題號: Option<usize>,
    pub 自訂反查碼: Option<String>,
}

impl 作業 {
    pub fn 練習題(科目: 方案選單, 題號: usize) -> Self {
        Self {
            科目,
            題號: Some(題號),
            自訂反查碼: None,
        }
    }

    pub fn 自訂(科目: 方案選單, 反查碼: String) -> Self {
        Self {
            科目,
            題號: None,
            自訂反查碼: Some(反查碼),
        }
    }

    pub fn 自習(科目: 方案選單) -> Self {
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

#[allow(clippy::type_complexity)]
pub fn 作業機關(
    初始方案: 方案選單,
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
    impl 動作給一參數得一結果<bool>,
    // 作業回退
    impl 動作得一結果,
) {
    let (當前作業, 佈置作業) = create_signal(作業::練習題(初始方案, 0));
    let (作業進度, 更新作業進度) = create_signal(0);

    let 反查拼音組 =
        create_memo(move |_| 當前作業.with(|作業| 解析輸入碼序列(作業.反查碼())));

    let 重置作業進度 = move || 更新作業進度(0);

    let _ = watch(
        反查拼音組,
        move |_, _, _| {
            重置作業進度();
        },
        false,
    );

    let 作業推進 = move |迴轉: bool| {
        let 拼音數 = 反查拼音組.with(Vec::len);
        if 迴轉 && 作業進度() + 1 >= 拼音數 {
            重置作業進度();
            Ok(())
        }
        // 非迴轉態可推進至結束位置，即拼音數
        else if 作業進度() < 拼音數 {
            更新作業進度(作業進度() + 1);
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
