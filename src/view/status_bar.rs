use leptos::prelude::*;

use crate::action::动作;
use crate::gear::{ layout::键盘配列, theory::{ 方案列表, 方案选项 } };

#[component]
pub fn Rime状态栏(
    当前方案: ReadSignal<方案选项>,
    已选配列: ReadSignal<键盘配列>,
    点击方案: impl 动作,
    点击配列: impl 动作
) -> impl IntoView {
    let 方案名称 = move || {
        let 选中项 = 当前方案.read();
        方案列表.iter()
            .find(|某项| 某项.0 == *选中项)
            .map_or("无方案", |此项| 此项.1.名称)
    };
    view! {
        <div class="status-bar">
            <div class="status-item" on:click=move |_| 点击方案() title="切换输入方案">
                <span class="status-label">方案</span>
                <span class="status-value">{方案名称}</span>
            </div>
            <span style="color: var(--secondary-fg-color); opacity: 0.2">"|"</span>
            <div class="status-item" on:click=move |_| 点击配列() title="切换键盘布局">
                <span class="status-label">布局</span>
                <span class="status-value">{已选配列.read().to_string()}</span>
            </div>
        </div>
    }
}
