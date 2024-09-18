use keyberon::key_code::KeyCode;
use leptos::*;
use leptos::{
    ev::{keydown, keyup, KeyboardEvent},
    logging::log,
};
use leptos_use::{use_document, use_event_listener, use_window_focus};

mod engine;
mod key_code;
mod layout;
mod style;

use engine::並擊狀態;
use key_code::網頁鍵值轉換;
use layout::{盤面選擇碼, 鍵的定義, 鍵盤矩陣};
use style::樣式;

#[component]
fn RIME_鍵圖(
    鍵: &'static 鍵的定義,
    目標盤面: 盤面選擇碼,
    並擊: ReadSignal<並擊狀態>,
) -> impl IntoView {
    let 是否空格 = 鍵.鍵碼 == KeyCode::Space;
    let 有效盤面 = 鍵.選擇盤面(目標盤面);
    let 是否後備盤面 = 有效盤面.is_some_and(|盤面| 盤面.0 != 目標盤面);
    let 是否空鍵 = 有效盤面.is_some_and(|盤面| 盤面.1.is_empty());
    let 刻印 = 有效盤面.map(|盤面刻印| 盤面刻印.1);
    let 是否落鍵 = move || 並擊.with(|並擊| 並擊.實時落鍵.contains(&鍵.鍵碼));
    let 是否擊中 = move || 並擊.with(|並擊| 並擊.累計擊鍵.contains(&鍵.鍵碼));
    view! {
        <div class="key"
            class:empty={是否空鍵}
            class:fallback={是否後備盤面}
            class:keydown={是否落鍵}
            class:pressed={是否擊中}
            class:space={是否空格}
        >
            <kbd class="label">{刻印}</kbd>
        </div>
    }
}

#[component]
fn RIME_鍵盤圖(盤面: 盤面選擇碼, 並擊: ReadSignal<並擊狀態>) -> impl IntoView {
    view! {
        <div class="board">
        {鍵盤矩陣.iter().map(|行| view! {
            <div class="row">
            { 行.iter().map(|鍵| view! {
                <RIME_鍵圖 鍵={鍵} 目標盤面={盤面} 並擊={並擊}/>
            }).collect_view() }
            </div>
        }).collect_view() }
        </div>
    }
}

#[component]
fn RIME_打字機應用() -> impl IntoView {
    let (並擊狀態流, 並擊狀態變更) = create_signal(並擊狀態::new());

    let _ = use_event_listener(use_document().body(), keydown, move |evt: KeyboardEvent| {
        log!("keydown '{}' {}", &evt.key(), evt.code());
        並擊狀態變更.update(|並擊| 並擊.落鍵(網頁鍵值轉換(&evt.code())));
    });
    let _ = use_event_listener(use_document().body(), keyup, move |evt: KeyboardEvent| {
        log!("keyup '{}' {}", &evt.key(), &evt.code());
        並擊狀態變更.update(|並擊| 並擊.抬鍵(網頁鍵值轉換(&evt.code())));
    });

    let 鍵盤輸入焦點源 = create_selector(use_window_focus());
    create_effect(move |_| {
        if 鍵盤輸入焦點源.selected(false) {
            並擊狀態變更.update(並擊狀態::重置);
        }
    });

    let 輸入碼 = move || 並擊狀態流.with(並擊狀態::並擊序列);

    let _ = watch(
        輸入碼,
        move |新輸入碼, _舊輸入碼, _| {
            log!("輸入碼: {}", 新輸入碼);
        },
        false,
    );

    let 拼音 = move || 並擊狀態::並擊變換(&輸入碼());

    let styler_class = 樣式();
    view! { class = styler_class,
        <div class="input-code">
            <kbd class="raw-input">{輸入碼}</kbd>
            <span class="translated-input">{拼音}</span>
        </div>
        <RIME_鍵盤圖 盤面={2} 並擊={並擊狀態流}/>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <RIME_打字機應用/> });
}
