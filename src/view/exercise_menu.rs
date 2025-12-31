use leptos::html;
use leptos::logging::log;
use leptos::prelude::*;

use crate::action::动作给一参数;
use crate::drill::练习题;

#[component]
pub fn Rime练习题选单(
    预设练习题: Signal<&'static [练习题<'static>]>,
    当前题号: Signal<Option<usize>>,
    选择题号: impl 动作给一参数<usize>
) -> impl IntoView {
    let 练习题列表 = NodeRef::<html::Select>::new();
    let _ = Effect::new(move |_| {
        if let Some(输入栏) = 练习题列表.get() {
            let 选项序号 = 当前题号()
                .and_then(|题号| 题号.try_into().ok())
                .unwrap_or(-1);
            输入栏.set_selected_index(选项序号);
            let _ = 输入栏.focus();
        }
    });

    view! {
        <select class="exercises"
            node_ref=练习题列表
            on:change=move |ev| {
                let 题号 = event_target_value(&ev);
                log!("题号: {}", 题号);
                if let Ok(题号) = 题号.parse::<usize>() {
                    if 题号 < 预设练习题().len() {
                        选择题号(题号);
                    }
                }
            }
        >
        {
            预设练习题().iter().enumerate().map(|(题号, 题目)| view! {
                <option value={题号}>{题目.标题}</option>
            }).collect_view()
        }
        </select>
    }
}
