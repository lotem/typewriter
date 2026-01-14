//! 本節用到的概念
//! - 配列 :: 描述物理按鍵的數目和空間排佈
//! - 矩陣 :: 定義按鍵邏輯上的行列座標, 採用 ISO/IEC 9995-1 的鍵位標註法
//! - 鍵盤佈局 :: 各個盤面及各個位置上的字符定義
//! - 盤面 :: 也稱層, 定義在盤面選擇碼指定的狀態下各鍵碼映射到哪些字符
//! - 鍵面刻印 :: 鍵面顯示的文字

use leptos::prelude::*;
use strum::{Display, EnumIter};

use crate::gear::theory::輸入方案機關輸出信號;
use crate::key_code::KeyCode;

#[derive(Clone, Copy, Default)]
pub struct 盤面選擇碼(pub u64);

impl 盤面選擇碼 {
    pub fn 是否可選盤面(&self, 盤面號: usize) -> bool {
        盤面號 == 0 || (self.0 & (1 << (盤面號 - 1))) != 0
    }

    pub fn 頂層盤面(&self) -> usize {
        (0..64)
            .rfind(|&盤面號| self.是否可選盤面(盤面號))
            .unwrap_or_default()
    }
}

#[derive(Clone, Copy)]
pub struct 矩陣座標(pub usize, pub usize);

/// 矩陣的行列座標按照 ISO/IEC 9995-1 的鍵位標註法顯示.
/// 空格至數字行從下到上編號爲 A 到 E.
/// 本品只做字母數字區, 因此 A 行列號從 03 開始, 其他各行從 01 開始.
impl std::fmt::Display for 矩陣座標 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.0 {
            // (0, 0) -> E01
            i @ 0..=3 => {
                let 行號 = char::from_u32('E' as u32 - i as u32).ok_or(std::fmt::Error {})?;
                let 列號 = self.1 + 1;
                write!(f, "{行號}{列號}")
            }
            // (4, 0) -> A03
            4 => {
                let 行號 = 'A';
                let 列號 = self.1 + 3;
                write!(f, "{行號}{列號}")
            }
            _ => Err(std::fmt::Error {}),
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct 刻印說明 {
    pub 中: Option<&'static str>,
    pub 上: Option<&'static str>,
    pub 下: Option<&'static str>,
    pub 左: Option<&'static str>,
    pub 右: Option<&'static str>,
}

impl 刻印說明 {
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
pub enum 鍵面刻印 {
    透明,
    無刻,
    有刻(刻印說明),
}

impl 鍵面刻印 {
    pub fn 居中刻印文字(&self) -> Option<&'static str> {
        match self {
            鍵面刻印::有刻(刻印說明 {
                中: 有冇刻印, ..
            }) => *有冇刻印,
            _ => None,
        }
    }

    pub fn 上方刻印文字(&self) -> Option<&'static str> {
        match self {
            鍵面刻印::有刻(刻印說明 {
                上: 有冇刻印, ..
            }) => *有冇刻印,
            _ => None,
        }
    }

