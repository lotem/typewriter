use keyberon::key_code::KeyCode;
use lazy_regex::Regex;

use crate::chord::鍵組;
use crate::layout::盤面選擇碼;
use crate::spelling_algebra::{拼寫運算, 施展拼寫運算};

pub struct 鍵的定義<'a> {
    pub 輸入碼: &'a str,
    pub 鍵碼: KeyCode,
}

#[derive(Clone, Copy)]
pub struct 輸入方案定義<'a> {
    pub 名稱: &'a str,
    pub 盤面: 盤面選擇碼,
    pub 指法: 觸鍵方式,
    pub 字根表: &'a [鍵的定義<'a>],
    pub 轉寫法: 轉寫法定義<'a>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum 觸鍵方式 {
    連擊,
    並擊,
}

#[derive(Clone, Copy)]
pub struct 轉寫法定義<'a> {
    pub 拼式轉寫規則: &'a [拼寫運算<'a>],
    pub 字根拆分規則: &'a [拼寫運算<'a>],
    pub 拼式驗證規則: &'a [&'a Regex],
}

pub trait 判定鍵位 {
    fn 有無鍵位(&self) -> bool;
    fn 包含鍵位(&self, 鍵碼: &KeyCode) -> bool;
}

impl 判定鍵位 for &鍵組 {
    fn 有無鍵位(&self) -> bool {
        !self.0.is_empty()
    }

    fn 包含鍵位(&self, 鍵碼: &KeyCode) -> bool {
        self.0.contains(鍵碼)
    }
}

impl 判定鍵位 for KeyCode {
    fn 有無鍵位(&self) -> bool {
        *self != KeyCode::No
    }

    fn 包含鍵位(&self, 鍵碼: &KeyCode) -> bool {
        self == 鍵碼
    }
}

impl 輸入方案定義<'_> {
    pub fn 尋得字根(&self, 字根: &str) -> Option<&鍵的定義> {
        self.字根表.iter().find(|鍵| 鍵.輸入碼 == 字根)
    }

    pub fn 讀出鍵位(&self, 字根碼: &str) -> 鍵組 {
        鍵組(
            self.字根表
                .iter()
                .filter(|鍵| 字根碼.contains(鍵.輸入碼))
                .map(|鍵| 鍵.鍵碼)
                .collect(),
        )
    }

    pub fn 寫成字根碼(&self, 鍵位: impl 判定鍵位) -> String {
        if !鍵位.有無鍵位() {
            String::new()
        } else {
            self.字根表
                .iter()
                .filter(|鍵| 鍵位.包含鍵位(&鍵.鍵碼))
                .map(|鍵| 鍵.輸入碼)
                .collect::<String>()
        }
    }
}

impl 轉寫法定義<'_> {
    pub fn 字根碼轉寫爲拼式(&self, 字根碼: &str) -> Option<String> {
        施展拼寫運算(字根碼, self.拼式轉寫規則)
    }

    pub fn 拼式拆分爲字根碼(&self, 轉寫碼: &str) -> Option<String> {
        施展拼寫運算(轉寫碼, self.字根拆分規則)
    }

    pub fn 驗證拼式(&self, 待驗證拼式: &str) -> bool {
        self.拼式驗證規則.iter().any(|r| r.is_match(待驗證拼式))
    }
}
