#![feature(type_alias_impl_trait)]
#![allow(confusable_idents)]
#![allow(uncommon_codepoints)]

use leptos::prelude::*;
use leptos_router::components::{Redirect, Route, Router, Routes};
use leptos_router::path;

mod action;
mod app;
mod app_state;
mod definition;
mod drill;
mod engine;
mod gear;
mod key_code;
mod spelling_algebra;
mod theory;
mod view;

use app::Rime打字機應用;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <Router>
                <main>
                    <Routes fallback=|| view! { "404 Not Found" }>
                        <Route path=path!("/typewriter") view=|| view! { <Redirect path="/typewriter/combo_pinyin"/> }/>
                        <Route path=path!("/typewriter/:theory") view=Rime打字機應用 />
                    </Routes>
                </main>
            </Router>
        }
    });
}
