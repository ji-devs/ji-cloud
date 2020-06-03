//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

mod settings;
mod router;
mod page;
mod pages;
mod header;
mod path;
mod utils;
mod globals;

use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// enable logging and panic hook only during debug builds
cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook", debug_assertions))] {
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

#[wasm_bindgen(start)]
pub fn main_js() {
    setup_logger();
    utils::firebase::setup();
    log::info!("{:?}", &*settings::SETTINGS);


    let page = page::Page::new();

    dominator::append_dom(&dominator::body(), page.render());
}
