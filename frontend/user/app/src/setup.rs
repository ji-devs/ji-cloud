use shipyard::*;
use web_sys::{Element, Document};
use wasm_bindgen::prelude::*;
use std::collections::VecDeque;
use std::rc::Rc;
use crate::{
    components::*,
    utils::templates::TemplateManager
};

pub fn global_uniques(
    (tm, document, body, world): (TemplateManager, Document, Element, Rc<World>),
    storages:AllStoragesViewMut
) {

    let url = web_sys::window().unwrap_throw().location().href().unwrap_throw();
    let route = Route::from_url(&url);
    storages.add_unique(route);
    storages.add_unique_non_send_sync(world);
    storages.add_unique_non_send_sync(tm);
    storages.add_unique_non_send_sync(document);
    storages.add_unique_non_send_sync(DomRoot(body));

    /*
    storages.add_unique_non_send_sync(Router {
        on_location: initial_events.router_location
    });
    */
}
