use keyberon::key_code::KeyCode;
use leptos::*;
use leptos::{
    ev::{keydown, keyup, KeyboardEvent},
    logging::log,
};
use leptos_use::{use_document, use_event_listener, use_window_focus};

use crate::engine::{並擊狀態, 反查變換, 寫成並擊序列, 鍵組};
use crate::key_code::網頁鍵值轉換;
use crate::layout::{盤面選擇碼, 鍵的定義, 鍵盤矩陣};
use crate::style::樣式;

#[component]
fn RIME_鍵圖(
    鍵: &'static 鍵的定義, 目標盤面: 盤面選擇碼, 標註法: 鍵面標註法
) -> impl IntoView {
    let 是否空格 = 鍵.鍵碼 == KeyCode::Space;
    let 有效盤面 = 鍵.選擇盤面(目標盤面);
    let 是否後備盤面 = 有效盤面.is_some_and(|盤面| 盤面.0 != 目標盤面);
    let 是否空鍵 = 有效盤面.is_some_and(|盤面| 盤面.1.is_empty());
    let 刻印 = 有效盤面.map(|盤面刻印| 盤面刻印.1);
    let 鍵位提示 = move || 標註法.鍵位提示(鍵);
    let 是否落鍵 = move || 標註法.是否落鍵(鍵);
    let 是否擊中 = move || 標註法.是否擊中(鍵);
    view! {
        <div class="key"
            class:empty={是否空鍵}
            class:fallback={是否後備盤面}
            class:hint={鍵位提示}
            class:keydown={是否落鍵}
            class:pressed={是否擊中}
            class:space={是否空格}
        >
            <kbd class="label">{刻印}</kbd>
        </div>
    }
}

#[derive(Clone, Copy)]
struct 鍵面標註法 {
    目標並擊: Memo<Option<鍵組>>,
    實況並擊: ReadSignal<並擊狀態>,
}

impl 鍵面標註法 {
    fn 鍵位提示(&self, 鍵: &鍵的定義) -> bool {
        self.目標並擊
            .get()
            .is_some_and(|並擊| 並擊.contains(&鍵.鍵碼))
    }

    fn 是否落鍵(&self, 鍵: &鍵的定義) -> bool {
        self.實況並擊.with(|並擊| 並擊.實時落鍵.contains(&鍵.鍵碼))
    }

    fn 是否擊中(&self, 鍵: &鍵的定義) -> bool {
        self.實況並擊.with(|並擊| 並擊.累計擊鍵.contains(&鍵.鍵碼))
    }
}

#[component]
fn RIME_鍵盤圖(
    盤面: 盤面選擇碼,
    目標並擊: Memo<Option<鍵組>>,
    實況並擊: ReadSignal<並擊狀態>,
) -> impl IntoView {
    view! {
        <div class="board">
        { 鍵盤矩陣.iter().map(|行| view! {
            <div class="row">
            { 行.iter().map(|鍵| view! {
                <RIME_鍵圖 鍵={鍵} 目標盤面={盤面} 標註法={鍵面標註法{目標並擊, 實況並擊}}/>
            }).collect_view() }
            </div>
        }).collect_view() }
        </div>
    }
}

#[component]
pub fn RIME_打字機應用() -> impl IntoView {
    let (並擊狀態流, 並擊狀態變更) = create_signal(並擊狀態::new());

    let (反查碼, 更新反查碼) = create_signal(String::new());
    let (鍵位反查模式, 開關編碼反查輸入欄) = create_signal(false);

    let 反查輸入欄的引用 = create_node_ref::<html::Input>();
    create_effect(move |_| {
        if 鍵位反查模式() {
            if let Some(引用) = 反查輸入欄的引用() {
                let _不看結果 = 引用.on_mount(|輸入欄| {
                    let _不看結果 = 輸入欄.focus();
                });
            }
        }
    });

    let 開啓反查 = move || {
        並擊狀態變更.update(並擊狀態::重置);
        更新反查碼(String::new());
        開關編碼反查輸入欄(true);
    };
    let 關閉反查 = move || {
        開關編碼反查輸入欄(false);
    };

    let _ = use_event_listener(use_document().body(), keydown, move |evt: KeyboardEvent| {
        log!("keydown '{}' {}", &evt.key(), evt.code());
        if !鍵位反查模式() {
            並擊狀態變更.update(|並擊| 並擊.落鍵(網頁鍵值轉換(&evt.code())));
        }
    });
    let _ = use_event_listener(use_document().body(), keyup, move |evt: KeyboardEvent| {
        log!("keyup '{}' {}", &evt.key(), &evt.code());
        if !鍵位反查模式() {
            並擊狀態變更.update(|並擊| 並擊.抬鍵(網頁鍵值轉換(&evt.code())));
        }
    });

    let 鍵盤輸入焦點源 = create_selector(use_window_focus());
    create_effect(move |_| {
        if 鍵盤輸入焦點源.selected(false) {
            並擊狀態變更.update(並擊狀態::重置);
        }
    });

    let 並擊碼 = move || 並擊狀態流.with(並擊狀態::並擊序列);
    let 並擊所得拼音 = create_memo(move |_| 並擊狀態::並擊變換(&並擊碼()));

    let 反查拼音 = move || (!反查碼().is_empty()).then(反查碼);
    let 反查鍵位 = create_memo(move |_| 反查拼音().as_deref().and_then(反查變換));
    let 反查所得並擊碼 = move || 反查鍵位().as_ref().map(寫成並擊序列);

    let 顯示反查碼 = move || 反查所得並擊碼().is_some();
    let 顯示實況並擊碼 = move || !顯示反查碼() && !並擊碼().is_empty();
    let 並擊成功 = move || 反查拼音().is_some_and(|甲| 並擊所得拼音().is_some_and(|乙| 甲 == 乙));

    let 輸入碼 = move || 反查所得並擊碼().unwrap_or_else(並擊碼);
    let 拼音 = move || {
        反查拼音()
            .or_else(並擊所得拼音)
            // 加尖括弧表示拉丁文轉寫，即拼音
            .map(|拼音| format!("⟨{拼音}⟩"))
    };

    let styler_class = 樣式();
    view! { class = styler_class,
        <div class="input-code"
            class:freeplay={顯示實況並擊碼}
            class:target={顯示反查碼}
            class:success={並擊成功}
            on:click=move |_| 開啓反查()
        >
            <Show
                when=move || 鍵位反查模式()
                fallback=move || view! {
                    <kbd class="raw-input">{輸入碼}</kbd>
                    <span class="translated-input">{拼音}</span>
                }
            >
                <input type="text" class="lookup-code"
                    _ref=反查輸入欄的引用
                    on:input=move |ev| {
                        更新反查碼(event_target_value(&ev));
                    }
                    on:blur=move |_| 關閉反查()
                />
            </Show>
        </div>
        <RIME_鍵盤圖 盤面={2} 目標並擊={反查鍵位} 實況並擊={並擊狀態流}/>
    }
}
