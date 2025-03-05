use leptos::prelude::*;
use std::borrow::Cow;

use crate::definition::觸鍵方式;
use crate::gear::{
    assignment::{作業機關輸出信號, 對照輸入碼},
    theory::輸入方案機關輸出信號,
};

#[derive(Clone, Copy, PartialEq)]
pub enum 字幕步進 {
    逐字,
    逐詞,
}

impl From<觸鍵方式> for 字幕步進 {
    fn from(source: 觸鍵方式) -> Self {
        match source {
            觸鍵方式::連擊 => 字幕步進::逐字,
            觸鍵方式::並擊 => 字幕步進::逐詞,
        }
    }
}

#[derive(Clone)]
pub enum 字幕格式<'a> {
    自動生成,
    自訂(&'a str),
    詞句(&'static str),
    段落(字幕步進, &'static str),
}

struct 字幕指標<'a> {
    字幕: &'a str,
    指標: usize,
}

impl<'a> From<&'a str> for 字幕指標<'a> {
    fn from(字幕: &'a str) -> Self {
        Self { 字幕, 指標: 0 }
    }
}

/// 迭代字幕中的文字.
/// 傳入的字幕應當是從空白處切分出的一段.
/// 通常一音對一字. 例外情況用文字組標記 `[]` 括住與一個音節對應的一組文字.
/// 文字組不能包含空白字符及左右方括號.
impl Iterator for 字幕指標<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut 剩餘文字 = self.字幕.chars().skip(self.指標);
        match 剩餘文字.next() {
            Some('[') => {
                // 將文字組標記 [] 中的文字串視作一個文字
                let 文字組 = 剩餘文字.take_while(|字| *字 != ']');
                self.指標 += 文字組.clone().count() + 2;
                Some(文字組.collect())
            }
            Some(單字) => {
                self.指標 += 1;
                Some(單字.to_string())
            }
            None => None,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct 字幕段落<'a>(pub usize, pub usize, pub Cow<'a, str>);

fn 標註字序<'a>(衆段落: impl Iterator<Item = Cow<'a, str>>) -> Box<[字幕段落<'a>]> {
    let 未有段落 = Box::new(vec![]);
    衆段落
        .fold((0, 未有段落), |(起, mut 已標註字序的段落), 又一段| {
            let 止 = 起 + 字幕指標::from(又一段.as_ref()).count();
            (*已標註字序的段落).push(字幕段落(起, 止, 又一段));
            (止, 已標註字序的段落)
        })
        .1
        .into_boxed_slice()
}

#[derive(Clone)]
pub struct 字幕表示 {
    pub 已完成: String,
    pub 指標文字: String,
    pub 未完成: String,
}

#[derive(Clone, Copy)]
pub struct 字幕機關輸出信號 {
    pub 分段字幕: Memo<Box<[字幕段落<'static>]>>,
    pub 當前段落: Memo<Option<字幕段落<'static>>>,
    pub 段落表示: Signal<Option<字幕表示>>,
}

pub fn 字幕機關(
    方案: &輸入方案機關輸出信號,
    作業: &作業機關輸出信號,
) -> 字幕機關輸出信號 {
    let 指法 = 方案.指法;
    let 當前作業 = 作業.當前作業;
    let 作業進度 = 作業.作業進度;
    let 目標輸入碼序列 = 作業.目標輸入碼序列;

    let 分段字幕 = Memo::new(move |_| match 當前作業.read().字幕() {
        字幕格式::自動生成 => {
            let 步進 = 字幕步進::from(指法());
            生成字幕(步進, &目標輸入碼序列.read())
        }
        字幕格式::自訂(字幕) => {
            標註字序(字幕.split_whitespace().map(String::from).map(Cow::Owned))
        }
        字幕格式::詞句(字幕) => 標註字序(字幕.split_whitespace().map(Cow::Borrowed)),
        字幕格式::段落(字幕步進::逐字, 字幕) => 標註字序(
            字幕
                .lines()
                .map(|每一行| 每一行.split_whitespace().collect::<Vec<_>>().join("[ ]"))
                .map(Cow::Owned),
        ),
        字幕格式::段落(字幕步進::逐詞, 字幕) => 標註字序(
            字幕
                .lines()
                .map(|每一行| {
                    每一行
                        .split_whitespace()
                        .flat_map(|每個詞| ["[", 每個詞, " ]"])
                        .collect::<String>()
                })
                .map(Cow::Owned),
        ),
    });

    let 當前段落 = Memo::new(move |_| {
        分段字幕.with(|衆段落| {
            let 全文進度 = 作業進度();
            let 當前段落號 =
                衆段落.partition_point(|字幕段落(_, 段落結束, _)| *段落結束 <= 全文進度);
            衆段落
                .get(當前段落號)
                .or_else(|| {
                    衆段落
                        .last()
                        .filter(|字幕段落(_, 全文結束, _)| *全文結束 == 全文進度)
                })
                .cloned()
        })
    });

    let 段落表示 = Signal::derive(move || {
        當前段落().map(|字幕段落(段落起始, _, 段落文字)| {
            let 全文進度 = 作業進度();
            let 段落進度 = 全文進度 - 段落起始;
            let 已完成 = 字幕指標::from(段落文字.as_ref())
                .take(段落進度)
                .collect::<String>();
            let 指標文字 = 字幕指標::from(段落文字.as_ref())
                .skip(段落進度)
                .take(1)
                .collect::<String>();
            let 未完成 = 字幕指標::from(段落文字.as_ref())
                .skip(段落進度 + 1)
                .collect::<String>();
            字幕表示 {
                已完成,
                指標文字,
                未完成,
            }
        })
    });

    字幕機關輸出信號 {
        分段字幕,
        當前段落,
        段落表示,
    }
}

fn 生成字幕<'a>(
    步進: 字幕步進, 輸入碼序列: &[對照輸入碼]
) -> Box<[字幕段落<'a>]> {
    if 輸入碼序列.is_empty() {
        Box::new([])
    } else {
        Box::new([字幕段落(
            0,
            輸入碼序列.len(),
            Cow::Owned(
                輸入碼序列
                    .iter()
                    .flat_map(對照輸入碼::顯示輸入碼)
                    .map(match 步進 {
                        字幕步進::逐字 => 字幕逐字步進,
                        字幕步進::逐詞 => 字幕逐詞步進,
                    })
                    .collect::<String>(),
            ),
        )])
    }
}

fn 字幕逐字步進(輸入碼: &str) -> Cow<'_, str> {
    Cow::Borrowed(輸入碼)
}

fn 字幕逐詞步進(輸入碼: &str) -> Cow<'_, str> {
    Cow::Owned(format!("[{輸入碼} ]"))
}
