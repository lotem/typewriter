use lazy_regex::Regex;
use maybe_owned::MaybeOwned;
use std::collections::HashMap;

pub enum 拼写运算<'a> {
    变换 {
        模式: MaybeOwned<'a, Regex>,
        替换文字: &'a str,
    },
    转写 {
        字符映射: HashMap<char, char>,
    },
    消除 {
        模式: MaybeOwned<'a, Regex>,
    },
    派生 {
        模式: MaybeOwned<'a, Regex>,
        替换文字: &'a str,
    },
    模糊 {
        模式: MaybeOwned<'a, Regex>,
        替换文字: &'a str,
    },
    缩写 {
        模式: MaybeOwned<'a, Regex>,
        替换文字: &'a str,
    },
}

#[macro_export]
macro_rules! 变换 {
    ($模式:literal, $替换文字:literal) => {
        拼写运算::变换 {
            模式: regex!($模式).deref().into(),
            替换文字: $替换文字,
        }
    };
}

#[macro_export]
macro_rules! 转写 {
    ($左字表:literal, $右字表:literal) => {
        拼写运算::转写 {
            字符映射: std::iter::zip($左字表.chars(), $右字表.chars()).collect(),
        }
    };
}

#[macro_export]
macro_rules! 消除 {
    ($模式:literal) => {
        拼写运算::消除 {
            模式: regex!($模式).deref().into(),
        }
    };
}

#[macro_export]
macro_rules! 派生 {
    ($模式:literal, $替换文字:literal) => {
        拼写运算::派生 {
            模式: regex!($模式).deref().into(),
            替换文字: $替换文字,
        }
    };
}

#[macro_export]
macro_rules! 模糊 {
    ($模式:literal, $替换文字:literal) => {
        拼写运算::模糊 {
            模式: regex!($模式).deref().into(),
            替换文字: $替换文字,
        }
    };
}

#[macro_export]
macro_rules! 缩写 {
    ($模式:literal, $替换文字:literal) => {
        拼写运算::缩写 {
            模式: regex!($模式).deref().into(),
            替换文字: $替换文字,
        }
    };
}

pub fn 施展拼写运算(原形: &str, 运算规则: &[拼写运算]) -> Option<String> {
    if 原形.is_empty() {
        return None;
    }
    let mut 运算结果 = 原形.to_owned();
    for 运算 in 运算规则 {
        match 运算 {
            拼写运算::变换 {
                ref 模式, 替换文字
            }
            | 拼写运算::派生 {
                ref 模式, 替换文字
            }
            | 拼写运算::模糊 {
                ref 模式, 替换文字
            }
            | 拼写运算::缩写 {
                ref 模式, 替换文字
            } => {
                运算结果 = 模式.replace_all(&运算结果, *替换文字).to_string();
            }
            拼写运算::转写 { ref 字符映射 } => {
                运算结果 = 运算结果
                    .chars()
                    .map(|字符| 字符映射.get(&字符).copied().unwrap_or(字符))
                    .collect::<String>();
            }
            拼写运算::消除 { ref 模式 } => {
                if 模式.is_match(&运算结果) {
                    return None;
                }
            }
        };
    }
    (!运算结果.is_empty()).then_some(运算结果)
}
