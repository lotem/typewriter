use leptos::html::Div;
use leptos::*;
use leptos_use::on_click_outside;

#[component]
pub fn Rime編碼回顯區(
    輸入碼: Signal<String>,
    轉寫碼: Signal<Option<String>>,
) -> impl IntoView {
    view! {
        <kbd class="raw-input">{輸入碼}</kbd>
        <span class="translated-input">{轉寫碼}</span>
    }
}

#[component]
pub fn Rime反查輸入欄(
    反查碼: Signal<String>,
    示例輸入: Signal<String>,
    反查碼變更: impl Fn(String) + 'static,
) -> impl IntoView {
    let 反查輸入欄的引用 = create_node_ref::<html::Input>();
    create_render_effect(move |_| {
        if let Some(輸入欄) = 反查輸入欄的引用() {
            let _不看結果 = 輸入欄.on_mount(|輸入欄| {
                輸入欄.select();
            });
        }
    });

    view! {
        <input type="text" class="lookup-code"
            _ref=反查輸入欄的引用
            placeholder={示例輸入}
            value={反查碼}
            on:input=move |ev| {
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
    擊中目標: Signal<bool>,
    點擊動作: impl Fn() + 'static,
    關閉輸入欄: impl Fn() + Clone + 'static,
    children: Children,
) -> impl IntoView {
    let target = NodeRef::<Div>::new();
    let _ = on_click_outside(target, move |_| {
        關閉輸入欄();
    });

    view! {
        <div node_ref=target class="input-code"
            class:freeplay={move || 顯示選項() == 編碼欄顯示選項::顯示實況}
            class:target={move || 顯示選項() == 編碼欄顯示選項::顯示反查}
            class:success={擊中目標}
            on:click={move |_| 點擊動作()}
        >
        {children()}
        </div>
    }
}
