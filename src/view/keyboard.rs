use keyberon::key_code::KeyCode;
use leptos::*;

use crate::layout::{盤面刻印, 盤面選擇碼, 鍵的定義, 鍵盤矩陣};

pub trait 鍵面標註依據 {
    fn 刻印(&self) -> Option<String>;
    fn 是否空鍵(&self) -> bool;
    fn 是否後備盤面(&self) -> bool;
    fn 是否功能鍵(&self) -> bool;
    fn 是否空格(&self) -> bool;
}

pub trait 鍵面標註法 {
    fn 標註(&self, 鍵: &'static 鍵的定義) -> impl 鍵面標註依據 + Copy + 'static;
}

pub trait 鍵面動態着色法 {
    fn 鍵位提示(&self, 鍵: &鍵的定義) -> bool;
    fn 是否落鍵(&self, 鍵: &鍵的定義) -> bool;
    fn 是否擊中(&self, 鍵: &鍵的定義) -> bool;
}

#[derive(Clone, Copy)]
struct 選擇盤面 {
    鍵: &'static 鍵的定義,
    目標盤面: 盤面選擇碼,
}

impl 鍵面標註法 for 盤面選擇碼 {
    fn 標註(&self, 鍵: &'static 鍵的定義) -> impl 鍵面標註依據 + Copy + 'static {
        let 目標盤面 = *self;
        選擇盤面 { 鍵, 目標盤面 }
    }
}

impl 選擇盤面 {
    fn 有效盤面(&self) -> Option<盤面刻印> {
        self.鍵.選擇盤面(self.目標盤面)
    }
}

impl 鍵面標註依據 for 選擇盤面 {
    fn 刻印(&self) -> Option<String> {
        self.有效盤面().map(|盤面刻印| 盤面刻印.1.to_owned())
    }
    fn 是否空鍵(&self) -> bool {
        self.有效盤面().is_some_and(|盤面| 盤面.1.is_empty())
    }
    fn 是否後備盤面(&self) -> bool {
        self.有效盤面()
            .is_some_and(|盤面| 盤面.0 != self.目標盤面.0)
    }
    fn 是否功能鍵(&self) -> bool {
        [
            KeyCode::Escape,
            KeyCode::Tab,
            KeyCode::BSpace,
            KeyCode::Enter,
        ]
        .contains(&self.鍵.鍵碼)
    }
    fn 是否空格(&self) -> bool {
        self.鍵.鍵碼 == KeyCode::Space
    }
}

#[component]
pub fn Rime鍵圖<T, U>(鍵: &'static 鍵的定義, 標註法: T, 着色法: U) -> impl IntoView
where
    T: 鍵面標註法 + Copy + 'static,
    U: 鍵面動態着色法 + Copy + 'static,
{
    let 鍵面標註 = 標註法.標註(鍵);
    view! {
        <div class="key"
            class:empty={move || 鍵面標註.是否空鍵()}
            class:fallback={move || 鍵面標註.是否後備盤面()}
            class:function={move || 鍵面標註.是否功能鍵()}
            class:space={move || 鍵面標註.是否空格()}
            class:hint={move || 着色法.鍵位提示(鍵)}
            class:keydown={move || 着色法.是否落鍵(鍵)}
            class:pressed={move || 着色法.是否擊中(鍵)}
        >
            <kbd class="label">{move || 鍵面標註.刻印()}</kbd>
        </div>
    }
}

#[component]
pub fn Rime鍵盤圖<T, U>(標註法: T, 着色法: U) -> impl IntoView
where
    T: 鍵面標註法 + Copy + 'static,
    U: 鍵面動態着色法 + Copy + 'static,
{
    view! {
        <div class="board">
        { 鍵盤矩陣.iter().map(|行| view! {
            <div class="row">
            { 行.iter().map(|鍵| {
                view! {
                    <Rime鍵圖 鍵={鍵} 標註法={標註法} 着色法={着色法}/>
                }
            }).collect_view() }
            </div>
        }).collect_view() }
        </div>
    }
}