    pub fn 下方刻印文字(&self) -> Option<&'static str> {
        match self {
            鍵面刻印::有刻(刻印說明 {
                下: 有冇刻印, ..
            }) => *有冇刻印,
            _ => None,
        }
    }

    pub fn 左側刻印文字(&self) -> Option<&'static str> {
        match self {
            鍵面刻印::有刻(刻印說明 {
                左: 有冇刻印, ..
            }) => *有冇刻印,
            _ => None,
        }
    }

    pub fn 右側刻印文字(&self) -> Option<&'static str> {
        match self {
            鍵面刻印::有刻(刻印說明 {
                右: 有冇刻印, ..
            }) => *有冇刻印,
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct 鍵面映射 {
    pub 鍵碼: KeyCode,
    pub 刻印: 鍵面刻印,
}

pub type 鍵盤矩陣<'a> = &'a [&'a [KeyCode]];
pub type 盤面定義<'a> = &'a [&'a [鍵面刻印]];

#[derive(Clone, Copy, Default, Display, EnumIter, PartialEq)]
pub enum 配列 {
    #[default]
    主鍵盤區,
    字母鍵盤,
    正交直列,
    直列分體,
    正交直列帶數字行,
    直列分體帶數字行,
}

impl 配列 {
    pub fn 橫向交錯(&self) -> bool {
        matches!(self, 配列::主鍵盤區 | 配列::字母鍵盤)
    }
    pub fn 直列(&self) -> bool {
        matches!(
            self,
            配列::正交直列
                | 配列::直列分體
                | 配列::正交直列帶數字行
                | 配列::直列分體帶數字行
        )
    }
    pub fn 分體(&self) -> bool {
        matches!(self, 配列::直列分體 | 配列::直列分體帶數字行)
    }
    pub fn 規格(&self) -> usize {
        match self {
            配列::主鍵盤區 => 60,
            配列::字母鍵盤 => 30,
            配列::正交直列 | 配列::直列分體 => 30,
            配列::正交直列帶數字行 | 配列::直列分體帶數字行 => 60,
        }
    }
    pub fn 矩陣(&self) -> 鍵盤矩陣<'static> {
        match self {
            配列::主鍵盤區 => 主鍵盤區矩陣,
            配列::字母鍵盤 => 字母鍵盤矩陣,
            配列::正交直列 => 正交直列矩陣,
            配列::直列分體 => 直列分體矩陣,
            配列::正交直列帶數字行 => 正交直列帶數字行矩陣,
            配列::直列分體帶數字行 => 直列分體帶數字行矩陣,
        }
    }
}

#[derive(Clone, Copy)]
pub struct 鍵盤佈局 {
    pub 盤面: &'static [盤面定義<'static>],
    pub 默認盤面: 盤面選擇碼,
    pub 首選配列: 配列,
}

impl 鍵盤佈局 {
    pub fn 選擇盤面(
        &self,
        目標盤面: 盤面選擇碼,
        座標: 矩陣座標,
    ) -> Option<(usize, 鍵面刻印)> {
        self.盤面
            .iter()
            .enumerate()
            .rev()
            .filter(|&(盤面號, _)| 目標盤面.是否可選盤面(盤面號))
            .find_map(|(盤面號, 此盤面)| {
                self.從盤面讀取刻印(此盤面, 座標)
                    .and_then(|刻印| match 刻印 {
                        鍵面刻印::透明 => None,
                        _ => Some((盤面號, 刻印)),
                    })
            })
    }

    fn 從盤面讀取刻印(
        &self,
        此盤面: 盤面定義<'static>,
        座標: 矩陣座標,
    ) -> Option<鍵面刻印> {
        let 矩陣座標(行, 列) = 座標;
        此盤面.get(行).and_then(|此行| 此行.get(列)).copied()
    }
}

macro_rules! 矩陣 {
    [ $( [ $( $鍵:ident )* ] $(,)? )* ] => {
        &[ $( &[ $( crate::key_code::KeyCode::$鍵, )* ], )* ]
    };
}

const 主鍵盤區矩陣: 鍵盤矩陣<'static> = 矩陣![
    // ISO/IEC 9995-2 規定字母數字區至少包含 47 個用於輸入字符的鍵位
    // 以下是協調 48 文字鍵盤佈局的一種實現, 不用鍵位 E00 而選用鍵位 C12, E13
    [Kc1 Kc2 Kc3 Kc4 Kc5 Kc6 Kc7 Kc8 Kc9 Kc0 Minus Equal Grave],  // E01 - E13
    [Q W E R T Y U I O P LeftBracket RightBracket],               // D01 - D12
    [A S D F G H J K L Semicolon Quote Backslash],                // C01 - C12
    [Z X C V B N M Comma Dot Slash],                              // B01 - B10
    [Space]                                                       // A03
];

const 字母鍵盤矩陣: 鍵盤矩陣<'static> = 矩陣![
    [],                             // E01
    [Q W E R T Y U I O P],          // D01 - D10
    [A S D F G H J K L Semicolon],  // C01 - C10
    [Z X C V B N M Comma Dot],      // B01 - B09
    [Space]                         // A03
];

const 正交直列矩陣: 鍵盤矩陣<'static> = 矩陣![
    [],                               // E01
    [Q W E R T Y U I O P],            // D01 - D10
    [A S D F G H J K L Semicolon],    // C01 - C10
    [Z X C V B N M Comma Dot Slash],  // B01 - B10
    [Space]                           // A03
];

const 直列分體矩陣: 鍵盤矩陣<'static> = 矩陣![
    [],                               // E01
    [Q W E R T Y U I O P],            // D01 - D10
    [A S D F G H J K L Semicolon],    // C01 - C10
    [Z X C V B N M Comma Dot Slash],  // B01 - B10
    [No Space Space]                  // A03 - A05
];

const 正交直列帶數字行矩陣: 鍵盤矩陣<'static> = 矩陣![
    [Kc1 Kc2 Kc3 Kc4 Kc5 Kc6 Kc7 Kc8 Kc9 Kc0],  // E01 - E10
    [Q W E R T Y U I O P],                      // D01 - D10
    [A S D F G H J K L Semicolon],              // C01 - C10
    [Z X C V B N M Comma Dot Slash],            // B01 - B10
    [Space]                                     // A03
];

const 直列分體帶數字行矩陣: 鍵盤矩陣<'static> = 矩陣![
    [Kc1 Kc2 Kc3 Kc4 Kc5 Kc6 Kc7 Kc8 Kc9 Kc0],  // E01 - E10
    [Q W E R T Y U I O P],                      // D01 - D10
    [A S D F G H J K L Semicolon],              // C01 - C10
    [Z X C V B N M Comma Dot Slash],            // B01 - B10
    [No Space Space]                            // A03 - A05
];

