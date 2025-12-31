use leptos::prelude::*;
use std::borrow::Cow;

use crate::definition::击键方式;
use crate::gear::{ assignment::{ 作业模式输出信号, 对照输入码 }, theory::输入方案输出信号 };

#[derive(Clone, Copy, PartialEq)]
pub enum 字幕步进 {
    逐字,
    逐词,
}

impl From<击键方式> for 字幕步进 {
    fn from(source: 击键方式) -> Self {
        match source {
            击键方式::连击 => 字幕步进::逐字,
            击键方式::并击 => 字幕步进::逐词,
        }
    }
}

#[derive(Clone)]
pub enum 字幕格式<'a> {
    自动生成,
    自定义(&'a str),
    词句(&'static str),
    段落(字幕步进, &'static str),
}

struct 字幕指标<'a> {
    字幕: &'a str,
    指标: usize,
}

impl<'a> From<&'a str> for 字幕指标<'a> {
    fn from(字幕: &'a str) -> Self {
        Self { 字幕, 指标: 0 }
    }
}

/// 迭代字幕中的文字.
/// 传入的字幕应当是从空白处切分出的一段.
/// 通常一音对一字. 例外情况用文字组标记 `[]` 括住与一个音节对应的一组文字.
/// 文字组不能包含空白字符及左右方括号.
impl Iterator for 字幕指标<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut 剩余文字 = self.字幕.chars().skip(self.指标);
        match 剩余文字.next() {
            Some('[') => {
                // 将文字组标记 [] 中的文字串视作一个文字
                let 文字组 = 剩余文字.take_while(|字| *字 != ']');
                self.指标 += 文字组.clone().count() + 2;
                Some(文字组.collect())
            }
            Some(单字) => {
                self.指标 += 1;
                Some(单字.to_string())
            }
            None => None,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct 字幕段落<'a>(pub usize, pub usize, pub Cow<'a, str>);

fn 标注字序<'a>(众段落: impl Iterator<Item = Cow<'a, str>>) -> Box<[字幕段落<'a>]> {
    let 未有段落 = Box::new(vec![]);
    众段落.fold((0, 未有段落), |(起, mut 已标注字序的段落), 又一段| {
        let 止 = 起 + 字幕指标::from(又一段.as_ref()).count();
        (*已标注字序的段落).push(字幕段落(起, 止, 又一段));
        (止, 已标注字序的段落)
    }).1.into_boxed_slice()
}

fn 所属段落序号(众段落: &[字幕段落], 进度: usize) -> usize {
    众段落.partition_point(|字幕段落(_, 段落结束, _)| *段落结束 <= 进度)
}

#[derive(Clone)]
pub struct 字幕表示 {
    pub 已完成: String,
    pub 指标文字: String,
    pub 未完成: String,
}

#[derive(Clone, Copy)]
pub struct 字幕机关输出信号 {
    pub 分段字幕: Memo<Box<[字幕段落<'static>]>>,
    pub 当前段落: Memo<Option<字幕段落<'static>>>,
    pub 前序段落: Signal<Option<字幕段落<'static>>>,
    pub 段落表示: Signal<Option<字幕表示>>,
}

pub fn 字幕机关(方案: &输入方案输出信号, 作业: &作业模式输出信号) -> 字幕机关输出信号 {
    let 指法 = 方案.指法;
    let 当前作业 = 作业.当前作业;
    let 作业进度 = 作业.作业进度;
    let 目标输入码序列 = 作业.目标输入码序列;

    let 分段字幕 = Memo::new(move |_| {
        match 当前作业.read().字幕() {
            字幕格式::自动生成 => {
                let 步进 = 字幕步进::from(指法());
                生成字幕(步进, &目标输入码序列.read())
            }
            字幕格式::自定义(字幕) => {
                标注字序(字幕.split_whitespace().map(String::from).map(Cow::Owned))
            }
            字幕格式::词句(字幕) => 标注字序(字幕.split_whitespace().map(Cow::Borrowed)),
            字幕格式::段落(字幕步进::逐字, 字幕) =>
                标注字序(
                    字幕
                        .lines()
                        .map(|每一行| 每一行.split_whitespace().collect::<Vec<_>>().join("[ ]"))
                        .map(Cow::Owned)
                ),
            字幕格式::段落(字幕步进::逐词, 字幕) =>
                标注字序(
                    字幕
                        .lines()
                        .map(|每一行| {
                            每一行.split_whitespace()
                                .flat_map(|每个词| ["[", 每个词, " ]"])
                                .collect::<String>()
                        })
                        .map(Cow::Owned)
                ),
        }
    });

    let 当前段落 = Memo::new(move |_| {
        分段字幕.with(|众段落| {
            let 全文进度 = 作业进度();
            let 当前段落号 = 所属段落序号(众段落, 全文进度);
            众段落.get(当前段落号)
                .or_else(|| {
                    众段落.last().filter(|字幕段落(_, 全文结束, _)| *全文结束 == 全文进度)
                })
                .cloned()
        })
    });

    let 前序段落 = Signal::derive(move || {
        分段字幕.with(|众段落| {
            let 全文进度 = 作业进度();
            let 当前段落号 = 所属段落序号(众段落, 全文进度);
            if 当前段落号 == 0 {
                None
            } else {
                众段落.get(当前段落号 - 1).cloned()
            }
        })
    });

    let 段落表示 = Signal::derive(move || {
        当前段落().map(|字幕段落(段落起始, _, 段落文字)| {
            let 全文进度 = 作业进度();
            let 段落进度 = 全文进度 - 段落起始;
            let 已完成 = 字幕指标::from(段落文字.as_ref()).take(段落进度).collect::<String>();
            let 指标文字 = 字幕指标
                ::from(段落文字.as_ref())
                .skip(段落进度)
                .take(1)
                .collect::<String>();
            let 未完成 = 字幕指标
                ::from(段落文字.as_ref())
                .skip(段落进度 + 1)
                .collect::<String>();
            字幕表示 {
                已完成,
                指标文字,
                未完成,
            }
        })
    });

    字幕机关输出信号 {
        分段字幕,
        当前段落,
        前序段落,
        段落表示,
    }
}

fn 生成字幕<'a>(步进: 字幕步进, 输入码序列: &[对照输入码]) -> Box<[字幕段落<'a>]> {
    if 输入码序列.is_empty() {
        Box::new([])
    } else {
        Box::new([
            字幕段落(
                0,
                输入码序列.len(),
                Cow::Owned(
                    输入码序列
                        .iter()
                        .flat_map(对照输入码::显示输入码)
                        .map(match 步进 {
                            字幕步进::逐字 => 字幕逐字步进,
                            字幕步进::逐词 => 字幕逐词步进,
                        })
                        .collect::<String>()
                )
            ),
        ])
    }
}

fn 字幕逐字步进(输入码: &str) -> Cow<'_, str> {
    Cow::Borrowed(输入码)
}

fn 字幕逐词步进(输入码: &str) -> Cow<'_, str> {
    Cow::Owned(format!("[{输入码} ]"))
}
