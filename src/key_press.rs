use keyberon::key_code::KeyCode;
use leptos::logging::log;
use leptos::*;

use crate::assignment::對照輸入碼;
use crate::engine::輸入方案定義;

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
    // 反查所得字根碼
    Signal<Option<String>>,
    // 連擊比對成功
    Signal<bool>,
    // 連擊輸入碼
    ReadSignal<Vec<String>>,
    // 更新連擊輸入碼
    WriteSignal<Vec<String>>,
    // 清空連擊輸入碼
    impl Fn() + Copy + 'static,
    // 回退連擊輸入碼
    impl Fn() + Copy + 'static,
) {
    let (連擊狀態流, 連擊狀態變更) = create_signal(連擊狀態::default());

    let 實況字根碼 =
        Signal::derive(move || with!(|方案, 連擊狀態流| 方案.寫成字根碼(連擊狀態流.鍵碼)));

    let 反查所得字根碼 = create_memo(move |_| {
        with!(|方案, 目標輸入碼| 目標輸入碼
            .as_ref()
            .and_then(|對照碼| 對照碼.反查字根碼(&方案.轉寫法)))
    });

    let 連擊比對成功 =
        Signal::derive(move || 反查所得字根碼().is_some_and(|查得| 查得 == 實況字根碼()));

    let (連擊輸入碼, 更新連擊輸入碼) = create_signal(Vec::<String>::new());

    let 清空連擊輸入碼 = move || {
        update!(|更新連擊輸入碼| {
            更新連擊輸入碼.clear();
        })
    };

    let 回退連擊輸入碼 = move || {
        update!(|更新連擊輸入碼| {
            更新連擊輸入碼.pop();
        })
    };

    let 空格輸入碼 = create_memo(move |_| with!(|方案| 方案.寫成字根碼(KeyCode::Space)));

    let _ = watch(
        連擊狀態流,
        move |連擊, _, _| {
            if 目標輸入碼.with(|目標| 目標.is_none()) || 連擊比對成功() {
                match 連擊.鍵碼 {
                    KeyCode::Space => {
                        更新連擊輸入碼(vec![空格輸入碼()]);
                    }
                    鍵碼 => {
                        let 字根碼 = with!(|方案| 方案.寫成字根碼(鍵碼));
                        if !字根碼.is_empty() {
                            log!("更新連擊輸入碼 {字根碼}");
                            update!(|更新連擊輸入碼| {
                                if *更新連擊輸入碼 == [空格輸入碼()] {
                                    *更新連擊輸入碼 = vec![字根碼];
                                } else {
                                    更新連擊輸入碼.push(字根碼);
                                }
                            });
                        }
                    }
                }
            }
        },
        false,
    );

    (
        連擊狀態流,
        連擊狀態變更,
        實況字根碼,
        反查所得字根碼.into(),
        連擊比對成功,
        連擊輸入碼,
        更新連擊輸入碼,
        清空連擊輸入碼,
        回退連擊輸入碼,
    )
}
