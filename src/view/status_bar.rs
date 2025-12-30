use leptos::prelude::*;

use crate::action::動作;
use crate::gear::{
    layout::配列,
    theory::{方案選單, 方案選項},
};

#[component]
pub fn Rime狀態欄(
    現行方案: ReadSignal<方案選項>,
    已選配列: ReadSignal<配列>,
    點擊方案: impl 動作,
    點擊配列: impl 動作,
) -> impl IntoView {
    let 方案名稱 = move || {
        let 選中項 = 現行方案.read();
        方案選單
            .iter()
            .find(|某項| 某項.0 == *選中項)
            .map_or("無方案", |此項| 此項.1.名稱)
    };
    view! {
        <div class="status-bar">
            <div class="status-item" on:click=move |_| 點擊方案() title="切換輸入方案">
                <span class="status-label">方案</span>
                <span class="status-value">{方案名稱}</span>
            </div>
            <span style="color: var(--secondary-fg-color); opacity: 0.2">"|"</span>
            <div class="status-item" on:click=move |_| 點擊配列() title="切換鍵盤佈局">
                <span class="status-label">佈局</span>
                <span class="status-value">{move || 已選配列.read().to_string()}</span>
            </div>
        </div>
    }
}
