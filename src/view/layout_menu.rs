use leptos::html;
use leptos::logging::log;
use leptos::prelude::*;
use strum::IntoEnumIterator;

use crate::action::动作给一参数;
use crate::gear::layout::键盘配列;

#[component]
pub fn Rime配列选单(
    已选配列: ReadSignal<键盘配列>,
    选用配列: impl 动作给一参数<键盘配列>
) -> impl IntoView {
    let 配列选单的引用 = NodeRef::<html::Select>::new();
    let _ = Effect::new(move |_| {
        if let Some(输入栏) = 配列选单的引用.get() {
            let 选项序号: i32 = 键盘配列
                ::iter()
                .position(|键盘配列| 键盘配列 == 已选配列())
                .and_then(|序号| 序号.try_into().ok())
                .unwrap_or(-1);
            输入栏.set_selected_index(选项序号);
            let _ = 输入栏.focus();
        }
    });

    view! {
        <select class="layouts"
            node_ref=配列选单的引用
            on:change=move |ev| {
                if let Ok(选中第几项) = event_target_value(&ev).parse::<usize>() {
                    if let Some(键盘配列) = 键盘配列::iter().nth(选中第几项) {
                        log!("选用配列[{}]: {}", 选中第几项, 键盘配列);
                        选用配列(键盘配列);
                    }
                }
            }
        >
        {
            键盘配列::iter().enumerate().map(|(配列序号, 键盘配列)| view! {
                <option value={配列序号}>{键盘配列.to_string()}</option>
            }).collect_view()
        }
        </select>
    }
}
