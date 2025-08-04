use lazy_static::lazy_static;
use leptos::prelude::*;

use crate::definition::{觸鍵方式, 輸入方案定義, 轉寫法定義};
use crate::gear::layout::拉丁字母鍵盤佈局;
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
    佈局: &拉丁字母鍵盤佈局,
    指法: 觸鍵方式::連擊,
    字根表: &[],
    轉寫法: 轉寫法定義 {
        輸入碼格式: &[],
        正字法規則: &[],
        字根拆分規則: &[],
        拼式驗證規則: &[],
    },
};

#[derive(Clone, Copy)]
pub struct 輸入方案機關輸出信號 {
    pub 現行方案: ReadSignal<方案選項>,
    pub 選用方案: WriteSignal<方案選項>,
    pub 方案定義: Signal<輸入方案定義<'static>>,
    pub 指法: Signal<觸鍵方式>,
}

pub fn 輸入方案機關() -> 輸入方案機關輸出信號 {
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

    let 指法 = Signal::derive(move || 方案定義.read().指法);

    輸入方案機關輸出信號 {
        現行方案,
        選用方案,
        方案定義,
        指法,
    }
}
