use leptos::prelude::*;

mod action;
mod app;
mod definition;
mod drills;
mod engine;
mod gear;
mod key_code;
mod layout;
mod spelling_algebra;
mod theory;
mod view;

use app::Rime打字機應用;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Rime打字機應用/> });
}
