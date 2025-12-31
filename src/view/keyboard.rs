use leptos::prelude::*;

use crate::gear::layout::{
    功能键::众功能键, 盘面序号, 矩阵坐标, 键盘布局, 键盘配列, 键面刻印, 键面映射,
};
use crate::key_code::KeyCode;

pub trait 键面标注法 {
    fn 键码(&self) -> KeyCode;
    fn 刻印(&self) -> 键面刻印;
    fn 是否空键(&self) -> bool;
    fn 是否后备盘面(&self) -> bool;
    fn 是否功能键(&self) -> bool {
        众功能键.iter().any(|功能键| 功能键.键码 == self.键码())
    }
    fn 是否空格(&self) -> bool {
        self.键码() == KeyCode::Space
    }
}

pub trait 键面动态着色法 {
    fn 键位提示(&self, 键: KeyCode) -> bool;
    fn 是否落键(&self, 键: KeyCode) -> bool;
    fn 是否击中(&self, 键: KeyCode) -> bool;
}

#[derive(Clone, Copy)]
struct 选择键面 {
    布局: 键盘布局,
    键码: KeyCode,
    目标盘面: 盘面序号,
    坐标: 矩阵坐标,
}

impl 选择键面 {
    fn new(
        布局: 键盘布局, 键码: KeyCode, 目标盘面: 盘面序号, 坐标: 矩阵坐标
    ) -> Self {
        Self {
            布局,
            键码,
            目标盘面,
            坐标,
        }
    }

    fn 有效盘面(&self) -> Option<(usize, 键面刻印)> {
        self.布局.选择盘面(self.目标盘面, self.坐标)
    }
}

impl 键面标注法 for 选择键面 {
    fn 键码(&self) -> KeyCode {
        self.键码
    }
    fn 刻印(&self) -> 键面刻印 {
        self.有效盘面()
            .map_or(键面刻印::透明, move |(_, 刻印)| 刻印)
    }
    fn 是否空键(&self) -> bool {
        !self
            .有效盘面()
            .is_some_and(|(_, 刻印)| matches!(刻印, 键面刻印::有刻 { .. }))
    }
    fn 是否后备盘面(&self) -> bool {
        self.有效盘面()
            .is_some_and(|(盘面, _)| 盘面 != self.目标盘面.顶层盘面())
    }
}

impl 键面标注法 for 键面映射 {
    fn 键码(&self) -> KeyCode {
        self.键码
    }
    fn 刻印(&self) -> 键面刻印 {
        self.刻印
    }
    fn 是否空键(&self) -> bool {
        self.刻印 == 键面刻印::无刻
    }
    fn 是否后备盘面(&self) -> bool {
        self.刻印 == 键面刻印::透明
    }
}

#[component]
pub fn Rime键图<T, U>(键: KeyCode, 标注法: Signal<T>, 着色法: U) -> impl IntoView
where
    T: 键面标注法 + Copy + Send + Sync + 'static,
    U: 键面动态着色法 + Copy + Send + Sync + 'static,
{
    view! {
        <div class="key horizontal-box"
            class:empty={move || 标注法.read().是否空键()}
            class:fallback={move || 标注法.read().是否后备盘面()}
            class:function={move || 标注法.read().是否功能键()}
            class:space={move || 标注法.read().是否空格()}
            class:hint={move || 着色法.键位提示(键)}
            class:keydown={move || 着色法.是否落键(键)}
            class:pressed={move || 着色法.是否击中(键)}
        >
            <kbd class="label secondary">{move || 标注法.read().刻印().左侧刻印文字()}</kbd>
            <div class="vertical-box">
                <kbd class="label secondary">{move || 标注法.read().刻印().上方刻印文字()}</kbd>
                <kbd class="label primary">{move || 标注法.read().刻印().居中刻印文字()}</kbd>
                <kbd class="label secondary">{move || 标注法.read().刻印().下方刻印文字()}</kbd>
            </div>
            <kbd class="label secondary">{move || 标注法.read().刻印().右侧刻印文字()}</kbd>
        </div>
    }
}

#[component]
pub fn Rime键盘图<T>(
    键盘配列: ReadSignal<键盘配列>,
    键盘布局: Signal<键盘布局>,
    目标盘面: Signal<盘面序号>,
    着色法: T,
) -> impl IntoView
where
    T: 键面动态着色法 + Copy + Send + Sync + 'static,
{
    view! {
        <div
            class="board"
            class=("size-60", move || 键盘配列.read().规格() == 60)
            class=("size-30", move || 键盘配列.read().规格() == 30)
            class:ortholinear={move || 键盘配列.read().直列()}
            class:split={move || 键盘配列.read().分体()}
            class:staggered={move || 键盘配列.read().横向交错()}
        >
        { move || 键盘配列.read().矩阵().iter().enumerate().map(|(行座标, 行)| view! {
            <div class="row">
            { 行.iter().enumerate()
              .filter(|(_, &键)| 键 != KeyCode::No)
              .map(|(列座标, &键)| {
                let 标注法 = Signal::derive(move || 选择键面::new(键盘布局(), 键, 目标盘面(), 矩阵坐标(行座标, 列座标)));
                view! {
                    <Rime键图 键={键} 标注法={标注法} 着色法={着色法}/>
                }
            }).collect_view() }
            </div>
        }).collect_view() }
        </div>
    }
}
