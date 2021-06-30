#![feature(type_alias_impl_trait)]
#![feature(min_type_alias_impl_trait)]

//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod router;
mod debug;
mod state;
mod settings;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use web_sys::{window, Element};
use router::Router;

#[wasm_bindgen(start)]
pub async fn main_js() {
    setup_logger();
    components::module::_groups::cards::edit::config::init();
    let settings = utils::settings::init();

    let router = Rc::new(Router::new());

    router::render(router.clone());

    //std::mem::forget(Box::new(router));
}




// enable logging and panic hook only during debug builds
cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook"))] {
        fn setup_logger() {
            wasm_logger::init(wasm_logger::Config::default());
            console_error_panic_hook::set_once();
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            log::info!("rust logging enabled!!!");
        }
    } else {
        fn setup_logger() {
            log::info!("rust logging disabled!"); //<-- won't be seen
        }
    }
}

