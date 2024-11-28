use keyberon::key_code::KeyCode;
use lazy_regex::{regex, Regex};
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
pub struct 並擊輸入方案<'a> {
    pub 盤面: 盤面選擇碼,
    pub 並擊鍵序: &'a [鍵的定義<'a>],
    pub 並擊轉輸入碼規則: &'a [拼寫運算<'a>],
    pub 輸入碼轉並擊規則: &'a [拼寫運算<'a>],
    pub 驗證輸入碼規則: &'a [&'a Regex],
}

impl 並擊輸入方案<'_> {
    pub fn 讀出並擊鍵位(&self, 並擊碼: &str) -> 鍵組 {
        鍵組(
            self.並擊鍵序
                .iter()
                .filter(|鍵| 並擊碼.contains(鍵.輸入碼))
                .map(|鍵| 鍵.鍵碼)
                .collect(),
        )
    }

    pub fn 寫成並擊碼(&self, 鍵位: &鍵組) -> String {
        if 鍵位.0.is_empty() {
            String::new()
        } else {
            self.並擊鍵序
                .iter()
                .filter(|鍵| 鍵位.0.contains(&鍵.鍵碼))
                .map(|鍵| 鍵.輸入碼)
                .collect::<String>()
        }
    }

    pub fn 並擊轉輸入碼(&self, 並擊碼: &str) -> Option<String> {
        施展拼寫運算(並擊碼, self.並擊轉輸入碼規則)
    }

    pub fn 輸入碼轉並擊(&self, 輸入碼: &str) -> Option<String> {
        施展拼寫運算(輸入碼, self.輸入碼轉並擊規則)
    }

    pub fn 驗證輸入碼(&self, 待驗證: &str) -> bool {
        self.驗證輸入碼規則.iter().any(|r| r.is_match(待驗證))
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
    pub fn 反查並擊碼<'a>(&'a self, 方案: &並擊輸入方案<'a>) -> Option<String> {
        self.並擊碼原文.to_owned().or_else(|| {
            self.轉寫碼原文
                .as_deref()
                .filter(|轉寫碼| 方案.驗證輸入碼(轉寫碼))
                .and_then(|轉寫碼| 方案.輸入碼轉並擊(轉寫碼))
        })
    }
}

/// 將輸入碼序列解析爲輸入碼片段.
///
/// 輸入碼通常是拼音音節的序列, 音節之間用空白或隔音符號 `'` 分開.
/// 特殊形式的拼音寫在尖括號中, 如: `<'a>`。
///
/// 輸入碼片段也可以是以下形式:
///
/// - 用大寫字母連書並擊碼, 如 `ZFURO`
/// - 寫明並擊碼和對應的拼音, 如 `SHGUA=shu'ru'fa`
/// - 寫明並擊碼並將對應的拼音寫在尖括號中, 如 `SHGUA=<shu ru fa>`
pub fn 解析輸入碼序列(輸入碼序列: &str) -> Vec<對照輸入碼> {
    let 輸入碼片段模式 = regex!(
        r"(?x)
        (?P<chord> \p{Uppercase}+ )
        (?:
            = (?P<eq_code> [\w'] )+ |
            =< (?P<eq_quoted_code> [^<>]* ) >
        )? |
        (?P<code> \w+ ) |
        <(?P<quoted_code> [^<>]* )>
    "
    );
    輸入碼片段模式
        .captures_iter(輸入碼序列)
        .map(|片段| {
            let 並擊碼原文 = 片段.name("chord").map(|m| m.as_str().to_owned());
            let 轉寫碼原文 = 片段
                .name("code")
                .or_else(|| 片段.name("quoted_code"))
                .or_else(|| 片段.name("eq_code"))
                .or_else(|| 片段.name("eq_quoted_code"))
                .map(|m| m.as_str().to_owned());
            對照輸入碼 {
                並擊碼原文,
                轉寫碼原文,
            }
        })
        .collect()
}
