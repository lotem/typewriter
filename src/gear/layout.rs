//! 本节用到的概念
//! - 键盘配列 :: 描述物理按键的数目和空间排布
//! - 矩阵 :: 定义按键逻辑上的行列座标, 采用 ISO/IEC 9995-1 的键位标注法
//! - 键盘布局 :: 各个盘面及各个位置上的字符定义
//! - 盘面 :: 也称层, 定义在盘面选择码指定的状态下各键码映射到哪些字符
//! - 键面刻印 :: 键面显示的文字

use leptos::prelude::*;
use strum::{ Display, EnumIter };

use crate::gear::theory::输入方案输出信号;
use crate::key_code::KeyCode;

#[derive(Clone, Copy, Default)]
pub struct 盘面序号(pub u64);

impl 盘面序号 {
    pub fn 是否可选盘面(&self, 盘面号: usize) -> bool {
        (盘面号 == 0) || ((self.0 & (1 << (盘面号 - 1))) != 0)
    }

    pub fn 顶层盘面(&self) -> usize {
        (0..64).rfind(|&盘面号| self.是否可选盘面(盘面号)).unwrap_or_default()
    }
}

#[derive(Clone, Copy)]
pub struct 矩阵坐标(pub usize, pub usize);

/// 矩阵的行列座标按照 ISO/IEC 9995-1 的键位标注法显示.
/// 空格至数字行从下到上编号为 A 到 E.
/// 本品只做字母数字区, 因此 A 行列号从 03 开始, 其他各行从 01 开始.
impl std::fmt::Display for 矩阵坐标 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.0 {
            // (0, 0) -> E01
            i @ 0..=3 => {
                let 行号 = char::from_u32(('E' as u32) - (i as u32)).ok_or(std::fmt::Error {})?;
                let 列号 = self.1 + 1;
                write!(f, "{行号}{列号}")
            }
            // (4, 0) -> A03
            4 => {
                let 行号 = 'A';
                let 列号 = self.1 + 3;
                write!(f, "{行号}{列号}")
            }
            _ => Err(std::fmt::Error {}),
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct 刻印说明 {
    pub 中: Option<&'static str>,
    pub 上: Option<&'static str>,
    pub 下: Option<&'static str>,
    pub 左: Option<&'static str>,
    pub 右: Option<&'static str>,
}

impl 刻印说明 {
    pub const fn 居中(文字: &'static str) -> Self {
        Self {
            中: Some(文字),
            上: None,
            下: None,
            左: None,
            右: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum 键面刻印 {
    透明,
    无刻,
    有刻(刻印说明),
}

impl 键面刻印 {
    pub fn 居中刻印文字(&self) -> Option<&'static str> {
        match self {
            键面刻印::有刻(刻印说明 { 中: 有冇刻印, .. }) => *有冇刻印,
            _ => None,
        }
    }

    pub fn 上方刻印文字(&self) -> Option<&'static str> {
        match self {
            键面刻印::有刻(刻印说明 { 上: 有冇刻印, .. }) => *有冇刻印,
            _ => None,
        }
    }

    pub fn 下方刻印文字(&self) -> Option<&'static str> {
        match self {
            键面刻印::有刻(刻印说明 { 下: 有冇刻印, .. }) => *有冇刻印,
            _ => None,
        }
    }

    pub fn 左侧刻印文字(&self) -> Option<&'static str> {
        match self {
            键面刻印::有刻(刻印说明 { 左: 有冇刻印, .. }) => *有冇刻印,
            _ => None,
        }
    }

