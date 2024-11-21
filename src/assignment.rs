use crate::drills::預設練習題;

#[derive(Clone, PartialEq)]
pub struct 作業 {
    pub 題號: Option<usize>,
    pub 自訂反查碼: Option<String>,
}

impl 作業 {
    pub fn 練習題(題號: usize) -> Self {
        Self {
            題號: Some(題號),
            自訂反查碼: None,
        }
    }

    pub fn 自訂(反查碼: String) -> Self {
        Self {
            題號: None,
            自訂反查碼: Some(反查碼),
        }
    }

    pub fn 自習() -> Self {
        Self {
            題號: None,
            自訂反查碼: None,
        }
    }

    pub fn 反查碼(&self) -> &str {
        self.題號
            .and_then(|題號| 預設練習題.get(題號))
            .map(|題| 題.編碼)
            .or(self.自訂反查碼.as_deref())
            .unwrap_or("")
    }

    pub fn 字幕(&self) -> Option<&'static str> {
        self.題號
            .and_then(|題號| 預設練習題.get(題號))
            .and_then(|題| 題.字幕)
    }

    pub fn 是否練習題(&self) -> bool {
        self.題號.is_some()
    }
}
