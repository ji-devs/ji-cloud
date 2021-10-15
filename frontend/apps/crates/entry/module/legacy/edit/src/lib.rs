#![feature(type_alias_impl_trait)]
//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main_js() {
    utils::panic_hook::set_hook();
    utils::logging::setup_logging();

    utils::init::init().await;

    //just a stub... legacy modules can't be edited
}
