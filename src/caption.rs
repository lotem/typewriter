use leptos::*;
use std::borrow::Cow;

use crate::assignment::作業;
use crate::engine::對照輸入碼;

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

#[allow(clippy::type_complexity)]
pub fn 字幕機關<'a>(
    當前作業: ReadSignal<作業>,
    作業進度: ReadSignal<usize>,
    輸入碼序列: Memo<Box<[對照輸入碼]>>,
) -> (
    // 分段字幕
    Memo<Box<[字幕段落<'a>]>>,
    // 當前段落
    Signal<Option<字幕段落<'a>>>,
    // 按進度顯示字幕段落
    Signal<Option<(String, String, String)>>,
) {
    let 未有段落 = || Box::new(vec![]);
    let 分段字幕 = create_memo(move |_| {
        當前作業.with(|作業| {
            作業.字幕().map_or_else(
                || 輸入碼序列.with(|輸入碼| 生成字幕(輸入碼)),
                move |有字幕| {
                    有字幕
                        .split_whitespace()
                        .fold(
                            (0, 未有段落()),
                            |(起始字序, mut 已標註字序的段落), 又一段| {
                                let 結束字序 = 起始字序 + 字幕指標::from(又一段).count();
                                (*已標註字序的段落).push(字幕段落(
                                    起始字序,
                                    結束字序,
                                    Cow::Borrowed(又一段),
                                ));
                                (結束字序, 已標註字序的段落)
                            },
                        )
                        .1
                        .into_boxed_slice()
                },
            )
        })
    });

    let 當前段落 = Signal::derive(move || {
        分段字幕.with(|衆段落| {
            let 全文進度 = 作業進度();
            let 當前段落號 =
                衆段落.partition_point(|字幕段落(_, 段落結束, _)| *段落結束 <= 全文進度);
            衆段落.get(當前段落號).cloned()
        })
    });

    let 按進度顯示字幕段落 = Signal::derive(move || {
        當前段落().map(|字幕段落(段落起始, _, 段落文字)| {
            let 全文進度 = 作業進度();
            let 段落進度 = 全文進度 - 段落起始;
            let 完成的字 = 字幕指標::from(段落文字.as_ref())
                .take(段落進度)
                .collect::<String>();
            let 當下的字 = 字幕指標::from(段落文字.as_ref())
                .skip(段落進度)
                .take(1)
                .collect::<String>();
            let 剩餘的字 = 字幕指標::from(段落文字.as_ref())
                .skip(段落進度 + 1)
                .collect::<String>();
            (完成的字, 當下的字, 剩餘的字)
        })
    });

    (分段字幕, 當前段落, 按進度顯示字幕段落)
}

fn 生成字幕<'a>(輸入碼序列: &[對照輸入碼]) -> Box<[字幕段落<'a>]> {
    vec![字幕段落(
        0,
        輸入碼序列.len(),
        Cow::Owned(輸入碼序列.iter().map(輸入碼做字幕).collect::<String>()),
    )]
    .into_boxed_slice()
}

fn 輸入碼做字幕(對照碼: &對照輸入碼) -> String {
    對照碼
        .轉寫碼原文
        .as_ref()
        .or(對照碼.字根碼原文.as_ref())
        .map(|輸入碼| format!("[{輸入碼} ]"))
        .unwrap_or_default()
}
