use dominator::{html, Dom, clone, svg, class};
use std::rc::Rc;
use utils::{prelude::*, resize::resize_info_signal};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::SignalVecExt,
};
use super::state::*;
use super::all::trace::state::*;
use web_sys::HtmlCanvasElement;
use awsm_web::canvas::get_2d_context;
use once_cell::sync::Lazy;
use std::fmt::Write;



pub fn render_traces_edit(state:Rc<TracesEdit>) -> Dom {
    html!("empty-fragment", {
        .child_signal(state.phase.signal_cloned().map(clone!(state => move |phase| {
            match phase {
                Phase::All => {
                    Some(super::all::dom::render_traces_all(state.clone()))
                },
                Phase::Draw(draw) => {
                    Some(super::draw::dom::render_traces_draw(draw.clone(), state.list.lock_ref()))
                }
            }
        })))
    })
}
