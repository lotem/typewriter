use lazy_static::lazy_static;
use leptos::prelude::*;

use crate::definition::{觸鍵方式, 輸入方案定義, 轉寫法定義};
use crate::layout::盤面選擇碼;
use crate::theory::{alphabet::拉丁字母輸入方案, combo_pinyin::宮保拼音輸入方案};

#[derive(Clone, Copy, Default, PartialEq)]
pub enum 方案選項 {
    #[default]
    宮保拼音,
    拉丁字母,
}

lazy_static! {
    pub static ref 方案選單: Vec<(方案選項, 輸入方案定義<'static>)> = vec![
        (方案選項::宮保拼音, 宮保拼音輸入方案()),
        (方案選項::拉丁字母, 拉丁字母輸入方案()),
    ];
}

const 未定義方案: 輸入方案定義<'static> = 輸入方案定義 {
    名稱: "未定義",
    盤面: 盤面選擇碼(0),
    指法: 觸鍵方式::連擊,
    字根表: &[],
    轉寫法: 轉寫法定義 {
        拼式轉寫規則: &[],
        字根拆分規則: &[],
        拼式驗證規則: &[],
    },
};

#[allow(clippy::type_complexity)]
pub fn 輸入方案機關() -> (
    // 現行方案
    ReadSignal<方案選項>,
    // 選用方案
    WriteSignal<方案選項>,
    // 方案定義
    Signal<輸入方案定義<'static>>,
) {
    let (現行方案, 選用方案) = signal(方案選項::default());

    let 方案定義 = Signal::derive(move || {
        方案選單
            .iter()
            .find_map(|&(方案, 定義)| {
                if 方案 == 現行方案() {
                    Some(定義)
                } else {
                    None
                }
            })
            .unwrap_or(未定義方案)
    });

    (現行方案, 選用方案, 方案定義)
}
