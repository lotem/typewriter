use keyberon::key_code::KeyCode;

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

#[derive(Clone, Copy, PartialEq)]
pub enum 鍵面刻印<'a> {
    透明,
    無刻,
    有刻(&'a str),
}

impl<'a> 鍵面刻印<'a> {
    pub fn 刻印文字(&self) -> Option<&'a str> {
        match self {
            鍵面刻印::有刻(文字) => Some(文字),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct 鍵面映射<'a> {
    pub 鍵碼: KeyCode,
    pub 刻印: 鍵面刻印<'a>,
}

pub type 鍵盤矩陣<'a> = &'a [&'a [KeyCode]];
pub type 盤面定義<'a> = &'a [&'a [鍵面刻印<'a>]];

pub struct 鍵盤佈局<'a> {
    pub 矩陣: 鍵盤矩陣<'a>,
    盤面: &'a [盤面定義<'a>],
}

impl 鍵盤佈局<'_> {
    pub fn 鍵的行列座標(&self, 目標鍵碼: KeyCode) -> Option<(usize, usize)> {
        self.矩陣.iter().enumerate().find_map(|(行號, 行)| {
            行.iter().enumerate().find_map(|(列號, 鍵)| {
                if *鍵 == 目標鍵碼 {
                    Some((行號, 列號))
                } else {
                    None
                }
            })
        })
    }

    pub fn 選擇盤面(
        &self,
        鍵碼: KeyCode,
        目標盤面: 盤面選擇碼,
    ) -> Option<(usize, 鍵面刻印)> {
        self.盤面
            .iter()
            .enumerate()
            .rev()
            .filter(|&(盤面號, _)| 目標盤面.是否可選盤面(盤面號))
            .find_map(|(盤面號, 此盤面)| {
                self.從盤面讀取刻印(此盤面, 鍵碼)
                    .and_then(|刻印| match 刻印 {
                        鍵面刻印::透明 => None,
                        _ => Some((盤面號, 刻印)),
                    })
            })
    }

    fn 從盤面讀取刻印<'a>(
        &self,
        此盤面: 盤面定義<'a>,
        鍵碼: KeyCode,
    ) -> Option<鍵面刻印<'a>> {
        let (行, 列) = self.鍵的行列座標(鍵碼)?;
        此盤面.get(行).and_then(|此行| 此行.get(列)).copied()
    }
}

macro_rules! 矩陣 {
    [ $( [ $( $鍵:ident )* ] $(,)? )* ] => {
        &[ $( &[ $( keyberon::key_code::KeyCode::$鍵, )* ], )* ]
    };
}

macro_rules! 盤面 {
    [ $( [ $( $鍵:tt )* ] $(,)? )* ] => {
        &[ $( &[ $( 鍵面!($鍵), )* ], )* ]
    };
}

macro_rules! 鍵面 {
    ( _ ) => {
        鍵面刻印::透明
    };
    ( 空 ) => {
        鍵面刻印::無刻
    };
    ( $字符:literal ) => {
        鍵面刻印::有刻($字符)
    };
    ( $字母:ident ) => {
        鍵面刻印::有刻(stringify!($字母))
    };
}

const 基本盤面: 盤面定義<'static> = 盤面![
    [ q w e r t y u i o p ],
    [ a s d f g h j k l ";" ],
    [ z x c v b n m "," "." "/" ],
    [ "␣" ]
];

const 上檔盤面: 盤面定義<'static> = 盤面![
    [ Q W E R T Y U I O P ],
    [ A S D F G H J K L ":" ],
    [ Z X C V B N M "<" ">" "?" ],
    [ _ ]
];

const 大寫字母盤面: 盤面定義<'static> = 盤面![
    [ Q W E R T Y U I O P ],
    [ A S D F G H J K L _ ],
    [ Z X C V B N M _ _ _ ],
    [ _ ]
];

const 宮保拼音盤面: 盤面定義<'static> = 盤面![
    [ 空 C L D T 空 U R O 空 ],
    [ 空 S H G K 空 I N E _ ],
    [ 空 Z F B P 空 Ü _ _ _ ],
    [ A ]
];

pub const 打字機鍵盤佈局: 鍵盤佈局<'static> = 鍵盤佈局 {
    矩陣: 矩陣![
        [Q W E R T Y U I O P],
        [A S D F G H J K L SColon],
        [Z X C V B N M Comma Dot Slash],
        [Space]
    ],
    盤面: &[基本盤面, 上檔盤面, 大寫字母盤面, 宮保拼音盤面],
};

pub mod 功能鍵 {
    use super::*;

    pub const 退出鍵: 鍵面映射 = 鍵面映射 {
        鍵碼: KeyCode::Escape,
        刻印: 鍵面刻印::有刻("退出"),
    };
    pub const 製表鍵: 鍵面映射 = 鍵面映射 {
        鍵碼: KeyCode::Tab,
        刻印: 鍵面刻印::有刻("製表"),
    };
    pub const 退格鍵: 鍵面映射 = 鍵面映射 {
        鍵碼: KeyCode::BSpace,
        刻印: 鍵面刻印::有刻("退格"),
    };
    pub const 回車鍵: 鍵面映射 = 鍵面映射 {
        鍵碼: KeyCode::Enter,
        刻印: 鍵面刻印::有刻("回車"),
    };
}
