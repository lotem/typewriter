use leptos::html;
use leptos::logging::log;
use leptos::prelude::*;
use strum::IntoEnumIterator;

use crate::action::動作給一參數;
use crate::gear::layout::配列;

#[component]
pub fn Rime配列選單(
    已選配列: ReadSignal<配列>,
    選用配列: impl 動作給一參數<配列>,
) -> impl IntoView {
    let 配列選單的引用 = NodeRef::<html::Select>::new();
    let _ = Effect::new(move |_| {
        if let Some(輸入欄) = 配列選單的引用.get() {
            let 選項序號: i32 = 配列::iter()
                .position(|配列| 配列 == 已選配列())
                .and_then(|序號| 序號.try_into().ok())
                .unwrap_or(-1);
            輸入欄.set_selected_index(選項序號);
            let _ = 輸入欄.focus();
        }
    });

    view! {
        <select class="layouts"
            node_ref=配列選單的引用
            on:change=move |ev| {
                if let Ok(選中第幾項) = event_target_value(&ev).parse::<usize>() {
                    if let Some(配列) = 配列::iter().nth(選中第幾項) {
                        log!("選用配列[{}]: {}", 選中第幾項, 配列);
                        選用配列(配列);
                    }
                }
            }
        >
        {
            配列::iter().enumerate().map(|(配列序號, 配列)| view! {
                <option value={配列序號}>{配列.to_string()}</option>
            }).collect_view()
        }
        </select>
    }
}
