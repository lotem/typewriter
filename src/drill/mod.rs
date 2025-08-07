mod alphabet;
mod ancient_chinese;
mod combo_pinyin;

use alphabet::字母鍵盤練習題;
use ancient_chinese::{
    上古漢語練習題, 早期中古漢語練習題, 晚期中古漢語練習題, 現代漢語練習題, 近古漢語練習題,
};
use combo_pinyin::宮保拼音練習題;

use crate::gear::{caption::字幕格式, theory::方案選項};

pub struct 練習題<'a> {
    pub 標題: &'a str,
    pub 編碼: &'a str,
    pub 字幕: 字幕格式<'a>,
}

const 各方案練習題組: &[(方案選項, &[練習題])] = &[
    (方案選項::拉丁字母, 字母鍵盤練習題),
    (方案選項::宮保拼音, 宮保拼音練習題),
    (方案選項::上古漢語, 上古漢語練習題),
    (方案選項::早期中古漢語, 早期中古漢語練習題),
    (方案選項::晚期中古漢語, 晚期中古漢語練習題),
    (方案選項::近古漢語, 近古漢語練習題),
    (方案選項::現代漢語, 現代漢語練習題),
];

impl 方案選項 {
    pub fn 配套練習題(&self) -> Option<&'static [練習題<'static>]> {
        各方案練習題組.iter().find_map(|&(方案, 練習題)| {
            if 方案 == *self {
                Some(練習題)
            } else {
                None
            }
        })
    }
}
