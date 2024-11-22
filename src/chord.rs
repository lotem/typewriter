use crate::engine::{並擊狀態, 輸入碼, 鍵組};
use leptos::*;

pub fn 並擊機關(
    目標輸入碼: impl Fn() -> Option<輸入碼> + Copy + 'static,
) -> (
    ReadSignal<並擊狀態>,
    WriteSignal<並擊狀態>,
    impl Fn() + Copy + 'static,
    impl Fn() -> String + Copy + 'static,
    Memo<Option<String>>,
    Memo<Option<鍵組>>,
    impl Fn() -> Option<String> + Copy + 'static,
    impl Fn() -> bool + Copy + 'static,
    impl Fn() -> bool + Copy + 'static,
    impl Fn() -> bool + Copy + 'static,
) {
    let (並擊狀態流, 並擊狀態變更) = create_signal(並擊狀態::new());

    let 重置並擊狀態 = move || 並擊狀態變更.update(並擊狀態::重置);

    let 實況並擊碼 = move || 並擊狀態流.with(並擊狀態::並擊序列);
    let 並擊所得拼音 = create_memo(move |_| 並擊狀態::並擊變換(&實況並擊碼()));

    let 反查鍵位 = create_memo(move |_| 目標輸入碼().as_ref().and_then(輸入碼::反查鍵位));
    let 反查所得並擊碼 = move || 反查鍵位().as_ref().map(鍵組::寫成並擊序列);

    let 並擊開始 = move || 並擊狀態流.with(|狀態| !狀態.實時落鍵.0.is_empty());
    let 並擊完成 =
        move || 並擊狀態流.with(|狀態| 狀態.實時落鍵.0.is_empty()) && !實況並擊碼().is_empty();

    let 並擊成功 = move || {
        // 拼音一致即爲成功，允許並擊碼不同
        目標輸入碼()
            .and_then(|輸入碼| 輸入碼.轉寫碼原文)
            .is_some_and(|查得| 並擊所得拼音().is_some_and(|擊得| 查得 == 擊得))
            // 拼音爲非音節形式的聲母、韻母，須比較並擊碼
            || 反查所得並擊碼().is_some_and(|查得| 查得 == 實況並擊碼())
    };

    (
        並擊狀態流,
        並擊狀態變更,
        重置並擊狀態,
        實況並擊碼,
        並擊所得拼音,
        反查鍵位,
        反查所得並擊碼,
        並擊開始,
        並擊完成,
        並擊成功,
    )
}
