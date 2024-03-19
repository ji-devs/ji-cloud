use crate::unwrap::UnwrapJiExt;
use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsCast};
use web_sys::EventTarget;

pub fn set_event_listener_once<E>(
    source: &EventTarget,
    event_name: &str,
    callback: Box<dyn FnOnce(E)>,
) where
    E: FromWasmAbi + 'static,
{
    let closure = Closure::once(callback);
    let _ = source.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());

    closure.forget();
}

pub fn set_event_listener<E>(source: &EventTarget, event_name: &str, callback: Box<dyn Fn(E)>)
where
    E: FromWasmAbi + 'static,
{
    let closure = Closure::wrap(callback);
    let _ = source.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());

    closure.forget();
}

pub fn is_iframe() -> bool {
    let window = web_sys::window().unwrap_ji();
    let top = window.top().unwrap_ji().unwrap_ji();
    window != top
}

/// Force a download from a url
pub fn download_url(filename: &str, url: &str) {
    let a = web_sys::window()
        .unwrap_ji()
        .document()
        .unwrap_ji()
        .create_element("a")
        .unwrap_ji()
        .dyn_into::<web_sys::HtmlAnchorElement>()
        .unwrap_ji();
    a.set_href(&url);
    a.set_download(&filename);
    a.click();
}
