use leptos::logging::log;
use leptos::prelude::*;

use crate::action::{動作, 動作給一參數};
use crate::gear::{assignment::作業機關輸出信號, theory::輸入方案機關輸出信號};
use crate::key_code::KeyCode;

#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub fn 擊發(&mut self, 鍵碼: KeyCode) {
        if 鍵碼 == self.鍵碼 {
            self.連擊次數 += 1;
        } else {
            self.鍵碼 = 鍵碼;
            self.連擊次數 = 1;
        }
    }
}

pub type 清空動作 = impl 動作;
pub type 回退動作 = impl 動作;
pub type 擊鍵動作 = impl 動作給一參數<KeyCode>;

#[derive(Clone)]
pub struct 連擊機關輸出信號 {
    pub 連擊狀態變更: WriteSignal<連擊狀態>,
    pub 連擊輸入碼: ReadSignal<Vec<String>>,
    pub 實況字根碼: Signal<String>,
    pub 連擊比對成功: Memo<bool>,
    pub 清空連擊輸入碼: 清空動作,
    pub 回退連擊輸入碼: 回退動作,
    pub 編輯連擊輸入碼: 擊鍵動作,
}

#[define_opaque(清空動作, 回退動作, 擊鍵動作)]
pub fn 連擊機關(
    方案: &輸入方案機關輸出信號,
    作業: &作業機關輸出信號,
) -> 連擊機關輸出信號 {
    let 方案 = 方案.方案定義;
    let 目標輸入碼片段 = 作業.目標輸入碼片段;

    let (連擊狀態流, 連擊狀態變更) = signal(連擊狀態::default());

    let 實況字根碼 = Signal::derive(move || 方案.read().寫成字根碼(連擊狀態流.read().鍵碼));
    let 反查所得字根碼 = move || {
        目標輸入碼片段
            .read()
            .as_ref()
            .and_then(|對照碼| 對照碼.反查字根碼(&方案.read().轉寫法))
    };
    let 實錄分隔鍵 = move || {
        let 實錄鍵碼 = 連擊狀態流.read().鍵碼;
        方案.read().查分隔鍵(move |鍵位| 鍵位.鍵碼 == 實錄鍵碼)
    };
    let 連擊比對成功 = Memo::new(move |_| {
        反查所得字根碼().is_some_and(|查得| {
            查得 == 實況字根碼() || 實錄分隔鍵().is_some_and(|分隔鍵| 查得 == 分隔鍵.輸入碼)
        })
    });

    let (連擊輸入碼, 更新連擊輸入碼) = signal(Vec::<String>::new());

    let 清空連擊輸入碼 = move || {
        更新連擊輸入碼.write().clear();
    };

    let 回退連擊輸入碼 = move || {
        更新連擊輸入碼.write().pop();
    };

    let 位於碼段邊界 = move || {
        let 輸入碼 = 連擊輸入碼.read();
        輸入碼.is_empty()
            || 輸入碼
                .last()
                .and_then(|末位輸入碼| 方案.read().查分隔鍵(|鍵位| 鍵位.輸入碼 == 末位輸入碼))
                .is_some()
    };

    let 編輯連擊輸入碼 = move |鍵碼: KeyCode| {
        let 自由輸入 = 目標輸入碼片段.read().is_none();
        let 擊鍵正確 = 連擊比對成功();
        if 自由輸入 || 擊鍵正確 {
            if 實錄分隔鍵().is_some() || 位於碼段邊界() {
                log!("清空碼段");
                更新連擊輸入碼.write().clear();
            }
            let 字根碼 = 方案.read().寫成字根碼(鍵碼);
            if !字根碼.is_empty() {
                log!("更新連擊輸入碼 {字根碼}");
                更新連擊輸入碼.write().push(字根碼);
            }
        }
    };

    連擊機關輸出信號 {
        連擊狀態變更,
        連擊輸入碼,
        實況字根碼,
        連擊比對成功,
        清空連擊輸入碼,
        回退連擊輸入碼,
        編輯連擊輸入碼,
    }
}
