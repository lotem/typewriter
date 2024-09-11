use leptos::*;

mod combo_pinyin;
mod layout;

use combo_pinyin::並擊變換;

#[component]
fn Rime_鍵盤視圖() -> impl IntoView {
    view! {}
}

#[component]
fn Rime_打字機應用視圖() -> impl IntoView {
    let 輸入碼 = "";
    view! {
        <div>
            <kbd>{輸入碼}</kbd>
        </div>
        <Rime_鍵盤視圖/>
    }
}

fn main() {
    mount_to_body(|| view! { <Rime_打字機應用視圖/> });
}
