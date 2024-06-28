//see: https://github.com/rust-lang/cargo/issues/8010

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod debug;
mod email;
mod email_handler;
mod likes;
mod login;
mod oauth;
mod password_handler;
mod register;
mod reset_password;
mod router;
mod school_end;
mod school_start;
mod settings;
mod strings;
mod subscribe1;
mod subscribe2;
mod welcome;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main_js() {
    utils::panic_hook::set_hook();
    utils::logging::setup_logging();

    crate::debug::init();

    utils::init::init().await;

    let router = router::Router::new();
    dominator::append_dom(&dominator::get_id("root"), router.render());
}
