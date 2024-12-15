use leptos::*;

mod action;
mod alphabet;
mod app;
mod combo_pinyin;
mod drills;
mod engine;
mod gear;
mod key_code;
mod layout;
mod spelling_algebra;
mod style;
mod view;

use app::Rime打字機應用;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Rime打字機應用/> });
}