#[macro_export]
macro_rules! 盤面 {
    [ $( [ $( $鍵:tt )* ] $(,)? )* ] => {
        &[ $( &[ $( 鍵面!($鍵), )* ], )* ]
    };
}

#[macro_export]
macro_rules! 鍵面 {
    ( _ ) => {
        鍵面刻印::透明
    };
    ( 空 ) => {
        鍵面刻印::無刻
    };
    ( $字符:literal ) => {
        鍵面刻印::有刻($crate::gear::layout::刻印說明::居中($字符))
    };
    ( $字母:ident ) => {
        鍵面刻印::有刻($crate::gear::layout::刻印說明::居中(
            stringify!($字母),
        ))
    };
    ( { 中: $居中:tt, 上: $居上:tt, 下: $居下:tt, 左: $居左:tt, 右: $居右:tt } ) => {
        鍵面刻印::有刻($crate::gear::layout::刻印說明 {
            中: $crate::標註!($居中),
            上: $crate::標註!($居上),
            下: $crate::標註!($居下),
            左: $crate::標註!($居左),
            右: $crate::標註!($居右),
        })
    };
}

#[macro_export]
macro_rules! 標註 {
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

pub const 基本盤面: 盤面定義<'static> = 盤面![
    [ "1" "2" "3" "4" "5" "6" "7" "8" "9" "0" "-" "=" "`" ],  // E01 - E13
    [ q w e r t y u i o p "[" "]" ],                          // D01 - D12
    [ a s d f g h j k l ";" "'" "\\" ],                       // C01 - C12
    [ z x c v b n m "," "." "/" ],                            // B01 - B10
    [ "␣" "␣" "␣" ]                                           // A03 - A05
];

pub const 上檔盤面: 盤面定義<'static> = 盤面![
    [ "!" "@" "#" "$" "%" "^" "&" "*" "(" ")" "_" "+" "~" ],
    [ Q W E R T Y U I O P "{" "}" ],
    [ A S D F G H J K L ":" "\"" "|" ],
    [ Z X C V B N M "<" ">" "?" ],
    [ _ _ _ ]
];

pub const 大寫字母盤面: 盤面定義<'static> = 盤面![
    [ _ _ _ _ _ _ _ _ _ _ _ _ _ _ ],
    [ Q W E R T Y U I O P _ _ ],
    [ A S D F G H J K L _ _ _ ],
    [ Z X C V B N M _ _ _ ],
    [ "␣" "␣" "␣" ]
];

pub const 拉丁字母鍵盤佈局: 鍵盤佈局 = 鍵盤佈局 {
    盤面: &[基本盤面, 上檔盤面, 大寫字母盤面],
    默認盤面: 盤面選擇碼(2),
    首選配列: 配列::字母鍵盤,
};

pub mod 功能鍵 {
    use super::*;

    pub const 退出鍵: 鍵面映射 = 鍵面映射 {
        鍵碼: KeyCode::Escape,
        刻印: 鍵面刻印::有刻(刻印說明::居中("退出")),
    };
    pub const 製表鍵: 鍵面映射 = 鍵面映射 {
        鍵碼: KeyCode::Tab,
        刻印: 鍵面刻印::有刻(刻印說明::居中("製表")),
    };
    pub const 退格鍵: 鍵面映射 = 鍵面映射 {
        鍵碼: KeyCode::Backspace,
        刻印: 鍵面刻印::有刻(刻印說明::居中("退格")),
    };
    pub const 回車鍵: 鍵面映射 = 鍵面映射 {
        鍵碼: KeyCode::Enter,
        刻印: 鍵面刻印::有刻(刻印說明::居中("回車")),
    };

    pub const 衆功能鍵: &[鍵面映射] = &[退出鍵, 製表鍵, 退格鍵, 回車鍵];
}

#[derive(Clone)]
pub struct 佈局機關輸出信號 {
    pub 已選配列: ReadSignal<配列>,
    pub 選用配列: WriteSignal<配列>,
    pub 當選盤面: ReadSignal<盤面選擇碼>,
    pub 選擇盤面: WriteSignal<盤面選擇碼>,
}

pub fn 佈局機關(方案: &輸入方案機關輸出信號) -> 佈局機關輸出信號 {
    let 方案定義 = 方案.方案定義;
    let 初始方案 = 方案定義.get_untracked();
    let (已選配列, 選用配列) = signal(初始方案.佈局.首選配列);
    let (當選盤面, 選擇盤面) = signal(初始方案.佈局.默認盤面);

    let _ = Effect::watch(
        方案定義,
        move |&方案, _, _| {
            選用配列(方案.佈局.首選配列);
            選擇盤面(方案.佈局.默認盤面);
        },
        false,
    );

    佈局機關輸出信號 {
        已選配列,
        選用配列,
        當選盤面,
        選擇盤面,
    }
}
