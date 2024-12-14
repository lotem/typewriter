use keyberon::key_code::KeyCode;
use leptos::*;
use leptos::{
    ev::{keydown, keyup, KeyboardEvent},
    logging::log,
};
use leptos_use::{use_document, use_event_listener, use_window_focus};

use crate::key_code::網頁鍵值轉換;

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
    處理功能鍵: impl Fn(KeyCode) -> bool + Copy + 'static,
    既然落鍵: impl Fn(KeyCode) + Copy + 'static,
    既然抬鍵: impl Fn(KeyCode) + Copy + 'static,
) {
    let _ = use_event_listener(use_document().body(), keydown, move |evt: KeyboardEvent| {
        log!("落鍵 key = {}, code = {}", &evt.key(), evt.code());
        let 鍵碼 = 網頁鍵值轉換(&evt.code());
        if 處理功能鍵(鍵碼) {
            evt.prevent_default();
        }
        既然落鍵(鍵碼);
    });

    let _ = use_event_listener(use_document().body(), keyup, move |evt: KeyboardEvent| {
        log!("抬鍵 key = {}, code = {}", &evt.key(), &evt.code());
        let 鍵碼 = 網頁鍵值轉換(&evt.code());
        既然抬鍵(鍵碼);
    });
}
