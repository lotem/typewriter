use leptos::*;

use crate::engine::{對照輸入碼, 輸入方案定義, 連擊狀態};

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
    // 重置連擊狀態
    impl Fn() + Copy + 'static,
) {
    let (連擊狀態流, 連擊狀態變更) = create_signal(連擊狀態::default());

    let 重置連擊狀態 = move || 連擊狀態變更.update(|連擊| *連擊 = 連擊狀態::default());

    let 實況字根碼 =
        Signal::derive(move || with!(|方案, 連擊狀態流| 方案.寫成字根碼(連擊狀態流.鍵碼)));

    let 反查所得字根碼 = create_memo(move |_| {
        with!(|方案, 目標輸入碼| 目標輸入碼
            .as_ref()
            .and_then(|對照碼| 對照碼.反查字根碼(&方案.轉寫法)))
    });

    let 連擊比對成功 =
        Signal::derive(move || 反查所得字根碼().is_some_and(|查得| 查得 == 實況字根碼()));

    (
        連擊狀態流,
        連擊狀態變更,
        實況字根碼,
        反查所得字根碼.into(),
        連擊比對成功,
        重置連擊狀態,
    )
}
