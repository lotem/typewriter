use lazy_static::lazy_static;
use leptos::prelude::*;

use crate::app_state::選用方案動作;
use crate::definition::{
    碼表格式, 觸鍵方式, 輸入方案定義, 轉寫法定義, 邊界判定規則
};
use crate::gear::layout::拉丁字母鍵盤佈局;
use crate::theory::{
    alphabet::拉丁字母輸入方案, cantonese::粵語輸入方案, combo_jyutping::宮保粵拼輸入方案,
    combo_pinyin::宮保拼音輸入方案, combo_zhuyin::宮保注音輸入方案, detenele::動態能力注音輸入方案,
    early_middle_chinese::早期中古漢語輸入方案, late_middle_chinese::晚期中古漢語輸入方案,
    modern_chinese::現代漢語輸入方案, old_chinese::上古漢語輸入方案,
    old_mandarin::近古漢語輸入方案, zhuyin::注音輸入方案,
};

#[derive(Clone, Copy, Default, PartialEq)]
pub enum 方案選項 {
    #[default]
    宮保拼音,
    拉丁字母,
    上古漢語,
    早期中古漢語,
    晚期中古漢語,
    近古漢語,
    現代漢語,
    粵語,
    宮保粵拼,
    注音,
    動態能力注音,
    宮保注音,
}

lazy_static! {
    pub static ref 方案選單: Vec<(方案選項, 輸入方案定義<'static>)> = vec![
        (方案選項::宮保拼音, 宮保拼音輸入方案()),
        (方案選項::拉丁字母, 拉丁字母輸入方案()),
        (方案選項::上古漢語, 上古漢語輸入方案()),
        (方案選項::早期中古漢語, 早期中古漢語輸入方案()),
        (方案選項::晚期中古漢語, 晚期中古漢語輸入方案()),
        (方案選項::近古漢語, 近古漢語輸入方案()),
        (方案選項::現代漢語, 現代漢語輸入方案()),
        (方案選項::粵語, 粵語輸入方案()),
        (方案選項::宮保粵拼, 宮保粵拼輸入方案()),
        (方案選項::注音, 注音輸入方案()),
        (方案選項::動態能力注音, 動態能力注音輸入方案()),
        (方案選項::宮保注音, 宮保注音輸入方案()),
    ];
}

const 未定義方案: 輸入方案定義<'static> = 輸入方案定義 {
    名稱: "未定義",
    佈局: &拉丁字母鍵盤佈局,
    指法: 觸鍵方式::連擊,
    編碼法: 碼表格式::逐鍵,
    字根表: &[],
    轉寫法: 轉寫法定義 {
        輸入碼表示: &[],
        輸入碼鍵位: &[],
        拼式轉寫規則: &[],
        字根拆分規則: &[],
        拼式驗證規則: &[],
        邊界判定: 邊界判定規則 {
            分隔鍵: &[],
            起始鍵: &[],
            終止鍵: &[],
        },
    },
    動態切換: &[],
};

#[derive(Clone, Copy)]
pub struct 輸入方案機關輸出信號 {
    pub 現行方案: Signal<方案選項>,
    pub 選用方案: 選用方案動作,
    pub 方案定義: Signal<輸入方案定義<'static>>,
    pub 指法: Signal<觸鍵方式>,
}

pub fn 輸入方案機關(
    現行方案: Signal<方案選項>,
    選用方案: 選用方案動作,
) -> 輸入方案機關輸出信號 {
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
