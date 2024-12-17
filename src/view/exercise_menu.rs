use leptos::html;
use leptos::logging::log;
use leptos::prelude::*;

use crate::action::動作給一參數;
use crate::drills::練習題;

#[component]
pub fn Rime練習題選單(
    預設練習題: Signal<&'static [練習題<'static>]>,
    當選題號: Signal<Option<usize>>,
    選中題號: impl 動作給一參數<usize>,
) -> impl IntoView {
    let 練習題選單的引用 = NodeRef::<html::Select>::new();
    let _ = Effect::new(move |_| {
        if let Some(輸入欄) = 練習題選單的引用.get() {
            let 選項序號 = 當選題號()
                .and_then(|題號| 題號.try_into().ok())
                .unwrap_or(-1);
            輸入欄.set_selected_index(選項序號);
            let _ = 輸入欄.focus();
        }
    });

    view! {
        <select class="exercises"
            node_ref=練習題選單的引用
            on:change=move |ev| {
                let 題號 = event_target_value(&ev);
                log!("題號: {}", 題號);
                if let Ok(題號) = 題號.parse::<usize>() {
                    if 題號 < 預設練習題().len() {
                        選中題號(題號);
                    }
                }
            }
        >
        {
            預設練習題().iter().enumerate().map(|(題號, 題)| view! {
                <option value={題號}>{題.標題}</option>
            }).collect_view()
        }
        </select>
    }
}
