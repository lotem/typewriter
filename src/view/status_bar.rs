use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;

use crate::action::動作;
use crate::gear::{layout::配列, mode::工作模式, theory::方案選項};
use crate::introspect::{內觀總樞, 內觀附欄, 響應角色};

fn 釋放按鈕焦點(事件: &leptos::ev::MouseEvent) {
    if let Some(元素) = 事件
        .current_target()
        .and_then(|目標| 目標.dyn_into::<web_sys::HtmlElement>().ok())
    {
        let _ = 元素.blur();
    }
}

#[component]
pub fn Rime狀態欄(
    現行方案: Memo<方案選項>,
    已選配列: Signal<配列>,
    現行工作模式: ReadSignal<工作模式>,
    內觀: 內觀總樞,
    點擊方案: impl 動作,
    點擊配列: impl 動作,
) -> impl IntoView {
    let 方案名稱 = move || 現行方案.read().to_string();
    let 佈局名稱 = move || 已選配列.read().to_string();
    let 視圖名稱 = move || {
        if 內觀.開啓.get() {
            "內觀視圖"
        } else {
            "常規視圖"
        }
    };

    view! {
        <header class="top-toolbar">
            // 左上翼: [輸入方案] 與 [佈局] 基座設定 (`mousedown` 阻止按鈕奪取鍵盤焦點, 避免並擊空格誤觸)
            <div class="top-wing left">
                <button
                    type="button"
                    tabindex="-1"
                    class="top-pill"
                    class:active=move || 現行工作模式() == 工作模式::選擇輸入方案
                    title="切換輸入方案 (Ctrl+`)"
                    on:mousedown=move |事件| 事件.prevent_default()
                    on:click=move |事件| {
                        釋放按鈕焦點(&事件);
                        點擊方案();
                    }
                >
                    <span class="pill-label">"方案"</span>
                    <span class="pill-value">{方案名稱}</span>
                    <kbd class="pill-kbd">"Ctrl+`"</kbd>
                </button>
                <button
                    type="button"
                    tabindex="-1"
                    class="top-pill"
                    class:active=move || 現行工作模式() == 工作模式::選擇配列
                    title="切換鍵盤佈局 (Ctrl+~ 即 Ctrl+Shift+`)"
                    on:mousedown=move |事件| 事件.prevent_default()
                    on:click=move |事件| {
                        釋放按鈕焦點(&事件);
                        點擊配列();
                    }
                >
                    <span class="pill-label">"佈局"</span>
                    <span class="pill-value">{佈局名稱}</span>
                    <kbd class="pill-kbd">"Ctrl+~"</kbd>
                </button>
            </div>

            // 中央翼: 僅在 [內觀視圖] 非錄屏態下輕量呈現水文五象角色聚光濾鏡
            <Show when=move || 內觀.開啓.get() && !內觀.錄屏模式.get()>
                <div class="top-wing center" title="懸停或點擊可按水文角色單獨聚光顯影">
                    {響應角色::全部角色.into_iter().map(|角色| {
                        let 已選中 = move || 內觀.聚光角色.get() == Some(角色);
                        view! {
                            <button
                                type="button"
                                tabindex="-1"
                                class="top-pill role-chip"
                                class:active=已選中
                                title=角色.釋義()
                                on:mousedown=move |事件| 事件.prevent_default()
                                on:mouseenter=move |_| 內觀.聚光角色.set(Some(角色))
                                on:mouseleave=move |_| 內觀.聚光角色.set(None)
                                on:click=move |事件| {
                                    釋放按鈕焦點(&事件);
                                    內觀.聚光角色.update(|當前| {
                                        *當前 = if *當前 == Some(角色) { None } else { Some(角色) };
                                    });
                                }
                            >
                                <span>{角色.符號()}</span>
                                <span>{角色.名稱()}</span>
                            </button>
                        }
                    }).collect_view()}
                    <Show when=move || 內觀.合詠活躍.get()>
                        <span class="xray-resonance-pill">"✦ 合詠通流"</span>
                    </Show>
                </div>
            </Show>

            // 右上翼: 最右固定錨定 [視圖] 總開關, [內觀] 專屬選項緊貼其左鄰展開
            <div class="top-wing right">
                <Show when=move || 內觀.開啓.get()>
                    <button
                        type="button"
                        tabindex="-1"
                        class="top-pill"
                        class:active=move || 內觀.是否顯示對照鍵盤()
                        title="於下方固定懸空顯示無刻定位縮微鍵盤, 與 [待鑄藍圖] 互斥"
                        on:mousedown=move |事件| 事件.prevent_default()
                        on:click=move |事件| {
                            釋放按鈕焦點(&事件);
                            內觀.切換附欄(內觀附欄::同屏對照);
                        }
                    >
                        <span class="pill-value">"同屏對照"</span>
                    </button>
                    <button
                        type="button"
                        tabindex="-1"
                        class="top-pill"
                        class:active=move || 內觀.是否顯示藍圖()
                        title="顯隱底部尚未實裝的 [七境] 演進藍圖節點, 與 [同屏對照] 互斥"
                        on:mousedown=move |事件| 事件.prevent_default()
                        on:click=move |事件| {
                            釋放按鈕焦點(&事件);
                            內觀.切換附欄(內觀附欄::待鑄藍圖);
                        }
                    >
                        <span class="pill-value">"待鑄藍圖"</span>
                    </button>
                    <button
                        type="button"
                        tabindex="-1"
                        class="top-pill"
                        class:active=move || 內觀.錄屏模式.get()
                        title="隱藏工具欄與邊框, 進入或退出全屏錄屏演示態 (Ctrl+| 即 Ctrl+Shift+\\)"
                        on:mousedown=move |事件| 事件.prevent_default()
                        on:click=move |事件| {
                            釋放按鈕焦點(&事件);
                            內觀.切換錄屏();
                        }
                    >
                        <span class="pill-value">"全屏錄屏"</span>
                        <kbd class="pill-kbd">"Ctrl+|"</kbd>
                    </button>
                </Show>

                <button
                    type="button"
                    tabindex="-1"
                    class="top-pill view-switch-pill"
                    class:active=move || 內觀.開啓.get()
                    title="切換 [常規視圖] / [內觀視圖] (Ctrl+\\)"
                    on:mousedown=move |事件| 事件.prevent_default()
                    on:click=move |事件| {
                        釋放按鈕焦點(&事件);
                        if 內觀.錄屏模式.get_untracked() {
                            內觀.錄屏模式.set(false);
                        }
                        內觀.切換視圖();
                    }
                >
                    <span class="pill-label">"視圖"</span>
                    <span class="pill-value">{視圖名稱}</span>
                    <kbd class="pill-kbd">"Ctrl+\\"</kbd>
                </button>
            </div>
        </header>
    }
}
