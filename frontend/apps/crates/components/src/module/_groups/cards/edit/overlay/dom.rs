use super::state::*;
use crate::{
    module::{_common::edit::prelude::*, _groups::cards::edit::state::*},
    tooltip::dom::render as render_tooltip,
};
use dominator::{html, Dom};
use futures_signals::{map_ref, signal::SignalExt};
use std::rc::Rc;

// This is really just because originally we didn't have self-managed overlays
// it could be gotten rid of and tooltips rendered in-place

impl<RawData: RawDataExt, E: ExtraExt> DomRenderable for Overlay<RawData, E> {
    fn render(state: Rc<Overlay<RawData, E>>) -> Dom {
        let sig = map_ref! {
            let delete = state.base.tooltips.delete.signal_cloned(),
            let list_error = state.base.tooltips.list_error.signal_cloned()
            => {
                let mut children:Vec<Dom> = Vec::new();
                if let Some(delete) = delete.as_ref() {
                    children.push(render_tooltip(delete.clone()));
                }
                if let Some(list_error) = list_error.as_ref() {
                    children.push(render_tooltip(list_error.clone()));
                }

                children
            }
        };

        html!("overlay-container", {
            .children_signal_vec(sig.to_signal_vec())
        })
    }
}
