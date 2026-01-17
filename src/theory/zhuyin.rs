use lazy_regex::{regex, Regex};
use lazy_static::lazy_static;

use crate::definition::{
    碼表格式, 觸鍵方式, 輸入方案定義, 轉寫法定義, 邊界判定規則, 鍵位定義
};
use crate::gear::layout::{
    上檔盤面, 基本盤面, 盤面定義, 盤面選擇碼, 配列, 鍵盤佈局, 鍵面刻印
};
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
    ($輸入碼: expr => $鍵碼: path) => {
        鍵位定義 {
            輸入碼: $輸入碼,
            盤面: 盤面選擇碼(0),
            鍵碼: $鍵碼,
        }
    };
}

const 注音聲母: &[鍵位定義] = &[
    鍵位!(ㄅ => KeyCode::Kc1),
    鍵位!(ㄆ => KeyCode::Q),
    鍵位!(ㄇ => KeyCode::A),
    鍵位!(ㄈ => KeyCode::Z),
    鍵位!(ㄉ => KeyCode::Kc2),
    鍵位!(ㄊ => KeyCode::W),
    鍵位!(ㄋ => KeyCode::S),
    鍵位!(ㄌ => KeyCode::X),
    鍵位!(ㄍ => KeyCode::E),
    鍵位!(ㄎ => KeyCode::D),
    鍵位!(ㄏ => KeyCode::C),
    鍵位!(ㄐ => KeyCode::R),
    鍵位!(ㄑ => KeyCode::F),
    鍵位!(ㄒ => KeyCode::V),
    鍵位!(ㄓ => KeyCode::Kc5),
    鍵位!(ㄔ => KeyCode::T),
    鍵位!(ㄕ => KeyCode::G),
    鍵位!(ㄖ => KeyCode::B),
    鍵位!(ㄗ => KeyCode::Y),
    鍵位!(ㄘ => KeyCode::H),
    鍵位!(ㄙ => KeyCode::N),
];
const 注音韻母: &[鍵位定義] = &[
    鍵位!(ㄧ => KeyCode::U),
    鍵位!(ㄨ => KeyCode::J),
    鍵位!(ㄩ => KeyCode::M),
    鍵位!(ㄚ => KeyCode::Kc8),
    鍵位!(ㄛ => KeyCode::I),
    鍵位!(ㄜ => KeyCode::K),
    鍵位!(ㄝ => KeyCode::Comma),
    鍵位!(ㄞ => KeyCode::Kc9),
    鍵位!(ㄟ => KeyCode::O),
    鍵位!(ㄠ => KeyCode::L),
    鍵位!(ㄡ => KeyCode::Dot),
    鍵位!(ㄢ => KeyCode::Kc0),
    鍵位!(ㄣ => KeyCode::P),
    鍵位!(ㄤ => KeyCode::Semicolon),
    鍵位!(ㄥ => KeyCode::Slash),
    鍵位!(ㄦ => KeyCode::Minus),
];

const 聲調符號: &[鍵位定義] = &[
    鍵位!(ˉ => KeyCode::Space),
    鍵位!(ˊ => KeyCode::Kc6),
    鍵位!(ˇ => KeyCode::Kc3),
    鍵位!(ˋ => KeyCode::Kc4),
    鍵位!("˙" => KeyCode::Kc7),
];

lazy_static! {
    static ref 字根表: Vec<鍵位定義<'static>> = [
        注音聲母,
        注音韻母,
        聲調符號,
    ].concat();

    static ref 注音拼式: Box<[&'static Regex]> = Box::new([
        regex!("^[ㄅㄆㄇㄈㄉㄊㄋㄌㄍㄎㄏㄐㄑㄒㄓㄔㄕㄖㄗㄘㄙ]?[ㄧㄨㄩ]?[ㄚㄛㄜㄝㄞㄟㄠㄡㄢㄣㄤㄥㄦ]?[ˉˊˇˋ˙]?$").deref(),
    ]);
}

const 注音盤面: 盤面定義<'static> = 盤面! {
    [ ㄅ ㄉ ˇ  ˋ  ㄓ ˊ  "˙" ㄚ ㄞ ㄢ ㄦ _ _ ],
    [ ㄆ ㄊ ㄍ ㄐ ㄔ ㄗ ㄧ  ㄛ ㄟ ㄣ _ _ ],
    [ ㄇ ㄋ ㄎ ㄑ ㄕ ㄘ ㄨ  ㄜ ㄠ ㄤ _ _ ],
    [ ㄈ ㄌ ㄏ ㄒ ㄖ ㄙ ㄩ  ㄝ ㄡ ㄥ ],
    [ ˉ ˉ ˉ ]
};

const 大千注音鍵盤佈局: 鍵盤佈局 = 鍵盤佈局 {
    盤面: &[基本盤面, 上檔盤面, 注音盤面],
    默認盤面: 盤面選擇碼(2),
    首選配列: 配列::主鍵盤區,
};

pub fn 注音輸入方案() -> 輸入方案定義<'static> {
    輸入方案定義 {
        名稱: "注音",
        佈局: &大千注音鍵盤佈局,
        指法: 觸鍵方式::連擊,
        編碼法: 碼表格式::連擊,
        字根表: &字根表,
        轉寫法: 轉寫法定義 {
            輸入碼表示: &[],
            輸入碼鍵位: &[],
            拼式轉寫規則: &[],
            字根拆分規則: &[],
            拼式驗證規則: &注音拼式,
            邊界判定: 邊界判定規則 {
                分隔鍵: &[],
                起始鍵: 注音聲母,
                終止鍵: 聲調符號,
            },
        },
        動態切換: &[],
    }
}
