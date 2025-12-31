use leptos::prelude::*;

use crate::gear::caption::字幕表示;

#[component]
pub fn Rime字幕屏(
    是否显示光标: Signal<bool>,
    按进度显示字幕: Signal<Option<字幕表示>>
) -> impl IntoView {
    view! {
        <div class="text-box">
            <div class="caption">
            {
                move || 按进度显示字幕().map(|字幕| view! {
                    <span class="accepted">{字幕.已完成}</span>
                    <span class="highlight" class:cursor={是否显示光标}>{字幕.指标文字}</span>
                    <span>{字幕.未完成}</span>
                })
            }
            </div>
        </div>
    }
}
