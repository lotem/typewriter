use leptos::{ev, leptos_dom::helpers::window_event_listener, logging::log, prelude::*};
use leptos_use::use_window_focus;

use crate::action::{動作, 動作給一參數, 動作給一參數得一結果};
use crate::key_code::{KeyCode, 網頁鍵值轉換};

pub fn 焦點事件處理機關(重置並擊狀態: impl 動作) {
    let 鍵盤輸入焦點源 = Selector::new(use_window_focus());
    Effect::new(move |_| {
        if 鍵盤輸入焦點源.selected(false) {
            重置並擊狀態();
        }
    });
}

#[allow(dead_code)]
pub struct 檔位 {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

pub struct 觸鍵消息 {
    pub 鍵碼: KeyCode,
    pub 檔位: 檔位,
}

pub fn 輸入事件處理機關(
    處理功能鍵: impl 動作給一參數得一結果<觸鍵消息, bool>,
    既然落鍵: impl 動作給一參數<KeyCode>,
    既然抬鍵: impl 動作給一參數<KeyCode>,
) {
    let keydown_handle = window_event_listener(ev::keydown, move |ev| {
        log!("落鍵 key = {}, code = {}", &ev.key(), ev.code());
        let 鍵碼 = 網頁鍵值轉換(&ev.code());
        let 檔位 = 檔位 {
            shift: ev.shift_key(),
            ctrl: ev.ctrl_key(),
            alt: ev.alt_key(),
            meta: ev.meta_key(),
        };
        if 處理功能鍵(觸鍵消息 { 鍵碼, 檔位 }) {
            ev.prevent_default();
        }
        if 鍵碼 != KeyCode::No {
            既然落鍵(鍵碼);
        }
    });

    let keyup_handle = window_event_listener(ev::keyup, move |ev| {
        log!("抬鍵 key = {}, code = {}", &ev.key(), &ev.code());
        let 鍵碼 = 網頁鍵值轉換(&ev.code());
        if 鍵碼 != KeyCode::No {
            既然抬鍵(鍵碼);
        }
    });

    on_cleanup(move || {
        keydown_handle.remove();
        keyup_handle.remove();
    });
}
