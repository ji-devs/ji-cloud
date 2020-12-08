use utils::routes::{Route, DevRoute};
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use dominator::{Dom, html};
use crate::pages::resize;

pub fn render() -> Dom {
    html!("div", {
        .child_signal(signal_dom())
    })
}

fn route_signal() -> impl Signal<Item = Route> {
    dominator::routing::url()
        .signal_ref(|url| Route::from_url(&url))
}

fn signal_dom() -> impl Signal<Item = Option<Dom>> {
        route_signal()
            .map(|route| {
                match route {
                    Route::Dev(route) => {
                        match route {
                            DevRoute::Showcase(id, page)=> {
                                match page.as_ref() {
                                    "resize" => Some(resize::render()),
                                    _ => None
                                }
                            }
                        }
                    }
                    _ => None
                }
            })
}