    pub fn 右侧刻印文字(&self) -> Option<&'static str> {
        match self {
            键面刻印::有刻(刻印说明 { 右: 有冇刻印, .. }) => *有冇刻印,
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct 键面映射 {
    pub 键码: KeyCode,
    pub 刻印: 键面刻印,
}

pub type 键盘矩阵<'a> = &'a [&'a [KeyCode]];
pub type 盘面定义<'a> = &'a [&'a [键面刻印]];

#[derive(Clone, Copy, Default, Display, EnumIter, PartialEq)]
pub enum 键盘配列 {
    #[default]
    主键盘区,
    字母键盘,
    正交直列,
    直列分体,
    正交直列带数字行,
    直列分体带数字行,
}

impl 键盘配列 {
    pub fn 横向交错(&self) -> bool {
        matches!(self, 键盘配列::主键盘区 | 键盘配列::字母键盘)
    }
    pub fn 直列(&self) -> bool {
        matches!(
            self,
            键盘配列::正交直列 |
                键盘配列::直列分体 |
                键盘配列::正交直列带数字行 |
                键盘配列::直列分体带数字行
        )
    }
    pub fn 分体(&self) -> bool {
        matches!(self, 键盘配列::直列分体 | 键盘配列::直列分体带数字行)
    }
    pub fn 规格(&self) -> usize {
        match self {
            键盘配列::主键盘区 => 60,
            键盘配列::字母键盘 => 30,
            键盘配列::正交直列 | 键盘配列::直列分体 => 30,
            键盘配列::正交直列带数字行 | 键盘配列::直列分体带数字行 => 60,
        }
    }
    pub fn 矩阵(&self) -> 键盘矩阵<'static> {
        match self {
            键盘配列::主键盘区 => 主键盘区矩阵,
            键盘配列::字母键盘 => 字母键盘矩阵,
            键盘配列::正交直列 => 正交直列矩阵,
            键盘配列::直列分体 => 直列分体矩阵,
            键盘配列::正交直列带数字行 => 正交直列带数字行矩阵,
            键盘配列::直列分体带数字行 => 直列分体带数字行矩阵,
        }
    }
}

#[derive(Clone, Copy)]
pub struct 键盘布局 {
    pub 盘面: &'static [盘面定义<'static>],
    pub 默认盘面: 盘面序号,
    pub 默认配列: 键盘配列,
}

impl 键盘布局 {
    pub fn 选择盘面(&self, 目标盘面: 盘面序号, 坐标: 矩阵坐标) -> Option<(usize, 键面刻印)> {
        self.盘面
            .iter()
            .enumerate()
            .rev()
            .filter(|&(盘面号, _)| 目标盘面.是否可选盘面(盘面号))
            .find_map(|(盘面号, 此盘面)| {
                self.从盘面读取刻印(此盘面, 坐标).and_then(|刻印| {
                    match 刻印 {
                        键面刻印::透明 => None,
                        _ => Some((盘面号, 刻印)),
                    }
                })
            })
    }

    fn 从盘面读取刻印(&self, 此盘面: 盘面定义<'static>, 坐标: 矩阵坐标) -> Option<键面刻印> {
        let 矩阵坐标(行, 列) = 坐标;
        此盘面.get(行)
            .and_then(|此行| 此行.get(列))
            .copied()
    }
}

macro_rules! 矩阵 {
    [$([$($键:ident)*] $(,)?)*] => {
        &[ $( &[ $( crate::key_code::KeyCode::$键, )* ], )* ]
    };
}

const 主键盘区矩阵: 键盘矩阵<'static> =
    矩阵![
    // ISO/IEC 9995-2 规定字母数字区至少包含 47 个用于输入字符的键位
    // 以下是协调 48 文字键盘布局的一种实现, 不用键位 E00 而选用键位 C12, E13
    [Kc1 Kc2 Kc3 Kc4 Kc5 Kc6 Kc7 Kc8 Kc9 Kc0 Minus Equal Grave],  // E01 - E13
    [Q W E R T Y U I O P LeftBracket RightBracket],               // D01 - D12
    [A S D F G H J K L Semicolon Quote Backslash],                // C01 - C12
    [Z X C V B N M Comma Dot Slash],                              // B01 - B10
    [Space]                                                       // A03
];

const 字母键盘矩阵: 键盘矩阵<'static> =
    矩阵![
    [],                             // E01
    [Q W E R T Y U I O P],          // D01 - D10
    [A S D F G H J K L Semicolon],  // C01 - C10
    [Z X C V B N M Comma Dot],      // B01 - B09
    [Space]                         // A03
];

const 正交直列矩阵: 键盘矩阵<'static> =
    矩阵![
    [],                               // E01
    [Q W E R T Y U I O P],            // D01 - D10
    [A S D F G H J K L Semicolon],    // C01 - C10
    [Z X C V B N M Comma Dot Slash],  // B01 - B10
    [Space]                           // A03
];

const 直列分体矩阵: 键盘矩阵<'static> =
    矩阵![
    [],                               // E01
    [Q W E R T Y U I O P],            // D01 - D10
    [A S D F G H J K L Semicolon],    // C01 - C10
    [Z X C V B N M Comma Dot Slash],  // B01 - B10
    [No Space Space]                  // A03 - A05
];

const 正交直列带数字行矩阵: 键盘矩阵<'static> =
    矩阵![
    [Kc1 Kc2 Kc3 Kc4 Kc5 Kc6 Kc7 Kc8 Kc9 Kc0],  // E01 - E10
    [Q W E R T Y U I O P],                      // D01 - D10
    [A S D F G H J K L Semicolon],              // C01 - C10
    [Z X C V B N M Comma Dot Slash],            // B01 - B10
    [Space]                                     // A03
];

const 直列分体带数字行矩阵: 键盘矩阵<'static> =
    矩阵![
    [Kc1 Kc2 Kc3 Kc4 Kc5 Kc6 Kc7 Kc8 Kc9 Kc0],  // E01 - E10
    [Q W E R T Y U I O P],                      // D01 - D10
    [A S D F G H J K L Semicolon],              // C01 - C10
    [Z X C V B N M Comma Dot Slash],            // B01 - B10
    [No Space Space]                            // A03 - A05
];

#[macro_export]
macro_rules! 盘面 {
    [
        $(
            [$($键:tt)*]
            $(,)?
        )*
    ] => {
        &[ $( &[ $( 键面!($键), )* ], )* ]
    };
}

#[macro_export]
macro_rules! 键面 {
    (_) => {
        键面刻印::透明
    };
    (空) => {
        键面刻印::无刻
    };
    ($字符:literal) => {
        键面刻印::有刻($crate::gear::layout::刻印说明::居中($字符))
    };
    ($字母:ident) => {
        键面刻印::有刻($crate::gear::layout::刻印说明::居中(
            stringify!($字母),
        ))
    };
    ({ 中: $居中:tt, 上: $居上:tt, 下: $居下:tt, 左: $居左:tt, 右: $居右:tt }) => {
        键面刻印::有刻($crate::gear::layout::刻印说明 {
            中: $crate::标注!($居中),
            上: $crate::标注!($居上),
            下: $crate::标注!($居下),
            左: $crate::标注!($居左),
            右: $crate::标注!($居右),
        })
    };
}

#[macro_export]
macro_rules! 标注 {
    (_) => {
        None
    };
    ($字符:literal) => {
        Some($字符)
    };
    ($字符:ident) => {
        Some(stringify!($字符))
    };
}

pub const 基本盘面: 盘面定义<'static> =
    盘面![
    [ "1" "2" "3" "4" "5" "6" "7" "8" "9" "0" "-" "=" "`" ],  // E01 - E13
    [ q w e r t y u i o p "[" "]" ],                          // D01 - D12
    [ a s d f g h j k l ";" "'" "\\" ],                       // C01 - C12
    [ z x c v b n m "," "." "/" ],                            // B01 - B10
    [ "␣" "␣" "␣" ]                                           // A03 - A05
];

pub const 上档盘面: 盘面定义<'static> =
    盘面![
    [ "!" "@" "#" "$" "%" "^" "&" "*" "(" ")" "_" "+" "~" ],
    [ Q W E R T Y U I O P "{" "}" ],
    [ A S D F G H J K L ":" "\"" "|" ],
    [ Z X C V B N M "<" ">" "?" ],
    [ _ _ _ ]
];

pub const 大写字母盘面: 盘面定义<'static> =
    盘面![
    [ _ _ _ _ _ _ _ _ _ _ _ _ _ _ ],
    [ Q W E R T Y U I O P _ _ ],
    [ A S D F G H J K L _ _ _ ],
    [ Z X C V B N M _ _ _ ],
    [ "␣" "␣" "␣" ]
];

pub const 拉丁字母键盘布局: 键盘布局 = 键盘布局 {
    盘面: &[基本盘面, 上档盘面, 大写字母盘面],
    默认盘面: 盘面序号(2),
    默认配列: 键盘配列::字母键盘,
};

pub mod 功能键 {
    use super::*;

    pub const 退出键: 键面映射 = 键面映射 {
        键码: KeyCode::Escape,
        刻印: 键面刻印::有刻(刻印说明::居中("退出")),
    };
    pub const 制表键: 键面映射 = 键面映射 {
        键码: KeyCode::Tab,
        刻印: 键面刻印::有刻(刻印说明::居中("制表")),
    };
    pub const 退格键: 键面映射 = 键面映射 {
        键码: KeyCode::Backspace,
        刻印: 键面刻印::有刻(刻印说明::居中("退格")),
    };
    pub const 回车键: 键面映射 = 键面映射 {
        键码: KeyCode::Enter,
        刻印: 键面刻印::有刻(刻印说明::居中("回车")),
    };

    pub const 众功能键: &[键面映射] = &[退出键, 制表键, 退格键, 回车键];
}

#[derive(Clone)]
pub struct 键盘配列输出信号 {
    pub 已选配列: ReadSignal<键盘配列>,
    pub 选用配列: WriteSignal<键盘配列>,
}

pub fn 配列机关(方案: &输入方案输出信号) -> 键盘配列输出信号 {
    let 方案定义 = 方案.方案定义;
    let 初始方案 = 方案定义.get_untracked();
    let (已选配列, 选用配列) = signal(初始方案.布局.默认配列);

    let _ = Effect::watch(
        方案定义,
        move |&方案, _, _| {
            选用配列(方案.布局.默认配列);
        },
        false
    );

    键盘配列输出信号 {
        已选配列,
        选用配列,
    }
}
