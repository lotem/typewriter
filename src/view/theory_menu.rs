use leptos::html;
use leptos::logging::log;
use leptos::prelude::*;

use crate::action::动作给一参数;
use crate::gear::theory::{ 方案列表, 方案选项 };

#[component]
pub fn Rime方案选单(
    当前方案: ReadSignal<方案选项>,
    选中方案: impl 动作给一参数<方案选项>
) -> impl IntoView {
    let 方案选单的引用 = NodeRef::<html::Select>::new();
    let _ = Effect::new(move |_| {
        if let Some(输入栏) = 方案选单的引用.get() {
            let 选项序号: i32 = 方案列表
                .iter()
                .position(|&(方案, _)| 方案 == 当前方案())
                .and_then(|序号| 序号.try_into().ok())
                .unwrap_or(-1);
            输入栏.set_selected_index(选项序号);
            let _ = 输入栏.focus();
        }
    });

    view! {
        <select class="theories"
            node_ref=方案选单的引用
            on:change=move |ev| {
                if let Ok(选中第几项) = event_target_value(&ev).parse::<usize>() {
                    if let Some(&(方案, 方案定义)) = 方案列表.get(选中第几项) {
                        log!("选中方案[{}]: {}", 选中第几项, 方案定义.名称);
                        选中方案(方案);
                    }
                }
            }
        >
        {
            方案列表.iter().enumerate().map(|(方案序号, (_, 方案定义))| view! {
                <option value={方案序号}>{方案定义.名称}</option>
            }).collect_view()
        }
        </select>
    }
}
