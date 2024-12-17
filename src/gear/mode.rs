use leptos::prelude::*;

use crate::action::*;
use crate::gear::{assignment::作業, theory::方案選項};

#[derive(Clone, Debug, PartialEq)]
pub enum 工作模式 {
    錄入,
    輸入反查碼,
    選取練習題,
    選擇輸入方案,
}

pub fn 工作模式機關(
    現行方案: ReadSignal<方案選項>,
    作業進度完成: Signal<bool>,
    佈置作業: WriteSignal<作業>,
    重置作業進度: impl 動作,
    重置輸入狀態: impl 動作,
) -> (
    // 現行工作模式
    ReadSignal<工作模式>,
    // 開啓反查輸入
    impl 動作,
    // 開啓練習題選單
    impl 動作,
    // 開啓方案選單
    impl 動作,
    // 關閉輸入欄
    impl 動作,
) {
    let (現行工作模式, 設置工作模式) = signal(工作模式::錄入);

    let 開啓反查輸入 = move || {
        if 作業進度完成() {
            佈置作業(作業::自習(現行方案()));
        }
        重置輸入狀態();
        設置工作模式(工作模式::輸入反查碼);
    };

    let 開啓練習題選單 = move || {
        重置作業進度();
        重置輸入狀態();
        設置工作模式(工作模式::選取練習題);
    };

    let 開啓方案選單 = move || {
        設置工作模式(工作模式::選擇輸入方案);
    };

    let 關閉輸入欄 = move || {
        設置工作模式(工作模式::錄入);
    };

    (
        現行工作模式,
        開啓反查輸入,
        開啓練習題選單,
        開啓方案選單,
        關閉輸入欄,
    )
}
