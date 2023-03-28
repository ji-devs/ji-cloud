//see: https://github.com/rust-lang/cargo/issues/8010

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod course;
mod debug;
mod jig;
mod pro_dev;
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
    dominator::append_dom(&dominator::get_id("root"), router.render());
    /*

    let page = page::Page::new();

    dominator::append_dom(&dominator::get_id("root"), page.render());
    */
}
