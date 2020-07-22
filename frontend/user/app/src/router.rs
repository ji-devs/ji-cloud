use std::sync::Arc;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use core::routes::Route;
use wasm_bindgen::UnwrapThrowExt;
use dominator::{Dom, class, html, clone, events};
use web_sys::Url;

pub fn route_signal() -> impl Signal<Item = Route> {
    dominator::routing::url()
        .signal_ref(|url| Route::from_url(&url))
}

