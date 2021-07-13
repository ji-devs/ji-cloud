use components::module::_common::edit::prelude::*;
use dominator::{html, Dom, DomBuilder, clone, apply_methods};
use std::rc::Rc;
use crate::base::state::*;
use super::state::*;
use utils::prelude::*;
use components::{
    backgrounds::dom::render_backgrounds, 
    stickers::dom::{render_stickers, render_sticker_raw_parent_mixin},
    traces::edit::dom::render_traces_edit
};

use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};

impl MainSelect {
    pub fn render(state: Rc<Self>) -> Dom {
        let theme_id = state.base.theme_id.get();
        let raw_stickers = Rc::new(state.base.stickers.to_raw());
        
        html!("empty-fragment", {
            .children_signal_vec(
                state.item_kinds()
                    .map(clone!(raw_stickers, state => move |(index, kind)| {
                        let sticker = &raw_stickers[index];

                        render_sticker_raw_parent_mixin(
                            &sticker,
                            theme_id,
                            match kind {
                                ItemKind::Static => DomBuilder::new_html("empty-fragment"),
                                ItemKind::Interactive(kind_state) => {
                                    apply_methods!(DomBuilder::new_html("box-outline"), {
                                        .property_signal("thick", state.is_selected(index))
                                        .event(clone!(state, index => move |evt:events::Close| {
                                            state.base.set_drag_item_deselected(index)
                                        }))
                                    })
                                }
                            },
                            clone!(state, index => move |dom| {
                                dom
                                    .style("cursor", "pointer")
                                    .event(clone!(state, index => move |evt:events::Click| {
                                        state.base.set_drag_item_selected(index)
                                    }))
                            })
                        )
                    }))
            )
        })
    }
}
 
