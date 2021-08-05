use dominator::{Dom, html, clone};
use std::rc::Rc;
use utils::prelude::*;
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use strum::IntoEnumIterator;
use crate::text_editor::config::STR_NEW_TEXT;
use crate::text_editor::font_css_converter::font_to_css;
use crate::text_editor::wysiwyg_types::ControlsChange;

use super::super::super::wysiwyg_types::{Align, ElementType, Font, Weight, BOLD_WEIGHT};
use super::super::super::state::State;
use super::color_controls;

const STR_WEIGHT_LABEL: &'static str = "Weight";
const STR_FONT_LABEL: &'static str = "Font";

const STR_WEIGHT_200: &'static str = "Light";
const STR_WEIGHT_400: &'static str = "Regular";
const STR_WEIGHT_700: &'static str = "Bold";
const STR_WEIGHT_900: &'static str = "Bolder";
const STR_WEIGHT_CUSTOM: &'static str = "Custom";


const WEIGHT_OPTIONS: &'static [u16] = &[200, 400, 700, 900];

fn readable_weight(weight: Weight) -> &'static str {
    match weight {
        200 => STR_WEIGHT_200,
        400 => STR_WEIGHT_400,
        700 => STR_WEIGHT_700,
        900 => STR_WEIGHT_900,
        _ => STR_WEIGHT_CUSTOM,
    }
}

pub fn render(state: Rc<State>) -> Dom {
    html!("text-editor-controls", {
        .property_signal("controlsDisabled", state.wysiwyg_ref.signal_ref(|x| x.is_none()))
        .children(&mut [
            html!("text-editor-controls-insert-button", {
                .property("slot", "insert-button")
                .property_signal("disabled", state.wysiwyg_ref.signal_ref(|x| x.is_some()))
                .event(clone!(state => move |_: events::Click| {
                    if let Some(on_new_text) = state.callbacks.on_new_text.as_ref() {
                        //TODO - this should create a slate value
                        //with the current settings and only replace the text
                        (on_new_text) (&State::text_to_value(STR_NEW_TEXT));
                    }
                }))
            }),
            html!("input-select", {
                .property("slot", "font")
                .property("label", STR_FONT_LABEL)
                .property_signal("value", state.controls.signal_cloned().map(|controls| controls.font.to_string()))
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
                .property("slot", "weight")
                .property("label", STR_WEIGHT_LABEL)
                .property_signal("value", state.controls.signal_cloned().map(|controls| readable_weight(controls.weight)))
                .children(WEIGHT_OPTIONS.iter().map(|weight| render_weight_option(state.clone(), *weight)))
            }),
            html!("text-editor-controls-input-number", {
                .property("slot", "font-size")
                .property_signal("value", state.controls.signal_cloned().map(|controls| {
                    controls.font_size
                }))
                .event(clone!(state => move |e: events::CustomChange| {
                    let value = e.value();
                    let value = u8::from_str_radix(&value, 10).unwrap_or(24);
                    state.set_control_value(ControlsChange::FontSize(value))
                }))
            }),
            html!("text-editor-controls-button", {
                .property("kind", "bold")
                .property("slot", "bold")
                .property_signal("active", state.controls.signal_cloned().map(|controls| {
                    controls.weight == BOLD_WEIGHT
                }))
                .event(clone!(state => move |_: events::Click| {
                    state.toggle_bold();
                }))
            }),
            html!("text-editor-controls-button", {
                .property("kind", "italic")
                .property("slot", "italic")
                .property_signal("active", state.controls.signal_cloned().map(|controls| {
                    controls.italic
                }))
                .event(clone!(state => move |_: events::Click| {
                    state.toggle_italic();
                }))
            }),
            html!("text-editor-controls-button", {
                .property("kind", "underline")
                .property("slot", "underline")
                .property_signal("active", state.controls.signal_cloned().map(|controls| {
                    controls.underline
                }))
                .event(clone!(state => move |_: events::Click| {
                    state.toggle_underline();
                }))
            }),
            html!("text-editor-controls-button", {
                .property("kind", "indent")
                .property("slot", "indent")
                .property_signal("active", state.controls.signal_cloned().map(|controls| {
                    controls.indent_count > 0
                }))
                .event(clone!(state => move |_: events::Click| {
                    let count: u8 = state.controls.lock_ref().indent_count + 1;
                    state.set_control_value(ControlsChange::IndentCount(count))
                }))
            }),
            html!("text-editor-controls-button", {
                .property("kind", "outdent")
                .property("slot", "outdent")
                .event(clone!(state => move |_: events::Click| {
                    let mut count: u8 = state.controls.lock_ref().indent_count;
                    if count > 0 {
                        count = count - 1;
                    }
                    state.set_control_value(ControlsChange::IndentCount(count))
                }))
                .property_signal("active", state.controls.signal_cloned().map(|controls| {
                    controls.indent_count == 0
                }))
            }),
            html!("button-sidebar", {
                .property("slot", "hebrew-keyboard")
                .property("mode", "keyboard")
            }),
            html!("button-sidebar", {
                .property("slot", "dicta")
                .property("mode", "dicta")
            }),
            html!("button-sidebar", {
                .property("slot", "sefaria")
                .property("mode", "sefaria")
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


fn render_element_option(state: Rc<State>, element: ElementType) -> Dom {
    html!("text-editor-controls-button", {
        .property("kind", element.to_string().to_lowercase())
        .property("slot", element.to_string().to_lowercase())
        .property_signal("active", state.controls.signal_cloned().map(clone!(element => move |controls| {
            if controls.element == element {
                true
            } else {
                false
            }
        })))
        .event(clone!(state, element => move |_: events::Click| {
            state.set_control_value(ControlsChange::Element(element.clone()));
        }))
    })
}

fn render_align_option(state: Rc<State>, align: Align) -> Dom {
    html!("text-editor-controls-button", {
        .property("kind", match align {
            Align::Left => "align-left",
            Align::Center => "align-center",
            Align::Right => "align-right",
        })
        .property("slot", match align {
            Align::Left => "align-left",
            Align::Center => "align-center",
            Align::Right => "align-right",
        })
        .property_signal("active", state.controls.signal_cloned().map(clone!(align => move |controls| {
            if controls.align == align {
                true
            } else {
                false
            }
        })))
        .event(clone!(state, align => move |_: events::Click| {
            state.set_control_value(ControlsChange::Align(align.clone()))
        }))
    })
}

fn render_weight_option(state: Rc<State>, weight: Weight) -> Dom {
    html!("input-select-option", {
        .style("font-weight", weight.to_string())
        .property_signal("selected", state.controls.signal_cloned().map(clone!(weight => move |controls| {
            if controls.weight == weight {
                true
            } else {
                false
            }
        })))
        .text(readable_weight(weight))
        .event(clone!(state, weight => move |evt: events::CustomSelectedChange| {
            if evt.selected() {
                state.set_control_value(ControlsChange::Weight(weight))
            }
        }))
    })
}

fn render_font_option(state: Rc<State>, font: &Font) -> Dom {
    html!("input-select-option", {
        .style("font-family", font_to_css(font))
        .property_signal("selected", state.controls.signal_cloned().map(clone!(font => move |controls| {
            if controls.font == font {
                true
            } else {
                false
            }
        })))
        .text(&font.to_string())
        .event(clone!(state, font => move |evt: events::CustomSelectedChange| {
            if evt.selected() {
                state.set_control_value(ControlsChange::Font(font.clone()))
            }
        }))
    })
}
