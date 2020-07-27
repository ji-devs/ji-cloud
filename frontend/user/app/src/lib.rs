//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate derive_more;

mod utils;
mod systems;
mod components;
mod dom;
mod setup;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use shipyard::*;
use web_sys::{window, Element};
use crate::utils::templates::TemplateManager;

/*
mod page;
mod pages;
mod header;
*/
#[wasm_bindgen(start)]
pub fn main_js() {
    setup_logger();
    let settings = core::settings::init();
    utils::firebase::setup(&settings);

    //init dom stuff
    let template_manager = TemplateManager::new(); 

    let document = window().unwrap_throw().document().unwrap_throw();
    let body:Element = document.body().unwrap_throw().into();

    //body.append_child(&template_manager.body()).unwrap_throw();
    //body.append_child(&template_manager.footer()).unwrap_throw();

    //init world
    let world = Rc::new(World::default());
    world.run_with_data(setup::global_uniques, (
            template_manager, 
            document, 
            body, 
            world.clone()
    ));
    systems::workloads::register(&world);

    world.run(systems::routes::init)

    /*

    let page = page::Page::new();

    dominator::append_dom(&dominator::body(), page.render());
	*/
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

