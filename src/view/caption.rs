use leptos::*;

#[component]
pub fn Rime字幕屏(按進度顯示字幕: Signal<Option<(String, String, String)>>) -> impl IntoView {

    view! {
        <div class="text-box">
            <div class="caption">
            {
                move || 按進度顯示字幕().map(|(完成的字, 當下的字, 剩餘的字)| view! {
                    <span class="accepted">{完成的字}</span>
                    <span class="highlight">{當下的字}</span>
                    <span>{剩餘的字}</span>
                })
            }
            </div>
        </div>
    }
}
