#![feature(type_alias_impl_trait)]
//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod templates;
mod page;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main_js() {
    utils::panic_hook::set_hook();
    utils::logging::setup_logging();

    utils::init::init().await;

    dominator::append_dom(&dominator::body(), page::Page::render());
}
