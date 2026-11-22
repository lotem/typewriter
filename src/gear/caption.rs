use leptos::prelude::*;
use std::borrow::Cow;

use crate::definition::{碼表格式, 輸入方案定義};
use crate::gear::{
    assignment::{作業機關輸出信號, 對照輸入碼, 按行解析碼表, 碼表定義},
    theory::輸入方案機關輸出信號,
};

/// 練習題字幕定義 (二元正交設計):
/// - `自動`: 免寫字幕, 直接按碼表原文各非空行生成對應字幕段落 (`逐鍵` 逐字符對齊; `連擊`/`並擊` 按詞塊對齊).
/// - `詞句`: 顯式字幕文本, 按空白切分段落; 段內默認一字對一碼, 多字符合對一碼時以 `[文字組]` 括起.
#[derive(Clone, Debug, PartialEq)]
pub enum 字幕格式<'a> {
    自動,
    詞句(Cow<'a, str>),
}

impl<'a> 字幕格式<'a> {
    pub const fn 靜態詞句(文本: &'a str) -> Self {
        Self::詞句(Cow::Borrowed(文本))
    }
}

/// 單段字幕內容表示: 優先以 `Cow` 零拷貝借用靜態題庫切片, 同時免除渲染期 `[文字組]` 重複解析.
#[derive(Clone, Debug, PartialEq)]
pub enum 段落內容<'a> {
    /// 段內無合字標記: 每個 `char` 即對應 1 個步進位 (`[` 與 `]` 亦作普通字面字符處理).
    逐字(Cow<'a, str>),
    /// 段內含多字符步進單元 (如顯式 `[文字組]` 脫殼後的切片, 或 `連擊`/`並擊` 自動生成的詞塊).
    分塊(Box<[Cow<'a, str>]>),
}

impl 段落內容<'_> {
    pub fn 步數(&self) -> usize {
        match self {
            Self::逐字(文本) => 文本.chars().count(),
            Self::分塊(單元組) => 單元組.len(),
        }
    }

    pub fn 拼接全文(&self) -> Cow<'_, str> {
        match self {
            Self::逐字(文本) => Cow::Borrowed(文本.as_ref()),
            Self::分塊(單元組) => Cow::Owned(單元組.iter().map(AsRef::as_ref).collect()),
        }
    }

    pub fn 切分表示(&self, 進度: usize) -> 字幕表示 {
        match self {
            Self::逐字(文本) => {
                let 起字節 = 文本
                    .char_indices()
                    .nth(進度)
                    .map_or(文本.len(), |(位置, _)| 位置);
                let 止字節 = if 起字節 < 文本.len() {
                    let 當前字長 = 文本[起字節..].chars().next().map_or(0, char::len_utf8);
                    起字節 + 當前字長
                } else {
                    文本.len()
                };
                字幕表示 {
                    已完成: 文本[..起字節].to_string(),
                    指標文字: 文本[起字節..止字節].to_string(),
                    未完成: 文本[止字節..].to_string(),
                }
            }
            Self::分塊(單元組) => {
                let 位置 = 進度.min(單元組.len());
                let 拼接 = |切片: &[Cow<'_, str>]| -> String {
                    切片.iter().map(AsRef::as_ref).collect()
                };
                字幕表示 {
                    已完成: 拼接(&單元組[..位置]),
                    指標文字: 單元組
                        .get(位置)
                        .map_or_else(String::new, ToString::to_string),
                    未完成: if 位置 < 單元組.len() {
                        拼接(&單元組[(位置 + 1)..])
                    } else {
                        String::new()
                    },
                }
            }
        }
    }
}

/// 單段字幕 `(起始進度, 結束進度, 段落內容)`.
#[derive(Clone, Debug, PartialEq)]
pub struct 字幕段落<'a>(pub usize, pub usize, pub 段落內容<'a>);

/// 單遍線性掃描顯式 `詞句` 片段並借用原切片構造步進單元 (`Cow::Borrowed`, 零堆字符串拷貝).
fn 解析借用詞句單元<'a>(片段: &'a str) -> Box<[Cow<'a, str>]> {
    let mut 單元組 = Vec::new();
    let mut 索引流 = 片段.char_indices();
    while let Some((起, 首字)) = 索引流.next() {
        if 首字 == '[' {
            let 組起 = 起 + '['.len_utf8();
            let mut 組止 = 片段.len();
            for (位置, 字) in 索引流.by_ref() {
                if 字 == ']' {
                    組止 = 位置;
                    break;
                }
            }
            單元組.push(Cow::Borrowed(&片段[組起..組止]));
        } else {
            let 止 = 起 + 首字.len_utf8();
            單元組.push(Cow::Borrowed(&片段[起..止]));
        }
    }
    單元組.into_boxed_slice()
}

