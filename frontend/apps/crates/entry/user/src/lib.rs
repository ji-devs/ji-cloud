//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod firebase;
mod google_maps;
mod router;
mod login;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use web_sys::{window, Element};
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
/*
mod page;
mod pages;
mod header;
*/
#[wasm_bindgen(start)]
pub async fn main_js() {
    setup_logger();
    let settings = utils::settings::init();



    let promise = unsafe { firebase::init_firebase(settings.firebase_dev) };

    match JsFuture::from(promise).await {
        Ok(_) => {
            let router = router::Router::new();
            dominator::append_dom(&dominator::body(), router.render());
        },
        Err(err) => {
            log::error!("could not initialize firebase!"); 
        }
    }
}




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

