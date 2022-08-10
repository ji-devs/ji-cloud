#![feature(type_alias_impl_trait)]
//see: https://github.com/rust-lang/cargo/issues/8010

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod base;
mod debug;
mod router;
mod state;

use components::audio::mixer::AUDIO_MIXER;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use router::Router;

#[wasm_bindgen(start)]
pub async fn main_js() {
    utils::panic_hook::set_hook();
    utils::logging::setup_logging();

    utils::init::init().await;

    let router = Rc::new(Router::new());

    router::render(router.clone());

    std::mem::forget(Box::new(router));

    AUDIO_MIXER.with(|_mixer| {})
}
