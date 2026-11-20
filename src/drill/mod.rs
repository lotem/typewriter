mod alphabet;
mod ancient_chinese;
mod cangjie;
mod cantonese;
mod combo_jyutping;
mod combo_pinyin;
mod combo_zhuyin;
mod zhuyin;

use alphabet::字母鍵盤練習題;
use ancient_chinese::{
    上古漢語練習題, 早期中古漢語練習題, 晚期中古漢語練習題, 現代漢語練習題, 近古漢語練習題,
};
use cangjie::倉頡練習題;
use cantonese::粵語練習題;
use combo_jyutping::宮保粵拼練習題;
use combo_pinyin::宮保拼音練習題;
use combo_zhuyin::宮保注音練習題;
use zhuyin::注音練習題;

use crate::definition::碼表格式;
use crate::gear::{assignment::碼表定義, caption::字幕格式, theory::方案選項};

#[derive(Clone)]
pub struct 練習題<'a> {
    pub 標題: &'a str,
    pub 題目: 題目來源<'a>,
}

#[derive(Clone)]
pub enum 題目來源<'a> {
    內建 {
        編碼: 碼表定義<'a>,
        字幕: 字幕格式<'a>,
    },
    求取 {
        網址: &'a str,
    },
}

impl<'a> 練習題<'a> {
    /// 編碼隨當前方案默認格式解析, 配顯式詞句字幕 (空白切段, `[文字組]` 合字).
    pub const fn 詞句(標題: &'a str, 編碼: &'a str, 字幕: &'a str) -> Self {
        Self {
            標題,
            題目: 題目來源::內建 {
                編碼: 碼表定義::靜態隨方案(編碼),
                字幕: 字幕格式::靜態詞句(字幕),
            },
        }
    }

    /// 原文直打: 編碼隨當前方案默認格式解析, 並按各非空行自動生成對應字幕段落 (免重複抄寫字幕).
    pub const fn 直打(標題: &'a str, 編碼: &'a str) -> Self {
        Self {
            標題,
            題目: 題目來源::內建 {
                編碼: 碼表定義::靜態隨方案(編碼),
                字幕: 字幕格式::自動,
            },
        }
    }

    /// 單鍵字根基本功練習: 強制按 `碼表格式::逐鍵` 逐字符切分, 配顯式詞句字幕.
    pub const fn 逐鍵(標題: &'a str, 編碼: &'a str, 字幕: &'a str) -> Self {
        Self {
            標題,
            題目: 題目來源::內建 {
                編碼: 碼表定義::靜態指定(碼表格式::逐鍵, 編碼),
                字幕: 字幕格式::靜態詞句(字幕),
            },
        }
    }

    /// 異步求取外部題庫文件 (`//` 前爲編碼, 後爲詞句字幕; 省略 `//` 則按行自動生成字幕).
    pub const fn 求取(標題: &'a str, 網址: &'a str) -> Self {
        Self {
            標題,
            題目: 題目來源::求取 { 網址 },
        }
    }
}

const 各方案練習題組: &[(方案選項, &[練習題])] = &[
    (方案選項::宮保拼音, 宮保拼音練習題),
    (方案選項::拉丁字母, 字母鍵盤練習題),
    (方案選項::倉頡, 倉頡練習題),
    (方案選項::倉頡七曜並擊, 倉頡練習題),
    (方案選項::注音, 注音練習題),
    (方案選項::動態能力注音, 注音練習題),
    (方案選項::宮保注音, 宮保注音練習題),
    (方案選項::宮保粵拼, 宮保粵拼練習題),
    (方案選項::粵語, 粵語練習題),
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
