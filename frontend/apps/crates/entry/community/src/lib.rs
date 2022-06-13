//see: https://github.com/rust-lang/cargo/issues/8010

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod badge_details;
mod badges_list;
mod dom;
mod member_details;
mod members_list;
mod state;

use state::Community;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main_js() {
    utils::panic_hook::set_hook();
    utils::logging::setup_logging();

    // crate::debug::init();

    utils::init::init().await;

    dominator::append_dom(&dominator::get_id("root"), Community::new().render());
}
