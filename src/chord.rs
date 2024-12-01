use leptos::*;

use crate::engine::{並擊狀態, 對照輸入碼, 輸入方案定義, 鍵組};

#[allow(clippy::type_complexity)]
pub fn 並擊機關(
    方案: Signal<輸入方案定義<'static>>,
    目標輸入碼: Signal<Option<對照輸入碼>>,
) -> (
    // 並擊狀態流
    ReadSignal<並擊狀態>,
    // 並擊狀態變更
    WriteSignal<並擊狀態>,
    // 實況並擊碼
    Signal<String>,
    // 並擊所得拼音
    Signal<Option<String>>,
    // 反查所得並擊碼
    Signal<Option<String>>,
    // 反查鍵位
    Signal<Option<鍵組>>,
    // 並擊開始
    Signal<bool>,
    // 並擊完成
    Signal<bool>,
    // 並擊成功
    Signal<bool>,
    // 重置並擊狀態
    impl Fn() + Copy + 'static,
) {
    let (並擊狀態流, 並擊狀態變更) = create_signal(並擊狀態::new());

    let 重置並擊狀態 = move || 並擊狀態變更.update(並擊狀態::重置);

    let 實況並擊碼 =
        Signal::derive(move || with!(|方案, 並擊狀態流| 方案.寫成字根碼(&並擊狀態流.累計擊鍵)));
    let 並擊所得拼音 = create_memo(move |_| with!(|方案| 方案.字根碼轉寫爲拼式(&實況並擊碼())));

    let 反查所得並擊碼 = create_memo(move |_| {
        with!(|方案, 目標輸入碼| 目標輸入碼
            .as_ref()
            .and_then(|對照碼| 對照碼.反查字根碼(方案)))
    });
    let 反查鍵位 = Signal::derive(move || {
        with!(|方案, 反查所得並擊碼| 反查所得並擊碼
            .as_deref()
            .map(|並擊碼| 方案.讀出鍵位(並擊碼)))
    });

    let 並擊開始 = Signal::derive(move || 並擊狀態流.with(|狀態| !狀態.實時落鍵.0.is_empty()));
    let 並擊完成 = Signal::derive(move || {
        並擊狀態流.with(|狀態| 狀態.實時落鍵.0.is_empty()) && !實況並擊碼().is_empty()
    });

    let 並擊成功 = Signal::derive(move || {
        // 拼音一致即爲成功，允許並擊碼不同
        目標輸入碼()
            .and_then(|輸入碼| 輸入碼.轉寫碼原文)
            .is_some_and(|查得| 並擊所得拼音().is_some_and(|擊得| 查得 == 擊得))
            // 拼音爲非音節形式的聲母、韻母，須比較並擊碼
            || 反查所得並擊碼().is_some_and(|查得| 查得 == 實況並擊碼())
    });

    (
        並擊狀態流,
        並擊狀態變更,
        實況並擊碼,
        並擊所得拼音.into(),
        反查所得並擊碼.into(),
        反查鍵位,
        並擊開始,
        並擊完成,
        並擊成功,
        重置並擊狀態,
    )
}
