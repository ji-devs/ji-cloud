use dominator::{html, Dom};
use js_sys::Reflect;
use shared::domain::{
    jig::codes::JigPlaySessionDragDrop,
    module::body::{_groups::design::Sticker, drag_drop},
};
use utils::unwrap::UnwrapJiExt;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::window;

pub fn render_drag_drop(module: &drag_drop::Content, session: &JigPlaySessionDragDrop) -> Dom {
    html!("div", {
        .children(
            session.items.iter().map(|(index, item)| {
                html!("div", {
                    .class("wrapper")
                    .child(
                        html!("div", {
                            .class("item")
                            .child(html!("div", {
                                .apply(|dom| {
                                    match &module.items.get(*index) {
                                        Some(item) => {
                                            match &item.sticker {
                                                Sticker::Sprite(sprite) => {
                                                    dom.child(html!("img-ji", {
                                                        .prop("id", sprite.image.id.0.to_string())
                                                        .prop("lib", sprite.image.lib.to_str())
                                                    }))
                                                },
                                                Sticker::Text(text) => {
                                                    dom.text(&extract_wysiwyg_text(&text.value))
                                                },
                                                Sticker::Embed(_) => {
                                                    dom.text("Embed")
                                                },
                                            }
                                        },
                                        None => dom.text("?"),
                                    }
                                })
                            }))
                            .child(html!("p", {
                                .text("Tries ")
                                .child(html!("strong", {
                                    .text(&(item.failed_tries + 1).to_string())
                                }))
                            }))
                        })
                    )
                })
            })
        )
    })
}

fn extract_wysiwyg_text(s: &str) -> String {
    let window = window().unwrap_ji();
    let extract_wysiwyg_text =
        Reflect::get(&window, &JsValue::from_str("extract_wysiwyg_text")).unwrap_ji();
    let extract_wysiwyg_text = extract_wysiwyg_text
        .dyn_ref::<js_sys::Function>()
        .unwrap_ji();
    let res = extract_wysiwyg_text
        .call1(&window, &JsValue::from_str(s))
        .unwrap_ji();
    res.as_string().unwrap_ji()
}
