use dominator::{html, Dom, DomBuilder, clone};
use web_sys::HtmlElement;
use super::state::*;
use std::rc::Rc;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, ReadOnlyMutable}
};
use components::module::_groups::cards::{
    lookup::{self, Side},
    play::card::dom::{render_card, render_card_mixin, CardOptions, render_empty_card, render_empty_card_mixin, EmptyCardOptions, EmptyKind, Size, StyleKind},
    edit::{
        config,
        state::*
    },
};
use utils::{prelude::*, drag::Drag};

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
                    let mut options = EmptyCardOptions::new(EmptyKind::Question, theme_id, Size::Matching);
                    Some(render_empty_card_mixin(options, |dom| {
                        dom
                            .event(clone!(state => move |evt:events::Click| {
                                log::info!("empty space clicked...")
                            }))
                            .property_signal("active", is_drag_over.signal())
                            .after_inserted(clone!(state => move |elem| {
                                *state.elem.borrow_mut() = Some(elem);
                            }))
                            .after_removed(clone!(state => move |elem| {
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
    html!("empty-fragment", {
        .property("slot", "bottom")
        .child_signal(state.phase.signal_cloned().map(clone!(state => move |phase| {
            let theme_id = state.theme_id;
            let mode = state.mode;
            let side = state.side;
            let card = &state.card;
            let other = &state.other;
            Some(match phase {
                BottomPhase::Show => {
                    let mut options = CardOptions::new(card, theme_id, mode, side, Size::Matching);
                    options.flipped = true;
                    render_card_mixin(options, |dom| {
                        dom
                            .event(clone!(state => move |evt:events::MouseDown| {
                                let elem:HtmlElement = evt.dyn_target().unwrap_ji();

                                super::actions::start_drag(state.clone(), elem, evt.x(), evt.y());
                            }))
                    })
                },
                BottomPhase::Remove => {
                    let mut options = EmptyCardOptions::new(EmptyKind::Translucent, theme_id, Size::Matching);
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
    let other = &state.other;

    let mut options = CardOptions::new(card, theme_id, mode, side, Size::Matching);
    options.flipped = true;
    options.style_kind = StyleKind::Dragging;

    render_card_mixin(options, |dom| {
        dom
            .property("hasTransform", true)
            .style_signal("transform", state.drag.transform_signal())
            .global_event_preventable(clone!(state => move |evt:events::MouseUp| {
                state.on_release(); 
                //on_mouse_up(evt.x() as i32, evt.y() as i32);
            }))
            .global_event_preventable(clone!(state => move |evt:events::MouseMove| {
                if let Some(point) = state.drag.update(evt.x(), evt.y()) {
                    state.evaluate_drag_over();
                }
            }))
            .after_inserted(clone!(state => move |elem| {
                *state.elem.borrow_mut() = Some(elem);
            }))
            .after_removed(clone!(state => move |elem| {
                *state.elem.borrow_mut() = None;
            }))
    })
}
