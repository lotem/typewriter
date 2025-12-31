use lazy_regex::regex;
use leptos::logging::log;
use leptos::prelude::*;
use std::cmp::min;

use crate::action::*;
use crate::definition::{ 击键方式, 转写定义, 输入方案定义 };
use crate::gear::{ caption::字幕格式, theory::{ 方案选项, 输入方案输出信号 } };
use crate::spelling_algebra::施展拼写运算;

#[derive(Clone, PartialEq)]
pub struct 作业 {
    pub 科目: 方案选项,
    pub 题号: Option<usize>,
    pub 自定义反查码: Option<String>,
}

impl 作业 {
    pub fn 练习题(科目: 方案选项, 题号: usize) -> Self {
        Self {
            科目,
            题号: Some(题号),
            自定义反查码: None,
        }
    }

    pub fn 自定义(科目: 方案选项, 反查码: String) -> Self {
        Self {
            科目,
            题号: None,
            自定义反查码: Some(反查码),
        }
    }

    pub fn 自习(科目: 方案选项) -> Self {
        Self {
            科目,
            题号: None,
            自定义反查码: None,
        }
    }

    fn 自定义编码(&self) -> Option<&str> {
        self.自定义反查码.as_ref().and_then(|反查码| {
            反查码.split_once("//")
                .map(|(编码, _字幕)| 编码.trim())
                .or(Some(反查码.as_str()))
        })
    }

    fn 自定义字幕(&self) -> Option<&str> {
        self.自定义反查码
            .as_ref()
            .and_then(|反查码| 反查码.split_once("//").map(|(_编码, 字幕)| 字幕.trim()))
    }

    pub fn 目标输入码(&self) -> Option<&str> {
        self.科目
            .配套练习题()
            .and_then(|练习题| self.题号.and_then(|题号| 练习题.get(题号)))
            .map(|题目| 题目.编码)
            .or(self.自定义编码())
    }

    pub fn 字幕(&self) -> 字幕格式<'_> {
        self.科目
            .配套练习题()
            .and_then(|练习题| self.题号.and_then(|题号| 练习题.get(题号)))
            .map(|题目| 题目.字幕.clone())
            .or_else(|| self.自定义字幕().map(字幕格式::自定义))
            .unwrap_or(字幕格式::自动生成)
    }

    pub fn 是否练习题(&self) -> bool {
        self.题号.is_some()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct 对照输入码 {
    pub 字根码原文: Option<String>,
    pub 转写码原文: Option<String>,
}

impl 对照输入码 {
    pub fn 反查字根码<'a>(&'a self, 转写: &转写定义<'a>) -> Option<String> {
        self.字根码原文.to_owned().or_else(|| {
            self.转写码原文
                .as_deref()
                .filter(|转写码| 转写.验证拼式(转写码))
                .and_then(|转写码| 转写.拼式拆分为字根码(转写码))
                .and_then(|字根码| 施展拼写运算(&字根码, 转写.编码预览).or(Some(字根码)))
        })
    }

    /// 用于显示的输入码. 优先显示转写码.
    pub fn 显示输入码(&self) -> Option<&str> {
        self.转写码原文.as_deref().or(self.字根码原文.as_deref())
    }
}

#[derive(Clone, Copy, Default)]
pub struct 步进设置 {
    pub 目标: Option<usize>,
    pub 循环: bool,
}

pub type 重置作业进度动作 = impl 动作;
pub type 作业推进动作 = impl 动作给一参数得一结果<步进设置>;
pub type 作业回退动作 = impl 动作给一参数得一结果<步进设置>;

#[derive(Clone)]
pub struct 作业模式输出信号 {
    pub 当前作业: ReadSignal<作业>,
    pub 布置作业: WriteSignal<作业>,
    pub 作业进度: ReadSignal<usize>,
    pub 重置作业进度: 重置作业进度动作,
    pub 目标输入码序列: Memo<Box<[对照输入码]>>,
    pub 目标输入码片段: Memo<Option<对照输入码>>,
    pub 作业推进: 作业推进动作,
    pub 作业回退: 作业回退动作,
    pub 有无作业: Signal<bool>,
    pub 作业完成: Signal<bool>,
}

