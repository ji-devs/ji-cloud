use super::state::*;
use components::module::_groups::cards::play::card::dom::{
    render_card, render_card_mixin, render_empty_card, render_empty_card_mixin, CardOptions,
    EmptyCardOptions, EmptyKind, Size, StyleKind,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::prelude::*;
use web_sys::HtmlElement;

pub fn render_top(state: Rc<CardTop>) -> Dom {
    let theme_id = state.theme_id;
    let mode = state.mode;
    let side = state.side;

    html!("matching-column", {
        .property("slot", "top")
        .child({
            let card = &state.card;
            let mut options = CardOptions::new(card, theme_id, mode, side, Size::Matching);
            options.flipped = true;
            render_card(options)
        })
        .child_signal(state.phase.signal_cloned().map(clone!(state, theme_id => move |phase| {

            match phase {
                TopPhase::Empty(is_drag_over) => {
                    let options = EmptyCardOptions::new(EmptyKind::Question, theme_id, Size::Matching);
                    Some(render_empty_card_mixin(options, |dom| {
                        dom
                            .event(|_evt:events::Click| {
                                log::info!("empty space clicked...")
                            })
                            .property_signal("active", is_drag_over.signal())
                            .after_inserted(clone!(state => move |elem| {
                                *state.elem.borrow_mut() = Some(elem);
                            }))
                            .after_removed(clone!(state => move |_elem| {
                                *state.elem.borrow_mut() = None;
                            }))

                    }))
                },
                TopPhase::Landed => {
                    let card = &state.other;
                    let mut options = CardOptions::new(card, theme_id, mode, side.negate(), Size::Matching);
                    options.flipped = true;
                    Some(render_card(options))
                }
            }
        })))
    })
}

pub fn render_bottom(state: Rc<CardBottom>) -> Dom {
    html!("div", {
        .property("slot", "bottom")
        .style("touch-action", "none")
        .event(clone!(state => move |evt:events::PointerDown| {
            let elem: HtmlElement = evt.dyn_target().unwrap_ji();
            super::actions::start_drag(state.clone(), elem, evt.x(), evt.y());
        }))
        .child_signal(state.phase.signal_cloned().map(clone!(state => move |phase| {
            let theme_id = state.theme_id;
            let mode = state.mode;
            let side = state.side;
            let card = &state.card;
            let _other = &state.other;
            Some(match phase {
                BottomPhase::Show => {
                    let mut options = CardOptions::new(card, theme_id, mode, side, Size::Matching);
                    options.flipped = true;
                    render_card_mixin(options, |dom| {
                        // block events on the element so that it's parent gets them (needed for touch)
                        dom.style("pointer-events", "none")
                    })
                },
                BottomPhase::Remove => {
                    let options = EmptyCardOptions::new(EmptyKind::Translucent, theme_id, Size::Matching);
                    render_empty_card(options)
                }
            })
        })))
    })
}

pub fn render_drag(state: Rc<CardDrag>) -> Dom {
    let theme_id = state.theme_id;
    let mode = state.mode;
    let side = state.side;
    let card = &state.card;
    let _other = &state.other;

    let mut options = CardOptions::new(card, theme_id, mode, side, Size::Matching);
    options.flipped = true;
    options.style_kind = StyleKind::Dragging;

    render_card_mixin(options, |dom| {
        dom.property("hasTransform", true)
            .style_signal("transform", state.drag.transform_signal())
            .global_event(clone!(state => move |_evt:events::PointerUp| {
                state.on_release();
                //on_mouse_up(evt.x() as i32, evt.y() as i32);
            }))
            .global_event(clone!(state => move |evt:events::PointerMove| {
                if let Some(_point) = state.drag.update(evt.x(), evt.y()) {
                    state.evaluate_drag_over();
                }
            }))
            .after_inserted(clone!(state => move |elem| {
                *state.elem.borrow_mut() = Some(elem);
            }))
            .after_removed(clone!(state => move |_elem| {
                *state.elem.borrow_mut() = None;
            }))
    })
}
