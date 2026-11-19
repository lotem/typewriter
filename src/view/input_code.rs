use leptos::html;
use leptos::prelude::*;
use leptos_use::on_click_outside;

use crate::action::{動作, 動作給一參數};
use crate::gear::{chord::並擊對照表示, key_press::連擊對照表示};

#[derive(Clone, Copy, PartialEq)]
pub enum 回顯區佈局 {
    單欄,
    左右對照,
}

#[component]
pub fn Rime編碼回顯區(
    佈局: Signal<回顯區佈局>,
    輸入碼: Signal<String>,
    轉寫碼: Signal<Option<String>>,
    連擊對照: Signal<Option<連擊對照表示>>,
    並擊對照: Signal<Option<並擊對照表示>>,
) -> impl IntoView {
    view! {
        {move || {
            if 佈局() == 回顯區佈局::單欄 {
                if let Some(對照) = 連擊對照().filter(|對照| 對照.分段對照) {
                    let 有錯碼 = !對照.錯碼段.is_empty();
                    let 待輸列表: Vec<char> = 對照.待輸後綴.chars().collect();
                    return view! {
                        <kbd class="raw-input single-column seq-echo-track">
                            {(!對照.已對前綴.is_empty()).then_some(view! {
                                <span class="seq-matched">{對照.已對前綴}</span>
                            })}
                            {有錯碼.then_some(view! {
                                <span class="seq-error">{對照.錯碼段}</span>
                            })}
                            {對照.顯示光標.then_some(view! {
                                <span class="seq-caret">"‸"</span>
                            })}
                            {待輸列表.into_iter().enumerate().map(|(序, 字根)| {
                                let 撥霧提示 = 有錯碼 && 序 == 0;
                                view! {
                                    <span
                                        class="pending-glyph"
                                        class:peek-on-error=撥霧提示
                                    >
                                        {字根.to_string()}
                                    </span>
                                }
                            }).collect_view()}
                        </kbd>
                    }
                    .into_any();
                }
            } else if let Some(對照) = 並擊對照() {
                return view! {
                    <kbd class="raw-input left-column chord-compare">
                        <span class="seq-error">{對照.實擊並擊碼}</span>
                        {(!對照.目標並擊碼.is_empty()).then_some(view! {
                            <span class="chord-target">{對照.目標並擊碼}</span>
                        })}
                    </kbd>
                    <span class="translated-input right-column chord-compare">
                        {對照.實擊轉寫碼.map(|實拼| view! {
                            <span class="seq-error">{實拼}</span>
                        })}
                        {對照.目標轉寫碼.map(|目標拼| view! {
                            <span class="chord-target">{目標拼}</span>
                        })}
                    </span>
                }
                .into_any();
            }

            view! {
                <kbd class="raw-input"
                    class:single-column=move || { 佈局() == 回顯區佈局::單欄 }
                    class:left-column=move || { 佈局() == 回顯區佈局::左右對照 }
                >{輸入碼}</kbd>
                <Show when=move || { 佈局() == 回顯區佈局::左右對照 }>
                    <span class="translated-input right-column">{轉寫碼}</span>
                </Show>
            }
            .into_any()
        }}
    }
}

#[component]
pub fn Rime反查輸入欄(
    反查碼: Signal<Option<String>>,
    示例輸入: Signal<String>,
    反查碼變更: impl 動作給一參數<String>,
) -> impl IntoView {
    let 反查輸入欄的引用 = NodeRef::<html::Input>::new();
    let (已手動編輯, 標記已編輯) = signal(false);

    Effect::new(move |_| {
        let 初始文本 = 反查碼().unwrap_or_default();
        // 刻意不追蹤 `已手動編輯`, 僅在掛載初值或異步習題首輪就緒且用家尚未敲字時填入輸入欄
        if 已手動編輯.get_untracked() {
            return;
        }
        if let Some(輸入欄) = 反查輸入欄的引用.get() {
            輸入欄.set_value(&初始文本);
            let _ = 輸入欄.focus();
            輸入欄.select();
        }
    });

    view! {
        <input type="text" class="lookup-code"
            node_ref=反查輸入欄的引用
            placeholder={示例輸入}
            on:input=move |ev| {
                標記已編輯(true);
                let 反查碼 = event_target_value(&ev);
                反查碼變更(反查碼);
            }
        />
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum 編碼欄顯示選項 {
    無顯示,
    顯示實況,
    顯示反查,
}

#[component]
pub fn Rime編碼欄(
    顯示選項: Signal<編碼欄顯示選項>,
    輸入正確: Signal<bool>,
    輸入錯誤: Signal<bool>,
    錯碼計數: Signal<usize>,
    點擊動作: impl 動作,
    關閉輸入欄: impl 動作,
    children: Children,
) -> impl IntoView {
    let target = NodeRef::<html::Div>::new();
    let _ = on_click_outside(target, move |_| {
        關閉輸入欄();
    });

    view! {
        <div node_ref=target class="input-code"
            class:freeplay={move || 顯示選項() == 編碼欄顯示選項::顯示實況}
            class:target={move || 顯示選項() == 編碼欄顯示選項::顯示反查}
            class:success={輸入正確}
            class:error={輸入錯誤}
            class:error-even={move || 輸入錯誤() && 錯碼計數().is_multiple_of(2)}
            class:error-odd={move || 輸入錯誤() && !錯碼計數().is_multiple_of(2)}
            on:click={move |_| 點擊動作()}
        >
        {children()}
        </div>
    }
}