#[define_opaque(重置作业进度动作, 作业推进动作, 作业回退动作)]
pub fn 作业模式(方案: &输入方案输出信号) -> 作业模式输出信号 {
    let 当前方案 = 方案.当前方案;
    let 方案定义 = 方案.方案定义;
    let 初始方案 = 当前方案.get_untracked();
    let (当前作业, 布置作业) = signal(作业::练习题(初始方案, 0));

    let _ = Effect::watch(
        当前方案,
        move |&方案, _, _| {
            布置作业(作业::练习题(方案, 0));
        },
        false
    );

    let (作业进度, 更新作业进度) = signal(0);

    let 重置作业进度 = move || {
        更新作业进度(0);
    };

    let 目标输入码序列 = Memo::new(move |_| {
        当前作业.read()
            .目标输入码()
            .map(|输入码| 解析输入码序列(输入码, &方案定义.read()))
            .unwrap_or(Box::new([]))
    });

    let _ = Effect::watch(
        目标输入码序列,
        move |目标输入码序列, _, _| {
            log!("更新了目标输入码: {}", 目标输入码序列.len());
            重置作业进度();
        },
        false
    );

    let 目标输入码片段 = Memo::new(move |_| {
        目标输入码序列.with(|输入码| {
            if 输入码.is_empty() {
                None
            } else {
                输入码.get(min(作业进度(), 输入码.len() - 1)).cloned()
            }
        })
    });

    let 作业推进 = move |步进: 步进设置| {
        let 当前进度 = 作业进度();
        let 全文长度 = 目标输入码序列.read().len();
        let 目标进度 = 步进.目标.unwrap_or(当前进度 + 1);
        if 步进.循环 && 目标进度 >= 全文长度 {
            重置作业进度();
            Ok(())
        } else if
            // 非循环状态可推进至全文结束位置
            目标进度 <= 全文长度
        {
            更新作业进度(目标进度);
            Ok(())
        } else {
            Err(未有())
        }
    };

    let 作业回退 = move |步进: 步进设置| {
        let 当前进度 = 作业进度();
        let 全文长度 = 目标输入码序列.read().len();
        match 步进.目标 {
            Some(目标进度) if 步进.循环 || 当前进度 > 目标进度 => {
                更新作业进度(目标进度);
                Ok(())
            }
            None if 步进.循环 && 当前进度 == 0 && 全文长度 > 0 => {
                更新作业进度(全文长度 - 1);
                Ok(())
            }
            None if 当前进度 > 0 => {
                更新作业进度(当前进度 - 1);
                Ok(())
            }
            _ => Err(未有()),
        }
    };

    let 有无作业 = Signal::derive(move || 当前作业.read().目标输入码().is_some());

    let 输入码总数 = move || 目标输入码序列.read().len();

    let 作业完成 = Signal::derive(move || 有无作业() && 作业进度() == 输入码总数());

    作业模式输出信号 {
        当前作业,
        布置作业,
        作业进度,
        重置作业进度,
        目标输入码序列,
        目标输入码片段,
        作业推进,
        作业回退,
        有无作业,
        作业完成,
    }
}

fn 解析输入码序列(输入码序列: &str, 方案: &输入方案定义) -> Box<[对照输入码]> {
    match 方案.指法 {
        击键方式::连击 => 解析连击输入码序列(输入码序列, 方案),
        击键方式::并击 => 解析并击输入码序列(输入码序列),
    }
}

fn 解析连击输入码序列(输入码序列: &str, 方案: &输入方案定义) -> Box<[对照输入码]> {
    输入码序列
        .lines()
        .map(str::trim)
        .flat_map(|片段| 片段.chars())
        .map(|字符| {
            let 输入码原文 = 字符.to_string();
            if 方案.寻得字根(&输入码原文).is_some() {
                对照输入码 {
                    字根码原文: Some(输入码原文),
                    转写码原文: None,
                }
            } else {
                对照输入码 {
                    字根码原文: None,
                    转写码原文: Some(输入码原文),
                }
            }
        })
        .collect()
}

/// 将并击输入码序列解析为输入码片段.
///
/// 输入码通常是拼音音节的序列, 音节之间用空白或隔音符号 `'` 分开.
/// 特殊形式的拼音写在尖括号中, 如: `<'a>`。
///
/// 输入码片段也可以是以下形式:
///
/// - 用大写字母连书并击码, 如 `ZFURO`
/// - 写明并击码和对应的拼音, 如 `SHGUA=shu'ru'fa`
/// - 写明并击码并将对应的拼音写在尖括号中, 如 `SHGUA=<shu ru fa>`
/// - 非大写字母的并击码，写在方括号中，如 `[端定]=<泥>`
fn 解析并击输入码序列(输入码序列: &str) -> Box<[对照输入码]> {
    let 输入码片段模式 = regex!(
        r"(?x)
        (?:
            (?P<chord> \p{Uppercase}+ ) |
            \[ (?P<non_ascii_chord> [^\]]+ ) \]
        )(?:
            = (?P<eq_code> [\w'] )+ |
            =< (?P<eq_quoted_code> [^<>]* ) >
        )? |
        (?P<code> \w+ ) |
        <(?P<quoted_code> [^<>]* )>
    "
    );
    输入码片段模式
        .captures_iter(输入码序列)
        .map(|片段| {
            let 并击码原文 = 片段
                .name("chord")
                .or_else(|| 片段.name("non_ascii_chord"))
                .map(|m| m.as_str().to_owned());
            let 转写码原文 = 片段
                .name("code")
                .or_else(|| 片段.name("quoted_code"))
                .or_else(|| 片段.name("eq_code"))
                .or_else(|| 片段.name("eq_quoted_code"))
                .map(|m| m.as_str().to_owned());
            对照输入码 {
                字根码原文: 并击码原文,
                转写码原文,
            }
        })
        .collect()
}
