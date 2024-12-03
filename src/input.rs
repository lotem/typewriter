use keyberon::key_code::KeyCode;
use leptos::*;
use leptos::{
    ev::{keydown, keyup, KeyboardEvent},
    logging::log,
};
use leptos_use::{use_document, use_event_listener, use_window_focus};

use crate::engine::{並擊狀態, 連擊狀態};
use crate::key_code::網頁鍵值轉換;
use crate::mode::工作模式;

pub fn 焦點事件處理機關(重置並擊狀態: impl Fn() + Copy + 'static) {
    let 鍵盤輸入焦點源 = create_selector(use_window_focus());
    create_effect(move |_| {
        if 鍵盤輸入焦點源.selected(false) {
            重置並擊狀態();
        }
    });
}

#[allow(clippy::too_many_arguments)]
pub fn 輸入事件處理機關(
    連擊狀態變更: WriteSignal<連擊狀態>,
    並擊狀態變更: WriteSignal<並擊狀態>,
    現行工作模式: ReadSignal<工作模式>,
    處理功能鍵: impl Fn(KeyCode) -> bool + Copy + 'static,
    既然落鍵: impl Fn() + Copy + 'static,
    既然抬鍵: impl Fn() + Copy + 'static,
) {
    let _ = use_event_listener(use_document().body(), keydown, move |evt: KeyboardEvent| {
        log!("落鍵 key = {}, code = {}", &evt.key(), evt.code());
        let 鍵碼 = 網頁鍵值轉換(&evt.code());
        if 處理功能鍵(鍵碼) {
            evt.prevent_default();
        }
        if 現行工作模式() == 工作模式::錄入 {
            連擊狀態變更.update(|連擊| *連擊 = 連擊.擊發(鍵碼));
            並擊狀態變更.update(|並擊| 並擊.落鍵(鍵碼));
        }
        既然落鍵();
    });

    let _ = use_event_listener(use_document().body(), keyup, move |evt: KeyboardEvent| {
        log!("抬鍵 key = {}, code = {}", &evt.key(), &evt.code());
        if 現行工作模式() == 工作模式::錄入 {
            let 鍵碼 = 網頁鍵值轉換(&evt.code());
            並擊狀態變更.update(|並擊| 並擊.抬鍵(鍵碼));
        }
        既然抬鍵();
    });
}
