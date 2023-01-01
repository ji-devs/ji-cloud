use dominator::{clone, dom_builder, html, with_node, Dom};
use futures_signals::signal::SignalExt;

use std::rc::Rc;
use utils::{
    math::{bounds, mat4::Matrix4},
    prelude::*,
    resize::resize_info_signal,
};
use wasm_bindgen::JsCast;

use super::super::helpers::*;
use super::state::*;

impl ImagePlayer {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        let transform_matrix = Matrix4::new_direct(state.raw.transform_matrix);
        let transform_signal = resize_info_signal().map(move |resize_info| {
            let mut m = transform_matrix.clone();
            m.denormalize(&resize_info);
            m.as_matrix_string()
        });

        log::info!("Loading {}!", state.raw.filename);

        if state.get_text().is_none() || !state.raw.filename.is_empty() {
            html!("img" => web_sys:: HtmlImageElement, {
                .attr("src", &state.base.design_media_url(&state.raw.filename))
                .style_signal("opacity", state.controller.hidden.signal().map(|hidden| {
                    if hidden {
                        "0"
                    } else {
                        "1"
                    }
                }))
                .style("cursor", if state.controller.interactive {"pointer"} else {"initial"})
                .style("display", "block")
                .style("position", "absolute")
                .style_signal("width", width_signal(state.size.signal_cloned()))
                .style_signal("height", height_signal(state.size.signal_cloned()))
                .style_signal("top", bounds::size_height_center_rem_signal(state.size.signal()))
                .style_signal("left", bounds::size_width_center_rem_signal(state.size.signal()))
                .style_signal("transform", transform_signal)
                .with_node!(elem => {
                    .event(clone!(state => move |_evt:events::Load| {
                        if state.size.get_cloned().is_none() {

                            let mut width = elem.natural_width() as f64;
                            let mut height = elem.natural_height() as f64;

                            // ugly hack... would be much better to fix at the source
                            // but, whatever...
                            if state.get_text().is_some() {
                                width /= 2.0;
                                height /= 2.0;
                            }

                            state.size.set(Some((width, height)));
                        }

                        *state.controller.elem.borrow_mut() = Some(elem.clone().unchecked_into());
                        state.base.insert_stage_click_listener(clone!(state => move |stage_click| {
                            state.controller.handle_click(stage_click)
                        }));
                    }))
                })
            })
        } else {
            html!("div" => web_sys:: HtmlDivElement, {
                .style_signal("opacity", state.controller.hidden.signal().map(|hidden| {
                    if hidden {
                        "0"
                    } else {
                        "1"
                    }
                }))
                .style("cursor", if state.controller.interactive {"pointer"} else {"initial"})
                .style("display", "block")
                .style("position", "absolute")
                .style_signal("width", width_signal(state.size.signal_cloned()))
                .style_signal("height", height_signal(state.size.signal_cloned()))
                .style_signal("top", bounds::size_height_center_rem_signal(state.size.signal()))
                .style_signal("left", bounds::size_width_center_rem_signal(state.size.signal()))
                .style_signal("transform", transform_signal)
                .after_inserted(clone!(state => move |elem| {
                    *state.controller.elem.borrow_mut() = Some(elem.unchecked_into());
                    state.base.insert_stage_click_listener(clone!(state => move |stage_click| {
                        state.controller.handle_click(stage_click)
                    }));
                }))
                .child(dom_builder!(parse_html(state.get_text().unwrap_ji()), {
                }))
            })
        }
    }
}

fn parse_html(html: &str) -> web_sys::HtmlElement {
    let parser = web_sys::DomParser::new().unwrap();

    // NOTE: this is error-prone, if the text itself contains "px" then it will be replaced
    // should instead parse as proper stylesheet, or at least a regex to replace the pattern
    // properly
    let html = html.replace("px", "rem");
    let document = parser
        .parse_from_string(&html, web_sys::SupportedType::TextHtml)
        .unwrap();

    document
        .body()
        .unwrap()
        .first_element_child()
        .unwrap()
        .unchecked_into()
}
