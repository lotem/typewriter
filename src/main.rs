use leptos::*;

mod app;
mod drills;
mod engine;
mod key_code;
mod layout;
mod style;

use app::RIME_打字機應用;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <RIME_打字機應用/> });
}
