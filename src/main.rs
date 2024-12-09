use leptos::*;

mod action;
mod alphabet;
mod app;
mod assignment;
mod caption;
mod chord;
mod combo_pinyin;
mod drills;
mod engine;
mod input;
mod key_code;
mod key_press;
mod layout;
mod mode;
mod spelling_algebra;
mod style;
mod theory;
mod view;

use app::Rime打字機應用;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Rime打字機應用/> });
}
