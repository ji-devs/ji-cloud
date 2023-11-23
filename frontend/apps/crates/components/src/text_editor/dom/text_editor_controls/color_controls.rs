use super::super::super::state::TextEditor;
use crate::{color_select::ColorSelector, text_editor::wysiwyg_types::ControlsChange};
use dominator::{class, clone, html, pseudo, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use rgb::RGBA8;
use std::rc::Rc;
use utils::{colors::*, prelude::*};

pub struct ColorState {
    pub select_for: Mutable<Option<ColorSelectFor>>,
    pub picker: Rc<ColorSelector>,
}

impl ColorState {
    pub fn new(state: Rc<TextEditor>) -> Self {
        let picker = ColorSelector::new(
            (*state).theme_id.clone(),
            None,
            None,
            Some(clone!(state => move |color| {
                let color = rgba8_to_hex_optional(&color);
                let select_for = {
                    state.color_state.borrow().as_ref().unwrap_ji().select_for.get()
                };

                match select_for {
                    Some(ColorSelectFor::Highlight) => {state.set_control_value(ControlsChange::HighlightColor(color))},
                    Some(ColorSelectFor::Text) => {state.set_control_value(ControlsChange::Color(color))},
                    Some(ColorSelectFor::Box) => {state.set_control_value(ControlsChange::BoxColor(color))},
                    None => {}
                };
            })),
        );

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
    Box,
}

pub fn render(state: Rc<TextEditor>) -> Dom {
    let color_state = state.color_state.borrow().as_ref().unwrap_ji().clone();

    html!("anchored-overlay", {
        .prop("slot", "colors")
        .prop("positionY", "top-in")
        .prop("positionX", "right-out")
        .prop("styled", true)
        .class(class! {
            .pseudo!("::part(overlay)", {
                .style("padding", "16px")
            })
        })
        .prop_signal("open", color_state.select_for.signal_cloned().map(|select_for| select_for.is_some()))
        .event(clone!(color_state => move |_: events::Close| {
            color_state.select_for.set(None);
        }))
        .child(html!("div", {
            .prop("slot", "anchor")
            .style("display", "flex")
            .style("justify-content", "space-evenly")
            .children(&mut [
                html!("text-editor-controls-button", {
                    .prop("kind", "color")
                    .prop_signal("active", color_state.select_for.signal_cloned().map(|select_for| {
                        matches!(select_for, Some(ColorSelectFor::Text))
                    }))
                    .event(clone!(state, color_state => move |_: events::Click| {
                        color_state.select_for.set(Some(ColorSelectFor::Text));
                        let color = { state.controls.lock_ref().color.clone() };
                        color_state.picker.set_value(hex_to_rgba8_optional(&color));
                    }))
                }),
                html!("text-editor-controls-button", {
                    .prop("kind", "highlight-color")
                    .prop_signal("active", color_state.select_for.signal_cloned().map(|select_for| {
                        matches!(select_for, Some(ColorSelectFor::Highlight))
                    }))
                    .event(clone!(state, color_state => move |_: events::Click| {
                        color_state.select_for.set(Some(ColorSelectFor::Highlight));
                        let color = { state.controls.lock_ref().highlight_color.clone() };
                        color_state.picker.set_value(hex_to_rgba8_optional(&color));
                    }))
                }),
                html!("text-editor-controls-button", {
                    .prop("kind", "box-color")
                    .prop_signal("active", color_state.select_for.signal_cloned().map(|select_for| {
                        matches!(select_for, Some(ColorSelectFor::Box))
                    }))
                    .event(clone!(state, color_state => move |_: events::Click| {
                        color_state.select_for.set(Some(ColorSelectFor::Box));
                        let color = { state.controls.lock_ref().box_color.clone() };
                        color_state.picker.set_value(hex_to_rgba8_optional(&color));
                    }))
                }),
            ])
        }))
        .child(color_state.picker.render(Some("overlay")))
    })
}

fn hex_to_rgba8_optional(color: &Option<String>) -> Option<RGBA8> {
    color.as_ref().map(|color| hex_to_rgba8(color))
}
fn rgba8_to_hex_optional(color: &Option<RGBA8>) -> Option<String> {
    (*color).map(|color| rgba8_to_hex(&color))
}
