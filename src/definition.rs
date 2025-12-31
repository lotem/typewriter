use lazy_regex::Regex;
use std::borrow::Cow;
use std::collections::BTreeSet;

use crate::gear::layout::键盘布局;
use crate::key_code::KeyCode;
use crate::spelling_algebra::{ 拼写运算, 施展拼写运算 };

pub struct 键位映射定义<'a> {
    pub 输入码: &'a str,
    pub 键码: KeyCode,
}

#[derive(Clone, Copy)]
pub struct 输入方案定义<'a> {
    pub 名称: &'a str,
    pub 布局: &'a 键盘布局,
    pub 指法: 击键方式,
    pub 键位映射: &'a [键位映射定义<'a>],
    pub 转写: 转写定义<'a>,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum 击键方式 {
    #[default]
    连击,
    并击,
}

#[derive(Clone, Copy)]
pub struct 转写定义<'a> {
    /// 将按键序列转换成惯用的表示形式，如字母与附标符号合字
    pub 编码预览: &'a [拼写运算<'a>],
    /// 将输入码的表示形式转换成按键序列
    pub 键位提示: &'a [拼写运算<'a>],
    /// 将输入码转写成符合词典规范的编码
    pub 输入棱镜: &'a [拼写运算<'a>],
    /// 将词典码拆分为按键序列
    pub 词库棱镜: &'a [拼写运算<'a>],
    /// 定义若干识别有效词典码的规则。若未定义任何规则，则不做验证
    pub 拼式验证规则: &'a [&'a Regex],
}

pub trait 判定键位 {
    fn 有无键位(&self) -> bool;
    fn 包含键位(&self, 键码: &KeyCode) -> bool;
}

#[derive(Clone, Debug, PartialEq)]
pub struct 键组(pub BTreeSet<KeyCode>);

impl 键组 {
    pub fn new() -> Self {
        键组(BTreeSet::new())
    }
}

impl 判定键位 for &键组 {
    fn 有无键位(&self) -> bool {
        !self.0.is_empty()
    }

    fn 包含键位(&self, 键码: &KeyCode) -> bool {
        self.0.contains(键码)
    }
}

impl 判定键位 for KeyCode {
    fn 有无键位(&self) -> bool {
        *self != KeyCode::No
    }

    fn 包含键位(&self, 键码: &KeyCode) -> bool {
        self == 键码
    }
}

impl 输入方案定义<'_> {
    pub fn 寻得字根(&self, 字根: &str) -> Option<&键位映射定义<'_>> {
        self.键位映射.iter().find(|键| 键.输入码 == 字根)
    }

    pub fn 读出键位(&self, 字根码: &str) -> 键组 {
        let 键码序列 = 施展拼写运算(字根码, self.转写.键位提示)
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(字根码));
        键组(
            self.键位映射
                .iter()
                .filter(|键| 键码序列.contains(键.输入码))
                .map(|键| 键.键码)
                .collect()
        )
    }

    pub fn 写成字根码(&self, 键位: impl 判定键位) -> String {
        if !键位.有无键位() {
            String::new()
        } else {
            let 字根码 = self.键位映射
                .iter()
                .filter(|键| 键位.包含键位(&键.键码))
                .map(|键| 键.输入码)
                .collect::<String>();
            施展拼写运算(&字根码, self.转写.编码预览).unwrap_or(字根码)
        }
    }
}

impl 转写定义<'_> {
    pub fn 字根码转写为拼式(&self, 字根码: &str) -> Option<String> {
        施展拼写运算(字根码, self.输入棱镜)
    }

    pub fn 拼式拆分为字根码(&self, 转写码: &str) -> Option<String> {
        施展拼写运算(转写码, self.词库棱镜)
    }

    pub fn 验证拼式(&self, 待验证拼式: &str) -> bool {
        self.拼式验证规则.iter().any(|r| r.is_match(待验证拼式))
    }
}

#[macro_export]
macro_rules! 默认映射 {
    ($字母:ident) => {
        键位映射定义 {
            输入码: stringify!($字母),
            键码: KeyCode::$字母,
        }
    };
}

#[macro_export]
macro_rules! 键位映射 {
    ($输入码:ident => $键码:path) => {
        键位映射定义 {
            输入码: stringify!($输入码),
            键码: $键码,
        }
    };
}
