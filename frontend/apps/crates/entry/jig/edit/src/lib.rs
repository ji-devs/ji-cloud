//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod edit;
mod gallery;
mod router;

use wasm_bindgen::prelude::*;

/*
mod page;
mod pages;
mod header;
*/
#[wasm_bindgen(start)]
pub async fn main_js() {
    utils::panic_hook::set_hook();
    utils::logging::setup_logging();

    utils::init::init().await;
    //init dom stuff

    let router = router::Router::new();
    dominator::append_dom(&dominator::body(), router.render());
    /*

    let page = page::Page::new();

    dominator::append_dom(&dominator::body(), page.render());
    */
}
