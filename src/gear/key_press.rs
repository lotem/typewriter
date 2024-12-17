use keyberon::key_code::KeyCode;
use leptos::logging::log;
use leptos::prelude::*;

use crate::action::{動作, 動作給一參數};
use crate::definition::輸入方案定義;
use crate::gear::assignment::對照輸入碼;

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

#[allow(clippy::type_complexity)]
pub fn 連擊機關(
    方案: Signal<輸入方案定義<'static>>,
    目標輸入碼: Signal<Option<對照輸入碼>>,
) -> (
    // 連擊狀態流
    ReadSignal<連擊狀態>,
    // 連擊狀態變更
    WriteSignal<連擊狀態>,
    // 實況字根碼
    Signal<String>,
    // 連擊比對成功
    Signal<bool>,
    // 連擊輸入碼
    ReadSignal<Vec<String>>,
    // 更新連擊輸入碼
    impl 動作給一參數<KeyCode>,
    // 清空連擊輸入碼
    impl 動作,
    // 回退連擊輸入碼
    impl 動作,
) {
    let (連擊狀態流, 連擊狀態變更) = signal(連擊狀態::default());

    let 實況字根碼 = Signal::derive(move || 方案.read().寫成字根碼(連擊狀態流.read().鍵碼));

    let 反查所得字根碼 = move || {
        目標輸入碼
            .read()
            .as_ref()
            .and_then(|對照碼| 對照碼.反查字根碼(&方案.read().轉寫法))
    };

    let 連擊比對成功 =
        Memo::new(move |_| 反查所得字根碼().is_some_and(|查得| 查得 == 實況字根碼()));

    let (連擊輸入碼, 更新連擊輸入碼) = signal(Vec::<String>::new());

    let 清空連擊輸入碼 = move || {
        更新連擊輸入碼.write().clear();
    };

    let 回退連擊輸入碼 = move || {
        更新連擊輸入碼.write().pop();
    };

    let 編輯連擊輸入碼 = move |鍵碼: KeyCode| {
        let 自由輸入 = 目標輸入碼.read().is_none();
        let 擊鍵正確 = 連擊比對成功();
        if 自由輸入 || 擊鍵正確 {
            let 方案 = 方案.read();
            match 鍵碼 {
                KeyCode::Space => {
                    let 空格 = 方案.寫成字根碼(KeyCode::Space);
                    更新連擊輸入碼(vec![空格]);
                }
                鍵碼 => {
                    let 字根碼 = 方案.寫成字根碼(鍵碼);
                    if !字根碼.is_empty() {
                        log!("更新連擊輸入碼 {字根碼}");
                        let 空格 = 方案.寫成字根碼(KeyCode::Space);
                        if *連擊輸入碼.read() == [空格] {
                            更新連擊輸入碼(vec![字根碼]);
                        } else {
                            更新連擊輸入碼.write().push(字根碼);
                        }
                    }
                }
            }
        }
    };

    (
        連擊狀態流,
        連擊狀態變更,
        實況字根碼,
        連擊比對成功.into(),
        連擊輸入碼,
        編輯連擊輸入碼,
        清空連擊輸入碼,
        回退連擊輸入碼,
    )
}
