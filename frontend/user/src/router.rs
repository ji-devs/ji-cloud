use std::sync::Arc;
use lazy_static::lazy_static;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use ji_cloud_shared::frontend::routes::Route;
use wasm_bindgen::UnwrapThrowExt;
use dominator::{Dom, class, html, clone, events};
use web_sys::Url;
use crate::settings::SETTINGS;

pub fn route_signal() -> impl Signal<Item = Route> {
    dominator::routing::url()
        .signal_ref(|url| Route::from_url(&url))
}

