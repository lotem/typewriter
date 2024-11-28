use lazy_regex::Regex;
use maybe_owned::MaybeOwned;
use std::collections::HashMap;

pub enum 拼寫運算<'a> {
    變換 {
        模式: MaybeOwned<'a, Regex>,
        替換文字: &'a str,
    },
    轉寫 {
        字符映射: HashMap<char, char>,
    },
    消除 {
        模式: MaybeOwned<'a, Regex>,
    },
}

#[macro_export]
macro_rules! 變換 {
    ($模式:literal, $替換文字:literal) => {
        拼寫運算::變換 {
            模式: regex!($模式).deref().into(),
            替換文字: $替換文字,
        }
    };
}

#[macro_export]
macro_rules! 轉寫 {
    ($左字表:literal, $右字表:literal) => {
        拼寫運算::轉寫 {
            字符映射: std::iter::zip($左字表.chars(), $右字表.chars()).collect(),
        }
    };
}

#[macro_export]
macro_rules! 消除 {
    ($模式:literal) => {
        拼寫運算::消除 {
            模式: regex!($模式).deref().into(),
        }
    };
}

pub fn 施展拼寫運算(原形: &str, 運算規則: &[拼寫運算]) -> Option<String> {
    if 原形.is_empty() {
        return None;
    }
    let mut 運算結果 = 原形.to_owned();
    for 運算 in 運算規則 {
        match 運算 {
            拼寫運算::變換 {
                ref 模式, 替換文字
            } => {
                運算結果 = 模式.replace_all(&運算結果, *替換文字).to_string();
            }
            拼寫運算::轉寫 { ref 字符映射 } => {
                運算結果 = 運算結果
                    .chars()
                    .map(|字符| 字符映射.get(&字符).copied().unwrap_or(字符))
                    .collect::<String>();
            }
            拼寫運算::消除 { ref 模式 } => {
                if 模式.is_match(&運算結果) {
                    return None;
                }
            }
        };
    }
    (!運算結果.is_empty()).then_some(運算結果)
}
