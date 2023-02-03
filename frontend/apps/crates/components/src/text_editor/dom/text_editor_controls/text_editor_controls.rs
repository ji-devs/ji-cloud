use crate::hebrew_buttons::HebrewButtons;
use crate::text_editor::font_css_converter::font_to_css;
use crate::text_editor::wysiwyg_types::ControlsChange;
use dominator::{clone, html, Dom};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use shared::domain::module::body::_groups::design::{Text as RawText, DEFAULT_TEXT_VALUE};
use std::rc::Rc;
use strum::IntoEnumIterator;
use utils::prelude::*;

use super::super::super::state::TextEditor;
use super::super::super::wysiwyg_types::{Align, ElementType, Font, Weight, BOLD_WEIGHT};
use super::color_controls;

const STR_WEIGHT_LABEL: &str = "Weight";
const STR_FONT_LABEL: &str = "Font";

const STR_WEIGHT_200: &str = "Light";
const STR_WEIGHT_400: &str = "Regular";
const STR_WEIGHT_700: &str = "Bold";
const STR_WEIGHT_900: &str = "Bolder";
const STR_WEIGHT_CUSTOM: &str = "Custom";

const WEIGHT_OPTIONS: &[u16] = &[200, 400, 700, 900];

impl TextEditor {
    pub fn render_controls(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("text-editor-controls", {
            .prop_signal("controlsDisabled", state.wysiwyg_ref.signal_ref(|x| x.is_none()))
            .children(&mut [
                HebrewButtons::full().render(Some("hebrew-buttons")),
                html!("text-editor-controls-insert-button", {
                    .prop("slot", "insert-button")
                    .prop_signal("disabled", state.wysiwyg_ref.signal_ref(|x| x.is_some()))
                    .event(clone!(state => move |_: events::Click| {
                        if let Some(on_new_text) = state.callbacks.on_new_text.as_ref() {
                            //TODO - this should create a slate value
                            //with the current settings and only replace the text
                            (on_new_text) (&RawText::value_from_str(DEFAULT_TEXT_VALUE));
                        }
                    }))
                }),
                html!("input-select", {
                    .prop("slot", "font")
                    .prop("label", STR_FONT_LABEL)
                    .prop_signal("value", state.controls.signal_cloned().map(|controls| controls.font))
                    // .style_signal("font-family", state.controls.signal_cloned().map(|controls| format!("'{}'", controls.font.to_string())))
                    .children_signal_vec(
                        state
                            .fonts
                            .signal_cloned()
                            .to_signal_vec()
                            .map(clone!(state => move |font| render_font_option(state.clone(), &font)))
                    )
                }),
                html!("input-select", {
                    .prop("slot", "weight")
                    .prop("label", STR_WEIGHT_LABEL)
                    .prop_signal("value", state.controls.signal_cloned().map(|controls| readable_weight(controls.weight)))
                    .children(WEIGHT_OPTIONS.iter().map(|weight| render_weight_option(state.clone(), *weight)))
                }),
                html!("text-editor-controls-input-number", {
                    .prop("slot", "font-size")
                    .prop_signal("value", state.controls.signal_cloned().map(|controls| {
                        controls.font_size
                    }))
                    .event(clone!(state => move |e: events::CustomChange| {
                        let value = e.value();
                        let value = value.parse().unwrap_or(24);
                        state.set_control_value(ControlsChange::FontSize(value))
                    }))
                }),
                html!("text-editor-controls-button", {
                    .prop("kind", "bold")
                    .prop("slot", "bold")
                    .prop_signal("active", state.controls.signal_cloned().map(|controls| {
                        controls.weight == BOLD_WEIGHT
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        state.toggle_bold();
                    }))
                }),
                html!("text-editor-controls-button", {
                    .prop("kind", "italic")
                    .prop("slot", "italic")
                    .prop_signal("active", state.controls.signal_cloned().map(|controls| {
                        controls.italic
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        state.toggle_italic();
                    }))
                }),
                html!("text-editor-controls-button", {
                    .prop("kind", "underline")
                    .prop("slot", "underline")
                    .prop_signal("active", state.controls.signal_cloned().map(|controls| {
                        controls.underline
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        state.toggle_underline();
                    }))
                }),
                color_controls::render(state.clone()),
            ])
            .children(ElementType::iter()
                .map(|element| render_element_option(state.clone(), element))
            )
            .children(Align::iter()
                .map(|align| render_align_option(state.clone(), align))
            )
        })
    }
}

fn readable_weight(weight: Weight) -> &'static str {
    match weight {
        200 => STR_WEIGHT_200,
        400 => STR_WEIGHT_400,
        700 => STR_WEIGHT_700,
        900 => STR_WEIGHT_900,
        _ => STR_WEIGHT_CUSTOM,
    }
}

fn render_element_option(state: Rc<TextEditor>, element: ElementType) -> Dom {
    html!("text-editor-controls-button", {
        .prop("kind", element.to_string().to_lowercase())
        .prop("slot", element.to_string().to_lowercase())
        .prop_signal("active", state.controls.signal_cloned().map(clone!(element => move |controls| {
            controls.element == element
        })))
        .event(clone!(state, element => move |_: events::Click| {
            state.set_control_value(ControlsChange::Element(element.clone()));
        }))
    })
}

fn render_align_option(state: Rc<TextEditor>, align: Align) -> Dom {
    html!("text-editor-controls-button", {
        .prop("kind", match align {
            Align::Left => "align-left",
            Align::Center => "align-center",
            Align::Right => "align-right",
        })
        .prop("slot", match align {
            Align::Left => "align-left",
            Align::Center => "align-center",
            Align::Right => "align-right",
        })
        .prop_signal("active", state.controls.signal_cloned().map(clone!(align => move |controls| {
            controls.align == align
        })))
        .event(clone!(state, align => move |_: events::Click| {
            state.set_control_value(ControlsChange::Align(align.clone()))
        }))
    })
}

fn render_weight_option(state: Rc<TextEditor>, weight: Weight) -> Dom {
    html!("input-select-option", {
        .style("font-weight", weight.to_string())
        .prop_signal("selected", state.controls.signal_cloned().map(clone!(weight => move |controls| {
            controls.weight == weight
        })))
        .text(readable_weight(weight))
        .event(clone!(state, weight => move |evt: events::CustomSelectedChange| {
            if evt.selected() {
                state.set_control_value(ControlsChange::Weight(weight))
            }
        }))
    })
}

fn render_font_option(state: Rc<TextEditor>, font: &Font) -> Dom {
    html!("input-select-option", {
        .style("font-family", font_to_css(font))
        .prop_signal("selected", state.controls.signal_cloned().map(clone!(font => move |controls| {
            controls.font == font
        })))
        .text(font)
        .event(clone!(state, font => move |evt: events::CustomSelectedChange| {
            if evt.selected() {
                state.set_control_value(ControlsChange::Font(font.clone()))
            }
        }))
    })
}
