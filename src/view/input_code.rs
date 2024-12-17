use leptos::html;
use leptos::prelude::*;
use leptos_use::on_click_outside;

use crate::action::{動作, 動作給一參數};

#[derive(Clone, Copy, PartialEq)]
pub enum 回顯區佈局 {
    單欄,
    左右對照,
}

#[component]
pub fn Rime編碼回顯區(
    佈局: Signal<回顯區佈局>,
    輸入碼: Signal<String>,
    轉寫碼: Signal<Option<String>>,
) -> impl IntoView {
    view! {
        <kbd class="raw-input"
            class:single-column=move || { 佈局() == 回顯區佈局::單欄 }
            class:left-column=move || { 佈局() == 回顯區佈局::左右對照 }
        >{輸入碼}</kbd>
        <Show when=move || { 佈局() == 回顯區佈局::左右對照 }>
            <span class="translated-input right-column">{轉寫碼}</span>
        </Show>
    }
}

#[component]
pub fn Rime反查輸入欄(
    反查碼: Signal<Option<String>>,
    示例輸入: Signal<String>,
    反查碼變更: impl 動作給一參數<String>,
) -> impl IntoView {
    let 反查輸入欄的引用 = NodeRef::<html::Input>::new();
    反查輸入欄的引用.on_load(|輸入欄| {
        輸入欄.select();
    });

    view! {
        <input type="text" class="lookup-code"
            node_ref=反查輸入欄的引用
            placeholder={示例輸入}
            value={反查碼}
            on:input=move |ev| {
                let 反查碼 = event_target_value(&ev);
                反查碼變更(反查碼);
            }
        />
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum 編碼欄顯示選項 {
    無顯示,
    顯示實況,
    顯示反查,
}

#[component]
pub fn Rime編碼欄(
    顯示選項: Signal<編碼欄顯示選項>,
    輸入正確: Signal<bool>,
    點擊動作: impl 動作,
    關閉輸入欄: impl 動作,
    children: Children,
) -> impl IntoView {
    let target = NodeRef::<html::Div>::new();
    let _ = on_click_outside(target, move |_| {
        關閉輸入欄();
    });

    view! {
        <div node_ref=target class="input-code"
            class:freeplay={move || 顯示選項() == 編碼欄顯示選項::顯示實況}
            class:target={move || 顯示選項() == 編碼欄顯示選項::顯示反查}
            class:success={輸入正確}
            on:click={move |_| 點擊動作()}
        >
        {children()}
        </div>
    }
}
