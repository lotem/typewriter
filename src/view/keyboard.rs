use keyberon::key_code::KeyCode;
use leptos::prelude::*;

use crate::layout::{
    功能鍵::衆功能鍵, 打字機鍵盤佈局, 盤面選擇碼, 鍵面刻印, 鍵面映射
};

pub trait 鍵面標註法<'a> {
    fn 鍵碼(&self) -> KeyCode;
    fn 刻印(&self) -> Option<&'a str>;
    fn 是否空鍵(&self) -> bool;
    fn 是否後備盤面(&self) -> bool;
    fn 是否功能鍵(&self) -> bool {
        衆功能鍵.iter().any(|功能鍵| 功能鍵.鍵碼 == self.鍵碼())
    }
    fn 是否空格(&self) -> bool {
        self.鍵碼() == KeyCode::Space
    }
}

pub trait 鍵面動態着色法 {
    fn 鍵位提示(&self, 鍵: KeyCode) -> bool;
    fn 是否落鍵(&self, 鍵: KeyCode) -> bool;
    fn 是否擊中(&self, 鍵: KeyCode) -> bool;
}

#[derive(Clone, Copy)]
struct 選擇鍵面<'a> {
    鍵碼: KeyCode,
    目標盤面: 盤面選擇碼,
    有效盤面: Option<(usize, 鍵面刻印<'a>)>,
}

impl 選擇鍵面<'_> {
    fn new(鍵碼: KeyCode, 目標盤面: 盤面選擇碼) -> Self {
        Self {
            鍵碼,
            目標盤面,
            有效盤面: 打字機鍵盤佈局.選擇盤面(鍵碼, 目標盤面),
        }
    }
}

impl<'a> 鍵面標註法<'a> for 選擇鍵面<'a> {
    fn 鍵碼(&self) -> KeyCode {
        self.鍵碼
    }
    fn 刻印(&self) -> Option<&'a str> {
        self.有效盤面.and_then(|(_, 刻印)| match 刻印 {
            鍵面刻印::有刻(刻印文字) => Some(刻印文字),
            _ => None,
        })
    }
    fn 是否空鍵(&self) -> bool {
        self.有效盤面
            .is_some_and(|(_, 刻印)| !matches!(刻印, 鍵面刻印::有刻(_)))
    }
    fn 是否後備盤面(&self) -> bool {
        self.有效盤面
            .is_some_and(|(盤面, _)| 盤面 != self.目標盤面.頂層盤面())
    }
}

impl<'a> 鍵面標註法<'a> for 鍵面映射<'a> {
    fn 鍵碼(&self) -> KeyCode {
        self.鍵碼
    }
    fn 刻印(&self) -> Option<&'a str> {
        self.刻印.刻印文字()
    }
    fn 是否空鍵(&self) -> bool {
        self.刻印 == 鍵面刻印::無刻
    }
    fn 是否後備盤面(&self) -> bool {
        self.刻印 == 鍵面刻印::透明
    }
}

#[component]
pub fn Rime鍵圖<T, U>(鍵: KeyCode, 標註法: Signal<T>, 着色法: U) -> impl IntoView
where
    T: 鍵面標註法<'static> + Copy + Send + Sync + 'static,
    U: 鍵面動態着色法 + Copy + Send + Sync + 'static,
{
    view! {
        <div class="key"
            class:empty={move || 標註法.read().是否空鍵()}
            class:fallback={move || 標註法.read().是否後備盤面()}
            class:function={move || 標註法.read().是否功能鍵()}
            class:space={move || 標註法.read().是否空格()}
            class:hint={move || 着色法.鍵位提示(鍵)}
            class:keydown={move || 着色法.是否落鍵(鍵)}
            class:pressed={move || 着色法.是否擊中(鍵)}
        >
            <kbd class="label">{move || 標註法.read().刻印()}</kbd>
        </div>
    }
}

#[component]
pub fn Rime鍵盤圖<T>(目標盤面: Signal<盤面選擇碼>, 着色法: T) -> impl IntoView
where
    T: 鍵面動態着色法 + Copy + Send + Sync + 'static,
{
    view! {
        <div class="board ortholinear split">
        { 打字機鍵盤佈局.矩陣.iter().map(|行| view! {
            <div class="row">
            { 行.iter().map(|&鍵| {
                let 標註法 = Signal::derive(move || 選擇鍵面::new(鍵, 目標盤面()));
                view! {
                    <Rime鍵圖 鍵={鍵} 標註法={標註法} 着色法={着色法}/>
                }
            }).collect_view() }
            </div>
        }).collect_view() }
        </div>
    }
}
