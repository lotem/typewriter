use leptos::html;
use leptos::prelude::*;
use leptos_use::on_click_outside;

use crate::action::{ 动作给一参数, 动作 };

#[derive(Clone, Copy, PartialEq)]
pub enum 回显区布局 {
    单栏,
    左右对照,
}

#[component]
pub fn Rime编码回显区(
    布局: Signal<回显区布局>,
    输入码: Signal<String>,
    转写码: Signal<Option<String>>
) -> impl IntoView {
    view! {
        <kbd class="raw-input"
            class:single-column=move || { 布局() == 回显区布局::单栏 }
            class:left-column=move || { 布局() == 回显区布局::左右对照 }
        >{输入码}</kbd>
        <Show when=move || { 布局() == 回显区布局::左右对照 }>
            <span class="translated-input right-column">{转写码}</span>
        </Show>
    }
}

#[component]
pub fn Rime反查输入栏(
    反查码: Signal<Option<String>>,
    示例输入: Signal<String>,
    反查码变更: impl 动作给一参数<String>
) -> impl IntoView {
    let 反查输入栏的引用 = NodeRef::<html::Input>::new();
    反查输入栏的引用.on_load(|输入栏| {
        输入栏.select();
    });

    view! {
        <input type="text" class="lookup-code"
            node_ref=反查输入栏的引用
            placeholder={示例输入}
            value={反查码}
            on:input=move |ev| {
                let 反查码 = event_target_value(&ev);
                反查码变更(反查码);
            }
        />
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum 编码栏显示选项 {
    无显示,
    显示实况,
    显示反查,
}

#[component]
pub fn Rime编码栏(
    显示选项: Signal<编码栏显示选项>,
    输入正确: Signal<bool>,
    点击动作: impl 动作,
    关闭输入栏: impl 动作,
    children: Children
) -> impl IntoView {
    let target = NodeRef::<html::Div>::new();
    let _ = on_click_outside(target, move |_| {
        关闭输入栏();
    });

    view! {
        <div node_ref=target class="input-code"
            class:freeplay={move || 显示选项() == 编码栏显示选项::显示实况}
            class:target={move || 显示选项() == 编码栏显示选项::显示反查}
            class:success={输入正确}
            on:click={move |_| 点击动作()}
        >
        {children()}
        </div>
    }
}
