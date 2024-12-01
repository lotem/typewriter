use keyberon::key_code::KeyCode;
use lazy_regex::Regex;
use std::collections::BTreeSet;

use crate::layout::盤面選擇碼;
use crate::spelling_algebra::{拼寫運算, 施展拼寫運算};

pub struct 鍵的定義<'a> {
    pub 輸入碼: &'a str,
    pub 鍵碼: KeyCode,
}

#[derive(Clone, PartialEq)]
pub struct 鍵組(pub BTreeSet<KeyCode>);

impl 鍵組 {
    pub fn new() -> Self {
        鍵組(BTreeSet::new())
    }
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

    pub fn 字根碼轉寫爲拼式(&self, 字根碼: &str) -> Option<String> {
        施展拼寫運算(字根碼, self.轉寫法.拼式轉寫規則)
    }

    pub fn 拼式拆分爲字根碼(&self, 轉寫碼: &str) -> Option<String> {
        施展拼寫運算(轉寫碼, self.轉寫法.字根拆分規則)
    }

    pub fn 驗證拼式(&self, 待驗證拼式: &str) -> bool {
        self.轉寫法
            .拼式驗證規則
            .iter()
            .any(|r| r.is_match(待驗證拼式))
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct 連擊狀態 {
    pub 鍵碼: KeyCode,
    pub 連擊次數: usize,
}

impl Default for 連擊狀態 {
    fn default() -> Self {
        Self {
            鍵碼: KeyCode::No,
            連擊次數: 0,
        }
    }
}

impl 連擊狀態 {
    pub fn 擊發(&self, 鍵碼: KeyCode) -> 連擊狀態 {
        let 連擊次數 = if 鍵碼 == self.鍵碼 {
            self.連擊次數 + 1
        } else {
            1
        };
        Self {
            鍵碼, 連擊次數
        }
    }
}

pub struct 並擊狀態 {
    pub 實時落鍵: 鍵組,
    pub 累計擊鍵: 鍵組,
}

impl 並擊狀態 {
    pub fn new() -> Self {
        並擊狀態 {
            實時落鍵: 鍵組::new(),
            累計擊鍵: 鍵組::new(),
        }
    }

    pub fn 落鍵(&mut self, 鍵碼: KeyCode) {
        if self.實時落鍵.0.is_empty() {
            self.並擊開始();
        }
        self.實時落鍵.0.insert(鍵碼);
        self.累計擊鍵.0.insert(鍵碼);
    }

    pub fn 抬鍵(&mut self, 鍵碼: KeyCode) {
        self.實時落鍵.0.remove(&鍵碼);
        if self.實時落鍵.0.is_empty() {
            self.並擊完成();
        }
    }

    pub fn 重置(&mut self) {
        self.實時落鍵.0.clear();
        self.累計擊鍵.0.clear();
    }

    pub fn 並擊開始(&mut self) {
        self.重置();
    }

    pub fn 並擊完成(&mut self) {}
}

#[derive(Clone, PartialEq)]
pub struct 對照輸入碼 {
    pub 並擊碼原文: Option<String>,
    pub 轉寫碼原文: Option<String>,
}

impl 對照輸入碼 {
    pub fn 反查字根碼<'a>(&'a self, 方案: &輸入方案定義<'a>) -> Option<String> {
        self.並擊碼原文.to_owned().or_else(|| {
            self.轉寫碼原文
                .as_deref()
                .filter(|轉寫碼| 方案.驗證拼式(轉寫碼))
                .and_then(|轉寫碼| 方案.拼式拆分爲字根碼(轉寫碼))
        })
    }
}
