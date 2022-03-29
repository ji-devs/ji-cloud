#![feature(type_alias_impl_trait)]
//see: https://github.com/rust-lang/cargo/issues/8010

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod base;
mod config;
mod debug;
mod router;
mod state;

use router::Router;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main_js() {
    utils::panic_hook::set_hook();
    utils::logging::setup_logging();

    utils::init::init().await;

    components::module::_groups::cards::play::config::init();
    let router = Rc::new(Router::new());

    router::render(router);

    //std::mem::forget(Box::new(router));
}
