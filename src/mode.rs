use leptos::*;

use crate::assignment::作業;

#[derive(Clone, PartialEq)]
pub enum 工作模式 {
    錄入,
    輸入反查碼,
    選取練習題,
}

pub fn 工作模式機關(
    作業進度完成: Signal<bool>,
    佈置作業: WriteSignal<作業>,
    重置作業進度: impl Fn() + Copy + 'static,
    重置並擊狀態: impl Fn() + Copy + 'static,
) -> (
    // 現行工作模式
    ReadSignal<工作模式>,
    // 開啓反查輸入
    impl Fn() + Copy + 'static,
    // 開啓練習題選單
    impl Fn() + Copy + 'static,
    // 關閉輸入欄
    impl Fn() + Copy + 'static,
) {
    let (現行工作模式, 設置工作模式) = create_signal(工作模式::錄入);

    let 開啓反查輸入 = move || {
        if 作業進度完成() {
            佈置作業(作業::自習());
        }
        重置並擊狀態();
        設置工作模式(工作模式::輸入反查碼);
    };

    let 開啓練習題選單 = move || {
        重置作業進度();
        重置並擊狀態();
        設置工作模式(工作模式::選取練習題);
    };

    let 關閉輸入欄 = move || {
        設置工作模式(工作模式::錄入);
    };

    (現行工作模式, 開啓反查輸入, 開啓練習題選單, 關閉輸入欄)
}
