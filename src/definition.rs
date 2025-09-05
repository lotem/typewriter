use lazy_regex::Regex;
use std::borrow::Cow;
use std::collections::BTreeSet;

use crate::gear::layout::鍵盤佈局;
use crate::key_code::KeyCode;
use crate::spelling_algebra::{拼寫運算, 施展拼寫運算};

pub struct 鍵位定義<'a> {
    pub 輸入碼: &'a str,
    pub 鍵碼: KeyCode,
}

#[derive(Clone, Copy)]
pub struct 輸入方案定義<'a> {
    pub 名稱: &'a str,
    pub 佈局: &'a 鍵盤佈局,
    pub 指法: 觸鍵方式,
    pub 字根表: &'a [鍵位定義<'a>],
    pub 轉寫法: 轉寫法定義<'a>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum 觸鍵方式 {
    連擊,
    並擊,
}

#[derive(Clone, Copy)]
pub struct 轉寫法定義<'a> {
    /// 將按鍵序列轉換成慣用的表示形式，如字母與附標符號合字
    pub 輸入碼表示: &'a [拼寫運算<'a>],
    /// 將輸入碼的表示形式轉換成按鍵序列
    pub 輸入碼鍵位: &'a [拼寫運算<'a>],
    /// 將輸入碼轉寫成符合詞典規範的編碼
    pub 拼式轉寫規則: &'a [拼寫運算<'a>],
    /// 將詞典碼拆分爲按鍵序列
    pub 字根拆分規則: &'a [拼寫運算<'a>],
    /// 定義若干識別有效詞典碼的規則。若未定義任何規則，則不做驗證
    pub 拼式驗證規則: &'a [&'a Regex],
}

pub trait 判定鍵位 {
    fn 有無鍵位(&self) -> bool;
    fn 包含鍵位(&self, 鍵碼: &KeyCode) -> bool;
}

#[derive(Clone, Debug, PartialEq)]
pub struct 鍵組(pub BTreeSet<KeyCode>);

impl 鍵組 {
    pub fn new() -> Self {
        鍵組(BTreeSet::new())
    }
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
    pub fn 尋得字根(&self, 字根: &str) -> Option<&鍵位定義<'_>> {
        self.字根表.iter().find(|鍵| 鍵.輸入碼 == 字根)
    }

    pub fn 讀出鍵位(&self, 字根碼: &str) -> 鍵組 {
        let 鍵碼序列 = 施展拼寫運算(字根碼, self.轉寫法.輸入碼鍵位)
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(字根碼));
        鍵組(
            self.字根表
                .iter()
                .filter(|鍵| 鍵碼序列.contains(鍵.輸入碼))
                .map(|鍵| 鍵.鍵碼)
                .collect(),
        )
    }

    pub fn 寫成字根碼(&self, 鍵位: impl 判定鍵位) -> String {
        if !鍵位.有無鍵位() {
            String::new()
        } else {
            let 字根碼 = self
                .字根表
                .iter()
                .filter(|鍵| 鍵位.包含鍵位(&鍵.鍵碼))
                .map(|鍵| 鍵.輸入碼)
                .collect::<String>();
            施展拼寫運算(&字根碼, self.轉寫法.輸入碼表示).unwrap_or(字根碼)
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