pub(crate) fn 編排字幕<'a>(
    碼表: &碼表定義<'a>,
    字幕: &字幕格式<'a>,
    方案: &輸入方案定義,
) -> Box<[字幕段落<'a>]> {
    match 字幕 {
        字幕格式::自動 => 生成自動字幕(碼表, 方案),
        字幕格式::詞句(Cow::Borrowed(靜態文本)) => {
            let mut 起 = 0usize;
            let mut 衆段落 = Vec::new();
            for 片段 in 靜態文本.split_whitespace() {
                let 內容 = if 片段.contains('[') {
                    段落內容::分塊(解析借用詞句單元(片段))
                } else {
                    段落內容::逐字(Cow::Borrowed(片段))
                };
                let 止 = 起 + 內容.步數();
                衆段落.push(字幕段落(起, 止, 內容));
                起 = 止;
            }
            衆段落.into_boxed_slice()
        }
        字幕格式::詞句(Cow::Owned(自有文本)) => {
            let mut 起 = 0usize;
            let mut 衆段落 = Vec::new();
            for 片段 in 自有文本.split_whitespace() {
                let 內容 = if 片段.contains('[') {
                    let 單元組 = 解析借用詞句單元(片段)
                        .into_vec()
                        .into_iter()
                        .map(|單元| Cow::Owned(單元.into_owned()))
                        .collect::<Box<[_]>>();
                    段落內容::分塊(單元組)
                } else {
                    段落內容::逐字(Cow::Owned(片段.to_string()))
                };
                let 止 = 起 + 內容.步數();
                衆段落.push(字幕段落(起, 止, 內容));
                起 = 止;
            }
            衆段落.into_boxed_slice()
        }
    }
}

