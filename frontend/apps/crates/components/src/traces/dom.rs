use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
    signal_vec::SignalVecExt,
};
use super::state::*;
use shared::domain::jig::module::body::Sticker as RawSticker;

pub fn render_edit(state:Rc<Traces>) -> Dom {

    html!("empty-fragment", {
        .child_signal(state.phase.signal_cloned().map(clone!(state => move |phase| {
            Some(match phase {
                Phase::DisplayAll => {
                    html!("empty-fragment", {
                        .child(html!("div", {
                            .style("position", "absolute")
                            .style("left", "0")
                            .style("top", "0")
                            .style("width", "100%")
                            .style("height", "100%")
                            .style("background-color", "rgba(0, 0, 0, .5)")
                            .event(clone!(state => move |evt:events::MouseDown| {
                                state.start_new_trace(evt.x() as i32, evt.y() as i32);
                            }))
                        }))
                        .children_signal_vec(
                            state.list
                                .signal_vec_cloned()
                                .enumerate()
                                .map(clone!(state => move |(index, trace)| {
                                    super::trace::dom::render_edit(state.clone(), index.clone(), trace)
                                }))
                        )
                    })
                },
                Phase::Edit(edit) => {
                    super::edit::dom::render(edit)
                }
            })
        })))
    })
}
