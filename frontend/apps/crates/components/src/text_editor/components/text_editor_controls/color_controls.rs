use std::rc::Rc;
use dominator::{Dom, html, clone};
use super::super::super::state::State;
use futures_signals::signal::Mutable;
use utils::{prelude::*, colors::*};
use futures_signals::signal::SignalExt;
use wasm_bindgen_futures::spawn_local;
use futures::future::ready;
use rgb::RGBA8;
use crate::{color_select::{
    self,
    state::State as ColorPickerState,
}, text_editor::wysiwyg_types::ControlsChange};

pub struct ColorState {
    pub select_for: Mutable<Option<ColorSelectFor>>,
    pub picker: Rc<ColorPickerState>,
}


impl ColorState {
    pub fn new(state:Rc<State>) -> Self {
        let picker = Rc::new(ColorPickerState::new(
            (*state).theme_id.clone(),
            None, 
            Some(clone!(state => move |color| {
                let color = rgba8_to_hex_optional(&Some(color));
                let select_for = {
                    state.color_state.borrow().as_ref().unwrap_ji().select_for.get()
                };

                match select_for {
                    Some(ColorSelectFor::Highlight) => {state.set_control_value(ControlsChange::HighlightColor(color))},
                    Some(ColorSelectFor::Text) => {state.set_control_value(ControlsChange::Color(color))},
                    None => {}
                };
            }))
        ));

        Self {
            select_for: Mutable::new(None),
            picker,
        }
    }
    
}

#[derive(Clone, Copy)]
pub enum ColorSelectFor {
    Text,
    Highlight,
}

pub fn render(state: Rc<State>) -> Dom {
    let color_state = state.color_state.borrow().as_ref().unwrap_ji().clone();

    html!("anchored-overlay", {
        // .future(state.theme_id.signal_cloned().for_each(clone!(state, color_state => move |theme_id| {
        //     color_state.picker.set_theme(theme_id);
        //     ready(())
        // })))
        .property("slot", "color")
        .property("positionY", "top-in")
        .property_signal("open", color_state.select_for.signal_cloned().map(|select_for| select_for.is_some()))
        .event(clone!(color_state => move |_: events::Close| {
            color_state.select_for.set(None);
        }))
        .child(html!("button-collection", {
            .property("slot", "anchor")
            .children(&mut [
                html!("text-editor-control", {
                    .property("type", "color")
                    .event(clone!(state, color_state => move |_: events::Click| {
                        color_state.select_for.set(Some(ColorSelectFor::Highlight));
                        let color = { state.controls.lock_ref().color.clone() };
                        if let Some(color) = hex_to_rgba8_optional(&color) {
                            color_state.picker.set_selected(color);
                        }
                    }))
                }),
                html!("text-editor-control", {
                    .property("type", "marker-color")
                    .event(clone!(state, color_state => move |_: events::Click| {
                        color_state.select_for.set(Some(ColorSelectFor::Text));
                        let color = { state.controls.lock_ref().color.clone() };
                        if let Some(color) = hex_to_rgba8_optional(&color) {
                            color_state.picker.set_selected(color);
                        }
                    }))
                }),
            ])
        }))
        .child(html!("text-editor-controls-overlay-shadow", {
            .property("slot", "overlay")
            .child(color_select::dom::render(color_state.picker.clone(), None))
        }))
    })
}


fn hex_to_rgba8_optional(color: &Option<String>) -> Option<RGBA8> {
    match color {
        Some(color) => Some(hex_to_rgba8(&color)),
        None => None,
    }
}
fn rgba8_to_hex_optional(color: &Option<RGBA8>) -> Option<String> {
    match *color {
        Some(color) => Some(rgba8_to_hex(&color)),
        None => None,
    }
}
