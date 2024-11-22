use leptos::logging::log;
use leptos::*;

use crate::drills::預設練習題;

#[component]
pub fn Rime編碼回顯區(
    輸入碼: Signal<String>, 拼音: Signal<Option<String>>
) -> impl IntoView {
    view! {
        <kbd class="raw-input">{輸入碼}</kbd>
        <span class="translated-input">{拼音}</span>
    }
}

#[component]
pub fn Rime反查輸入欄(
    反查碼: Signal<String>,
    示例輸入: Signal<String>,
    反查碼變更: impl Fn(String) + 'static,
    關閉輸入欄: impl Fn() + 'static,
) -> impl IntoView {
    let 反查輸入欄的引用 = create_node_ref::<html::Input>();
    create_render_effect(move |_| {
        if let Some(輸入欄) = 反查輸入欄的引用() {
            let _不看結果 = 輸入欄.on_mount(|輸入欄| {
                輸入欄.select();
            });
        }
    });

    view! {
        <input type="text" class="lookup-code"
            _ref=反查輸入欄的引用
            placeholder={示例輸入}
            value={反查碼}
            on:input=move |ev| {
                let 反查碼 = event_target_value(&ev);
                反查碼變更(反查碼);
            }
            on:blur=move |_| 關閉輸入欄()
        />
    }
}

#[component]
pub fn Rime練習題選單(
    當選題號: Signal<Option<usize>>,
    選中題號: impl Fn(usize) + 'static,
    關閉選單: impl Fn() + 'static,
) -> impl IntoView {
    let 練習題選單的引用 = create_node_ref::<html::Select>();
    create_render_effect(move |_| {
        let 選項序號 = 當選題號()
            .and_then(|題號| 題號.try_into().ok())
            .unwrap_or(-1);
        if let Some(輸入欄) = 練習題選單的引用() {
            let _不看結果 = 輸入欄.on_mount(move |輸入欄| {
                輸入欄.set_selected_index(選項序號);
                let _ = 輸入欄.focus();
            });
        }
    });

    view! {
        <select class="excercises"
            _ref=練習題選單的引用
            on:change=move |ev| {
                let 題號 = event_target_value(&ev);
                log!("題號: {}", 題號);
                if let Ok(題號) = 題號.parse::<usize>() {
                    if 題號 < 預設練習題.len() {
                        選中題號(題號);
                    }
                }
            }
            on:blur=move |_| 關閉選單()
        >
        {
            預設練習題.iter().enumerate().map(|(題號, 題)| view! {
                <option value={題號}>{題.標題}</option>
            }).collect_view()
        }
        </select>
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
    並擊成功: Signal<bool>,
    點擊動作: impl Fn() + 'static,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="input-code"
            class:freeplay={move || 顯示選項() == 編碼欄顯示選項::顯示實況}
            class:target={move || 顯示選項() == 編碼欄顯示選項::顯示反查}
            class:success={並擊成功}
            on:click={move |_| 點擊動作()}
        >
        {children()}
        </div>
    }
}
