use leptos::html;
use leptos::logging::log;
use leptos::prelude::*;

use crate::action::動作給一參數;
use crate::gear::theory::{方案選單, 方案選項};

#[component]
pub fn Rime方案選單(
    現行方案: ReadSignal<方案選項>,
    選中方案: impl 動作給一參數<方案選項>,
) -> impl IntoView {
    let 方案選單的引用 = NodeRef::<html::Select>::new();
    let _ = Effect::new(move |_| {
        if let Some(輸入欄) = 方案選單的引用.get() {
            let 選項序號: i32 = 方案選單
                .iter()
                .position(|&(方案, _)| 方案 == 現行方案())
                .and_then(|序號| 序號.try_into().ok())
                .unwrap_or(-1);
            輸入欄.set_selected_index(選項序號);
            let _ = 輸入欄.focus();
        }
    });

    view! {
        <select class="theories"
            node_ref=方案選單的引用
            on:change=move |ev| {
                if let Ok(選中第幾項) = event_target_value(&ev).parse::<usize>() {
                    if let Some(&(方案, 方案定義)) = 方案選單.get(選中第幾項) {
                        log!("選中方案[{}]: {}", 選中第幾項, 方案定義.名稱);
                        選中方案(方案);
                    }
                }
            }
        >
        {
            方案選單.iter().enumerate().map(|(方案序號, (_, 方案定義))| view! {
                <option value={方案序號}>{方案定義.名稱}</option>
            }).collect_view()
        }
        </select>
    }
}
