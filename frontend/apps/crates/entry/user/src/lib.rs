//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod debug;
mod email;
mod login;
mod oauth;
mod password;
mod profile;
mod register;
mod router;
mod strings;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main_js() {
    utils::panic_hook::set_hook();
    utils::logging::setup_logging();

    crate::debug::init();

    utils::init::init().await;

    let router = router::Router::new();
    dominator::append_dom(&dominator::body(), router.render());
}
