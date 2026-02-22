use leptos::prelude::*;

use crate::gear::layout::{
    功能鍵::衆功能鍵, 盤面選擇碼, 矩陣座標, 配列, 鍵盤佈局, 鍵面刻印, 鍵面映射,
};
use crate::key_code::KeyCode;

pub trait 鍵面標註法 {
    fn 鍵碼(&self) -> KeyCode;
    fn 刻印(&self) -> 鍵面刻印;
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
struct 選擇鍵面 {
    佈局: 鍵盤佈局,
    鍵碼: KeyCode,
    目標盤面: 盤面選擇碼,
    座標: 矩陣座標,
}

impl 選擇鍵面 {
    fn new(
        佈局: 鍵盤佈局, 鍵碼: KeyCode, 目標盤面: 盤面選擇碼, 座標: 矩陣座標
    ) -> Self {
        Self {
            佈局,
            鍵碼,
            目標盤面,
            座標,
        }
    }

    fn 有效盤面(&self) -> Option<(usize, 鍵面刻印)> {
        self.佈局.選擇盤面(self.目標盤面, self.座標)
    }
}

impl 鍵面標註法 for 選擇鍵面 {
    fn 鍵碼(&self) -> KeyCode {
        self.鍵碼
    }
    fn 刻印(&self) -> 鍵面刻印 {
        self.有效盤面()
            .map_or(鍵面刻印::透明, move |(_, 刻印)| 刻印)
    }
    fn 是否空鍵(&self) -> bool {
        !self
            .有效盤面()
            .is_some_and(|(_, 刻印)| matches!(刻印, 鍵面刻印::有刻 { .. }))
    }
    fn 是否後備盤面(&self) -> bool {
        self.有效盤面()
            .is_some_and(|(盤面, _)| 盤面 != self.目標盤面.頂層盤面())
    }
}

impl 鍵面標註法 for 鍵面映射 {
    fn 鍵碼(&self) -> KeyCode {
        self.鍵碼
    }
    fn 刻印(&self) -> 鍵面刻印 {
        self.刻印
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
    T: 鍵面標註法 + Copy + Send + Sync + 'static,
    U: 鍵面動態着色法 + Copy + Send + Sync + 'static,
{
    view! {
        <div class="key horizontal-box"
            class:empty={move || 標註法.read().是否空鍵()}
            class:fallback={move || 標註法.read().是否後備盤面()}
            class:function={move || 標註法.read().是否功能鍵()}
            class:space={move || 標註法.read().是否空格()}
            class:hint={move || 着色法.鍵位提示(鍵)}
            class:keydown={move || 着色法.是否落鍵(鍵)}
            class:pressed={move || 着色法.是否擊中(鍵)}
        >
            <kbd class="label secondary">{move || 標註法.read().刻印().左側刻印文字()}</kbd>
            <div class="vertical-box">
                <kbd class="label secondary">{move || 標註法.read().刻印().上方刻印文字()}</kbd>
                <kbd class="label primary">{move || 標註法.read().刻印().居中刻印文字()}</kbd>
                <kbd class="label secondary">{move || 標註法.read().刻印().下方刻印文字()}</kbd>
            </div>
            <kbd class="label secondary">{move || 標註法.read().刻印().右側刻印文字()}</kbd>
        </div>
    }
}

#[component]
pub fn Rime鍵盤圖<T>(
    配列: Signal<配列>,
    鍵盤佈局: Signal<鍵盤佈局>,
    目標盤面: ReadSignal<盤面選擇碼>,
    着色法: T,
) -> impl IntoView
where
    T: 鍵面動態着色法 + Copy + Send + Sync + 'static,
{
    view! {
        <div
            class="board"
            class=("size-60", move || 配列.read().規格() == 60)
            class=("size-30", move || 配列.read().規格() == 30)
            class:ortholinear={move || 配列.read().直列()}
            class:staggered={move || 配列.read().橫向交錯()}
            class:columnar={move || 配列.read().縱向交錯()}
            class:split={move || 配列.read().分體()}
        >
        { move || 配列.read().矩陣().iter().enumerate().map(|(行座標, 行)| view! {
            <div class="row">
            { 行.iter().enumerate()
              .filter(|(_, &鍵)| 鍵 != KeyCode::No)
              .map(|(列座標, &鍵)| {
                let 標註法 = Signal::derive(move || 選擇鍵面::new(鍵盤佈局(), 鍵, 目標盤面(), 矩陣座標(行座標, 列座標)));
                view! {
                    <Rime鍵圖 鍵={鍵} 標註法={標註法} 着色法={着色法}/>
                }
            }).collect_view() }
            </div>
        }).collect_view() }
        </div>
    }
}
