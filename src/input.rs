use leptos::*;
use leptos::{
    ev::{keydown, keyup, KeyboardEvent},
    logging::log,
};
use leptos_use::{use_document, use_event_listener, use_window_focus};

use crate::engine::並擊狀態;
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
    並擊狀態變更: WriteSignal<並擊狀態>,
    現行工作模式: ReadSignal<工作模式>,
    處理退出鍵: impl Fn() -> bool + Copy + 'static,
    處理製表鍵: impl Fn() -> bool + Copy + 'static,
    處理退格鍵: impl Fn() -> bool + Copy + 'static,
    處理回車鍵: impl Fn() -> bool + Copy + 'static,
    既然落鍵: impl Fn() + Copy + 'static,
    既然抬鍵: impl Fn() + Copy + 'static,
) {
    let 處理功能鍵 = move |evt: &KeyboardEvent| match evt.code().as_str() {
        "Escape" => {
            if 處理退出鍵() {
                evt.prevent_default();
            }
        }
        "Tab" => {
            if 處理製表鍵() {
                evt.prevent_default();
            }
        }
        "Backspace" => {
            if 處理退格鍵() {
                evt.prevent_default();
            }
        }
        "Enter" => {
            if 處理回車鍵() {
                evt.prevent_default();
            }
        }
        _ => (),
    };

    let _ = use_event_listener(use_document().body(), keydown, move |evt: KeyboardEvent| {
        log!("落鍵 key = {}, code = {}", &evt.key(), evt.code());
        處理功能鍵(&evt);
        if 現行工作模式() == 工作模式::錄入 {
            並擊狀態變更.update(|並擊| 並擊.落鍵(網頁鍵值轉換(&evt.code())));
        }
        既然落鍵();
    });

    let _ = use_event_listener(use_document().body(), keyup, move |evt: KeyboardEvent| {
        log!("抬鍵 key = {}, code = {}", &evt.key(), &evt.code());
        if 現行工作模式() == 工作模式::錄入 {
            並擊狀態變更.update(|並擊| 並擊.抬鍵(網頁鍵值轉換(&evt.code())));
        }
        既然抬鍵();
    });
}
