use leptos::prelude::*;

use crate::action::動作;
use crate::gear::{layout::配列, theory::方案選項};

#[component]
pub fn Rime狀態欄(
    現行方案: Signal<方案選項>,
    已選配列: Signal<配列>,
    點擊方案: impl 動作,
    點擊配列: impl 動作,
) -> impl IntoView {
    let 方案名稱 = move || 現行方案.read().to_string();
    let 佈局名稱 = move || 已選配列.read().to_string();
    view! {
        <div class="status-bar">
            <div class="status-item" on:click=move |_| 點擊方案() title="切換輸入方案">
                <span class="status-label">方案</span>
                <span class="status-value">{方案名稱}</span>
            </div>
            <span style="color: var(--secondary-fg-color); opacity: 0.2">"|"</span>
            <div class="status-item" on:click=move |_| 點擊配列() title="切換鍵盤佈局">
                <span class="status-label">佈局</span>
                <span class="status-value">{佈局名稱}</span>
            </div>
        </div>
    }
}
