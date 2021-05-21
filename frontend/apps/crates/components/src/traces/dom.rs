use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
    signal_vec::SignalVecExt,
};
use super::{state::*, trace::edit::state::*};
use shared::domain::jig::module::body::Sticker as RawSticker;

pub fn render_edit(state:Rc<Traces>) -> Dom {

    html!("empty-fragment", {
        .child_signal(state.phase.signal_cloned().map(clone!(state => move |phase| {
            Some(match phase {
                Phase::DisplayAll => {
                    html!("empty-fragment", {
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
                    super::trace::edit::dom::render(edit)
                }
            })
        })))
    })
}
