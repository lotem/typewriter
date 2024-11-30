use lazy_static::lazy_static;
use leptos::*;

use crate::alphabet::拉丁字母輸入方案;
use crate::combo_pinyin::宮保拼音輸入方案;
use crate::engine::輸入方案定義;

#[derive(Clone, Copy, PartialEq)]
pub enum 方案選項 {
    拉丁字母,
    宮保拼音,
}

lazy_static! {
    pub static ref 方案選單: Vec<(方案選項, 輸入方案定義<'static>)> = vec![
        (方案選項::拉丁字母, 拉丁字母輸入方案()),
        (方案選項::宮保拼音, 宮保拼音輸入方案()),
    ];
}

#[allow(clippy::type_complexity)]
pub fn 輸入方案機關() -> (
    ReadSignal<方案選項>,
    WriteSignal<方案選項>,
    Signal<輸入方案定義<'static>>,
) {
    let (現行方案, 選用方案) = create_signal(方案選項::宮保拼音);

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
            .unwrap()
    });

    (現行方案, 選用方案, 方案定義)
}
