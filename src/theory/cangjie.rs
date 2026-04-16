use lazy_regex::{regex, Regex};
use lazy_static::lazy_static;

use crate::definition::{
    碼表格式, 觸鍵方式, 輸入方案定義, 轉寫法定義, 邊界判定規則, 鍵位定義
};
use crate::gear::layout::{
    上檔盤面, 基本盤面, 盤面定義, 盤面選擇碼, 配列, 鍵盤佈局, 鍵面刻印
};
use crate::gear::theory::輸入方案環境;
use crate::key_code::KeyCode;
use crate::{盤面, 鍵面};

macro_rules! 鍵位 {
    ($輸入碼: ident => $鍵碼: path) => {
        鍵位定義 {
            輸入碼: stringify!($輸入碼),
            盤面: 盤面選擇碼(0),
            鍵碼: $鍵碼,
        }
    };
}

const 倉頡字母: &[鍵位定義] = &[
    鍵位!(日 => KeyCode::A),
    鍵位!(月 => KeyCode::B),
    鍵位!(金 => KeyCode::C),
    鍵位!(木 => KeyCode::D),
    鍵位!(水 => KeyCode::E),
    鍵位!(火 => KeyCode::F),
    鍵位!(土 => KeyCode::G),
    鍵位!(竹 => KeyCode::H),
    鍵位!(戈 => KeyCode::I),
    鍵位!(十 => KeyCode::J),
    鍵位!(大 => KeyCode::K),
    鍵位!(中 => KeyCode::L),
    鍵位!(一 => KeyCode::M),
    鍵位!(弓 => KeyCode::N),
    鍵位!(人 => KeyCode::O),
    鍵位!(心 => KeyCode::P),
    鍵位!(手 => KeyCode::Q),
    鍵位!(口 => KeyCode::R),
    鍵位!(尸 => KeyCode::S),
    鍵位!(廿 => KeyCode::T),
    鍵位!(山 => KeyCode::U),
    鍵位!(女 => KeyCode::V),
    鍵位!(田 => KeyCode::W),
    鍵位!(卜 => KeyCode::Y),
    鍵位!(難 => KeyCode::X),
];

lazy_static! {
    static ref 倉頡拼式: Box<[&'static Regex]> =
        Box::new([regex!("^[日月金木水火土竹戈十大中一弓人心手口尸廿山女田卜難]+$").deref(),]);
}

const 倉頡盤面: 盤面定義<'static> = 盤面! {
    [ _ _ _ _ _ _ _ _ _ _ _ _ _ ],
    [ 手 田 水 口 廿 卜 山 戈 人 心 _ _ ],
    [ 日 尸 木 火 土 竹 十 大 中 _ _ _ ],
    [ _ 難 金 女 月 弓 一 _ _ _ ],
    [ ˉ ˉ ˉ ]
};

const 倉頡鍵盤佈局: 鍵盤佈局 = 鍵盤佈局 {
    盤面: &[基本盤面, 上檔盤面, 倉頡盤面],
    默認盤面: 盤面選擇碼(2),
    首選配列: 配列::字母鍵盤,
};

pub fn 倉頡輸入方案(_環境: 輸入方案環境) -> 輸入方案定義<'static> {
    輸入方案定義 {
        名稱: "倉頡",
        佈局: &倉頡鍵盤佈局,
        指法: 觸鍵方式::連擊,
        編碼法: 碼表格式::連擊,
        字根表: 倉頡字母,
        轉寫法: 轉寫法定義 {
            輸入碼表示: &[],
            輸入碼鍵位: &[],
            拼式轉寫規則: &[],
            字根拆分規則: &[],
            拼式驗證規則: &倉頡拼式,
            邊界判定: 邊界判定規則 {
                分隔鍵: &[],
                起始鍵: &[],
                終止鍵: &[],
            },
        },
        動態切換: &[],
    }
}
