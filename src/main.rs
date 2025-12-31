#![feature(type_alias_impl_trait)]
#![allow(confusable_idents)]
#![allow(uncommon_codepoints)]

use leptos::prelude::*;

mod action; // 不清楚
mod app; // 主程序吧
mod definition; // 键盘有关的定义
mod drill; // 练习文章
mod engine;
mod gear; // 核心组件
mod key_code; // 键盘映射
mod rime2_api;
mod rime2_api_deprecated;
mod spelling_algebra; // 拼写运算
mod theory; // 输入方案，应该叫schema的
mod view; // 视图

use app::Rime打字机应用;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Rime打字机应用/> });
}
