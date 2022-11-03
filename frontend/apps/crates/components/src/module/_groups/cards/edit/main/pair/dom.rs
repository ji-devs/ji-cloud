use dominator::{html, Dom, clone};
use futures_signals::map_ref;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use futures_signals::signal::{Signal, SignalExt};

use super::card::dom::render as render_card;
use super::state::*;
use crate::hebrew_buttons::{HebrewButtons, HebrewButtonsConfig, Kind};
use crate::module::_groups::cards::edit::state::*;

pub fn render<RawData: RawDataExt, E: ExtraExt>(state: Rc<MainPair<RawData, E>>) -> Dom {
    html!("main-card-pair", {
        .property_signal("index", state.index.signal().map(|x| {
            JsValue::from_f64(x.unwrap_or_default() as f64)
        }))
        .child_signal(show_hebrew_buttons_signal(&state).dedupe().map(clone!(state => move |editing| {
            editing.then(|| {
                HebrewButtons::new(HebrewButtonsConfig {
                    kind: Kind::Full,
                    on_open_toggle: Box::new(clone!(state => move |is_open| {
                        state.hebrew_buttons_open.set_neq(is_open);
                    }))
                }).render(Some("hebrew-buttons"))
            })
        })))
        .child(render_card(state.left.clone()))
        .child(render_card(state.right.clone()))
    })
}

fn show_hebrew_buttons_signal<RawData: RawDataExt, E: ExtraExt>(
    state: &Rc<MainPair<RawData, E>>,
) -> impl Signal<Item = bool> {
    map_ref! {
        let editing_left = state.left.editing_active.signal(),
        let editing_right = state.right.editing_active.signal(),
        let hebrew_buttons_open = state.hebrew_buttons_open.signal() => {
            *editing_left || *editing_right || *hebrew_buttons_open
        }
    }
}
