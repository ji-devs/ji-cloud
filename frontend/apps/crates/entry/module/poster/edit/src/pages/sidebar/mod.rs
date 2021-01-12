mod layout;
mod images;

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use {
    layout::LayoutDom,
    images::ImagesDom
};
use crate::data::*;
use dominator::{Dom, clone, DomBuilder, events, apply_methods};
use dominator_helpers::{elem, with_data_id};
use futures_signals::signal::SignalExt;
use crate::templates;

const TOOLBAR_MAP:&[(&'static str, Tool)] = &[
];

pub fn render(state:Rc<State>) -> Dom {

    let apply_tool = |dom:DomBuilder<HtmlElement>, tool:Tool| {
        apply_methods!(dom, {
            .class_signal("first-hidden", state.tool.signal().map(clone!(tool => move |curr_tool| {
                    curr_tool == tool
            })))
            .class_signal("second-hidden", state.tool.signal().map(clone!(tool => move |curr_tool| {
                    curr_tool != tool
            })))

            .event(clone!(state => move |evt:events::Click| {
                state.tool.set(tool)
            }))
        })
    };

    elem!(templates::sidebar(), {

        .with_data_id!("toolbar", {
            .class_signal("hidden", {
                state.tool.signal().map(|curr_tool| curr_tool == Tool::Layout)
            })
            .with_data_id!("text", { .apply(|dom| apply_tool(dom, Tool::Text)) })
            .with_data_id!("images", { .apply(|dom| apply_tool(dom, Tool::Images)) })
            .with_data_id!("bg-color", { .apply(|dom| apply_tool(dom, Tool::BgColor)) })
            .with_data_id!("intro-sound", { .apply(|dom| apply_tool(dom, Tool::IntroSound)) })
            .with_data_id!("sound", { .apply(|dom| apply_tool(dom, Tool::Sound)) })
        })
        .child_signal(state.tool.signal().map(clone!(state => move |tool| {
            match tool {
                Tool::Layout => Some(LayoutDom::render(LayoutDom::new(state.clone()))),
                Tool::Images => Some(ImagesDom::render(ImagesDom::new(state.clone()))),
                _ => None
            }
        })))
    })

}
                
