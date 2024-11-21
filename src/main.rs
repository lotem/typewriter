use leptos::*;

mod app;
mod assignment;
mod drills;
mod engine;
mod key_code;
mod layout;
mod style;
mod view;

use app::Rime打字機應用;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Rime打字機應用/> });
}
