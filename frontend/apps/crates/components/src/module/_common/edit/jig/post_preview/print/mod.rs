use awsm_web::dom::StyleExt;
use futures::future::join_all;
use gloo_timers::future::TimeoutFuture;
use js_sys::Reflect;
use utils::callback_future::CallbackFuture;
use utils::{js_wrappers::set_event_listener_once, unwrap::UnwrapJiExt};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlIFrameElement, Window};

pub mod cards;
pub mod screenshot;

fn print(html: String, scripts: Vec<String>) {
    spawn_local(async move {
        let document = web_sys::window().unwrap_ji().document().unwrap_ji();
        let body = document.body().unwrap_ji();

        let iframe: HtmlIFrameElement = document
            .create_element("iframe")
            .unwrap_ji()
            .dyn_into()
            .unwrap_ji();

        iframe.set_style("height", "0");
        iframe.set_style("width", "0");
        iframe.set_style("border", "0");

        let _ = body.append_child(&iframe);

        // needed for Firefox, give the iframe a chance to be attached to the dom before doing anything with it
        TimeoutFuture::new(0).await;

        let iframe_window = iframe.content_window().unwrap_ji();

        let iframe_body = iframe_window.document().unwrap_ji().body().unwrap_ji();

        // Can't use dominator dominator::append_dom since the html! macro uses document.createElement but `document` has to be the iframe document
        iframe_body.set_inner_html(&html);

        let scripts = scripts
            .into_iter()
            .map(|script| add_script(iframe_window.clone(), script));
        join_all(scripts).await;

        // give css a chance to load, with simple tests 1 second was enough
        // querying the document for the css link and listening for it's load event might make more sense
        gloo_timers::future::TimeoutFuture::new(1_000).await;

        set_event_listener_once(
            &iframe_window,
            "afterprint",
            Box::new(move |_: web_sys::Event| {
                let _ = body.remove_child(&iframe);
            }),
        );

        let _ = iframe_window.print();
    });
}

fn add_script(window: Window, src: String) -> CallbackFuture<()> {
    CallbackFuture::new(Box::new(move |resolve| {
        let document = window.document().unwrap_ji();
        let body = window.document().unwrap_ji().body().unwrap_ji();

        let script = document.create_element("script").unwrap_ji();

        // using reflect rather then `dyn_into` `HtmlScriptElement` because dyn_into expects the script to be on the same document
        // but this script element is on the iframes document
        let _ = Reflect::set(
            &script,
            &JsValue::from_str("type"),
            &JsValue::from_str("module"),
        );
        let _ = Reflect::set(&script, &JsValue::from_str("src"), &JsValue::from_str(&src));

        let callback: Box<dyn FnOnce(_)> = Box::new(move |_: web_sys::Event| {
            resolve(());
        });

        set_event_listener_once(&script, "load", callback);
        let _ = body.append_child(&script);
    }))
}
