use keyberon::key_code::KeyCode;
use lazy_regex::{regex, Regex};
use lazy_static::lazy_static;

use crate::definition::{觸鍵方式, 輸入方案定義, 轉寫法定義, 鍵位定義};
use crate::layout::盤面選擇碼;
use crate::spelling_algebra::拼寫運算;
use crate::轉寫;

macro_rules! 字母鍵 {
    ($字母: ident) => {
        鍵位定義 {
            輸入碼: stringify!($字母),
            鍵碼: KeyCode::$字母,
        }
    };
}

const 字母表: &[鍵位定義] = &[
    字母鍵!(A),
    字母鍵!(B),
    字母鍵!(C),
    字母鍵!(D),
    字母鍵!(E),
    字母鍵!(F),
    字母鍵!(G),
    字母鍵!(H),
    字母鍵!(I),
    字母鍵!(J),
    字母鍵!(K),
    字母鍵!(L),
    字母鍵!(M),
    字母鍵!(N),
    字母鍵!(O),
    字母鍵!(P),
    字母鍵!(Q),
    字母鍵!(R),
    字母鍵!(S),
    字母鍵!(T),
    字母鍵!(U),
    字母鍵!(V),
    字母鍵!(W),
    字母鍵!(X),
    字母鍵!(Y),
    字母鍵!(Z),
    鍵位定義 {
        輸入碼: "␣",
        鍵碼: KeyCode::Space,
    },
    鍵位定義 {
        輸入碼: "'",
        鍵碼: KeyCode::Quote,
    },
    鍵位定義 {
        輸入碼: "-",
        鍵碼: KeyCode::Minus,
    },
];

lazy_static! {
    static ref 字母轉鍵位: Box<[拼寫運算<'static>]> = Box::new([轉寫!(
        "abcdefghijklmnopqrstuvwxyz ",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ␣"
    ),]);
    static ref 驗證拉丁文: Box<[&'static Regex]> = Box::new([regex!("^([-A-Za-z '])+$").deref(),]);
}

pub fn 拉丁字母輸入方案() -> 輸入方案定義<'static> {
    輸入方案定義 {
        名稱: "拉丁字母",
        盤面: 盤面選擇碼(2),
        指法: 觸鍵方式::連擊,
        字根表: 字母表,
        轉寫法: 轉寫法定義 {
            拼式轉寫規則: &[],
            字根拆分規則: &字母轉鍵位,
            拼式驗證規則: &驗證拉丁文,
        },
    }
}
