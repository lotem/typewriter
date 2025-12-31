use leptos::{ ev, leptos_dom::helpers::window_event_listener, logging::log, prelude::* };
use leptos_use::use_window_focus;

use crate::action::{ 动作给一参数, 动作给一参数得一结果, 动作 };
use crate::key_code::{ KeyCode, 网页键值转换 };

pub fn 焦点事件处理机关(重置并击状态: impl 动作) {
    let 键盘输入焦点源 = Selector::new(use_window_focus());
    Effect::new(move |_| {
        if 键盘输入焦点源.selected(&false) {
            重置并击状态();
        }
    });
}

#[allow(dead_code)]
pub struct 档位 {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

pub struct 击键消息 {
    pub 键码: KeyCode,
    pub 档位: 档位,
}

pub fn 输入事件处理机关(
    处理功能键: impl 动作给一参数得一结果<击键消息, bool>,
    既然落键: impl 动作给一参数<KeyCode>,
    既然擡键: impl 动作给一参数<KeyCode>
) {
    let keydown_handle = window_event_listener(ev::keydown, move |ev| {
        log!("落键 key = {}, code = {}", &ev.key(), ev.code());
        let 键码 = 网页键值转换(&ev.code());
        let 档位 = 档位 {
            shift: ev.shift_key(),
            ctrl: ev.ctrl_key(),
            alt: ev.alt_key(),
            meta: ev.meta_key(),
        };
        if 处理功能键(击键消息 { 键码, 档位 }) {
            ev.prevent_default();
        }
        if 键码 != KeyCode::No {
            既然落键(键码);
        }
    });

    let keyup_handle = window_event_listener(ev::keyup, move |ev| {
        log!("抬键 key = {}, code = {}", &ev.key(), &ev.code());
        let 键码 = 网页键值转换(&ev.code());
        if 键码 != KeyCode::No {
            既然擡键(键码);
        }
    });

    on_cleanup(move || {
        keydown_handle.remove();
        keyup_handle.remove();
    });
}