fn 生成自動字幕<'a>(
    碼表: &碼表定義<'a>, 方案: &輸入方案定義
) -> Box<[字幕段落<'a>]> {
    let 實際格式 = 碼表.實際格式(方案);
    let mut 起 = 0usize;
    let mut 衆段落 = Vec::new();

    if 實際格式 == 碼表格式::逐鍵 {
        match &碼表.原文 {
            Cow::Borrowed(靜態原文) => {
                for 單行 in 靜態原文.lines().map(str::trim).filter(|行| !行.is_empty()) {
                    let 內容 = 段落內容::逐字(Cow::Borrowed(單行));
                    let 止 = 起 + 內容.步數();
                    衆段落.push(字幕段落(起, 止, 內容));
                    起 = 止;
                }
            }
            Cow::Owned(自有原文) => {
                for 單行 in 自有原文.lines().map(str::trim).filter(|行| !行.is_empty()) {
                    let 內容 = 段落內容::逐字(Cow::Owned(單行.to_string()));
                    let 止 = 起 + 內容.步數();
                    衆段落.push(字幕段落(起, 止, 內容));
                    起 = 止;
                }
            }
        }
        return 衆段落.into_boxed_slice();
    }

    for 行碼組 in 按行解析碼表(碼表, 方案) {
        if 行碼組.is_empty() {
            continue;
        }
        let 單元組: Box<[Cow<'a, str>]> = 行碼組
            .iter()
            .filter_map(對照輸入碼::顯示輸入碼)
            .map(|碼| Cow::Owned(format!("{碼} ")))
            .collect();
        let 止 = 起 + 單元組.len();
        衆段落.push(字幕段落(起, 止, 段落內容::分塊(單元組)));
        起 = 止;
    }
    衆段落.into_boxed_slice()
}

fn 所屬段落序號(衆段落: &[字幕段落], 進度: usize) -> usize {
    衆段落.partition_point(|字幕段落(_, 段落結束, _)| *段落結束 <= 進度)
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
    pub 前序段落: Signal<Option<字幕段落<'static>>>,
    pub 段落表示: Signal<Option<字幕表示>>,
}

pub fn 字幕機關(
    方案: &輸入方案機關輸出信號,
    作業: &作業機關輸出信號,
) -> 字幕機關輸出信號 {
    let 方案定義 = 方案.方案定義;
    let 作業進度 = 作業.作業進度;
    let 已載入作業 = 作業.已載入作業;

    let 分段字幕 = Memo::new(move |_| {
        已載入作業.借閱(|作業| {
            作業.map_or_else(
                || Box::from([]),
                |作業| 編排字幕(&作業.碼表, &作業.字幕, &方案定義.read()),
            )
        })
    });

    let 當前段落 = Memo::new(move |_| {
        分段字幕.with(|衆段落| {
            let 全文進度 = 作業進度();
            let 當前段落號 = 所屬段落序號(衆段落, 全文進度);
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

    let 前序段落 = Signal::derive(move || {
        分段字幕.with(|衆段落| {
            let 全文進度 = 作業進度();
            let 當前段落號 = 所屬段落序號(衆段落, 全文進度);
            if 當前段落號 == 0 {
                None
            } else {
                衆段落.get(當前段落號 - 1).cloned()
            }
        })
    });

    let 段落表示 = Signal::derive(move || {
        當前段落().map(|字幕段落(段落起始, _, 內容)| {
            內容.切分表示(作業進度().saturating_sub(段落起始))
        })
    });

    字幕機關輸出信號 {
        分段字幕,
        當前段落,
        前序段落,
        段落表示,
    }
}

#[cfg(test)]
mod 測試 {
    use super::*;
    use crate::drill::題目來源;
    use crate::gear::assignment::解析碼表;
    use crate::gear::theory::{方案選單, 輸入方案環境};

    #[test]
    fn 測試全部內建練習題編碼與字幕步數嚴格對齊() {
        let 宿主 = Owner::new();
        宿主.with(|| {
            let 環境 = 輸入方案環境 {
                已選配列: Memo::new(|_| None),
            };
            for (選項, 構造方案) in 方案選單.iter() {
                let 方案定義 = 構造方案(環境);
                let Some(題組) = 選項.配套練習題() else {
                    continue;
                };
                for 題 in 題組 {
                    if let 題目來源::內建 {
                        ref 編碼, ref 字幕
                    } = 題.題目
                    {
                        let 碼序列 = 解析碼表(編碼, &方案定義);
                        let 衆段落 = 編排字幕(編碼, 字幕, &方案定義);
                        let 字幕總步數 = 衆段落.last().map_or(0, |字幕段落(_, 止, _)| *止);
                        assert_eq!(
                            碼序列.len(),
                            字幕總步數,
                            "方案 [{}] 練習題 [{}] 編碼數 ({}) 與字幕步數 ({}) 不一致",
                            選項,
                            題.標題,
                            碼序列.len(),
                            字幕總步數,
                        );
                    }
                }
            }
        });
    }

    #[test]
    fn 測試零拷貝借用與方括號原樣保留() {
        let 宿主 = Owner::new();
        宿主.with(|| {
            let 環境 = 輸入方案環境 {
                已選配列: Memo::new(|_| None),
            };
            // 1. 靜態詞句 (普通段與合字段) 均以 `Cow::Borrowed` 零拷貝借用原切片
            let 拼音方案 = crate::theory::combo_pinyin::宮保拼音輸入方案(環境);
            let 靜態碼表 = 碼表定義::靜態隨方案("xian wen SHB=<shi bu shi>");
            let 靜態字幕 = 字幕格式::靜態詞句("先問[是不是]");
            let 詞句段落 = 編排字幕(&靜態碼表, &靜態字幕, &拼音方案);
            assert_eq!(詞句段落.len(), 1);
            if let 段落內容::分塊(ref 單元組) = 詞句段落[0].2 {
                assert!(單元組.iter().all(|單元| matches!(單元, Cow::Borrowed(_))));
                assert_eq!(單元組[2].as_ref(), "是不是");
            } else {
                panic!("含 [文字組] 段落應解析爲分塊");
            }

            // 2. 逐鍵直打與反查: 靜態文本保持 `Cow::Borrowed`, 且字面量 `[ab]` 原樣拆爲 4 步
            let 拉丁方案 = crate::theory::alphabet::拉丁字母輸入方案(環境);
            let 英文碼表 = 碼表定義::靜態隨方案("[ab]\ngo");
            let 英文段落 = 編排字幕(&英文碼表, &字幕格式::自動, &拉丁方案);
            assert_eq!(英文段落.len(), 2);
            assert!(matches!(
                英文段落[0].2,
                段落內容::逐字(Cow::Borrowed("[ab]"))
            ));
            assert_eq!(英文段落[0].1, 4);
            let 表示 = 英文段落[0].2.切分表示(0);
            assert_eq!(表示.指標文字, "[");
            assert_eq!(表示.未完成, "ab]");

            // 3. 連擊方案反查多字碼時按詞塊對齊
            let 注音方案 = crate::theory::zhuyin::注音輸入方案(環境);
            let 注音碼表 = 碼表定義::靜態隨方案("ㄓㄨㄥˉ ㄍㄨㄛˊ\nㄉㄧˋ");
            let 注音段落 = 編排字幕(&注音碼表, &字幕格式::自動, &注音方案);
            assert_eq!(注音段落.len(), 2);
            assert_eq!(注音段落[0].1, 2);
            assert_eq!(注音段落[1].1, 3);
        });
    }
}
