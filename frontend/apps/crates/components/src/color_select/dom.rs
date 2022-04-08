use crate::color_select::actions::get_user_colors;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use rgb::RGBA8;
use std::rc::Rc;
use utils::{colors::*, prelude::*};

use super::actions::{add_user_color, delete_user_color, set_selected};
use super::state::State;
use wasm_bindgen_futures::spawn_local;

const STR_SYSTEM_COLORS_LABEL: &str = "General colors";
const STR_THEME_COLORS_LABEL: &str = "Theme colors";
const STR_USER_COLORS_LABEL: &str = "My colors";
const STR_ADD_COLOR: &str = "Add color";

pub fn render(state: Rc<State>, slot: Option<&str>) -> Dom {
    State::handle_theme(Rc::clone(&state));

    html!("color-select", {
        .future(clone!(state => async move {
            let user_colors = get_user_colors().await.unwrap_ji();
            state.user_colors.lock_mut().replace_cloned(user_colors);
        }))
        .apply_if(slot.is_some(), move |dom| {
            dom.property("slot", slot.unwrap_ji())
        })
        .apply_if(state.label.is_some(), clone!(state => move |dom| {
            dom.property("label", state.label.clone().unwrap_ji())
        }))
        .child(html!("empty-fragment", { // TODO: once we can have multiple child signals we wont need this
            .property("slot", "sections")
            .child_signal(state.theme_colors.signal_cloned().map(clone!(state => move |theme_colors| {
                Some(render_static_section(state.clone(), &theme_colors, STR_THEME_COLORS_LABEL))
            })))
        }))
        .child(render_static_section(state.clone(), state.system_colors.as_ref(), STR_SYSTEM_COLORS_LABEL))
        .child(render_add_color(state.clone()))
        .child_signal(state.user_colors.signal_vec_cloned().to_signal_cloned().map(clone!(state => move |user_colors| {
            if !user_colors.is_empty() {
                // this re-renders every time the anything in the vec changes, there might be better ways of doing the same thing
                Some(render_user_section(state.clone()))
            } else {
                None
            }
        })))
    })
}

fn render_static_section(state: Rc<State>, color_options: &[RGBA8], label: &str) -> Dom {
    html!("color-select-section", {
        .property("slot", "sections")
        .property("label", label)
        .children(color_options.iter().map(|color| {
            html!("color-select-item", {
                .property("color", rgba8_to_hex(color))
                .property("slot", "items")
                .property_signal("selected", state.value.signal_cloned().map(clone!(color => move |selected_color| {
                    match selected_color {
                        Some(selected_color) => color == selected_color,
                        None => false
                    }
                })))
                .event(clone!(color, state => move |_:events::Click| {
                    set_selected(Rc::clone(&state), Some(color));
                }))
            })
        }))
    })
}

fn render_user_section(state: Rc<State>) -> Dom {
    html!("color-select-section", {
        .property("slot", "sections")
        .property("label", STR_USER_COLORS_LABEL)
        .children_signal_vec(state.user_colors.signal_vec_cloned().enumerate().map(clone!(state => move |(index, color)| {
            html!("color-select-item", {
                .property("slot", "items")
                .property("color", rgba8_to_hex(&color))
                .property_signal("selected", state.value.signal_cloned().map(clone!(color => move |selected_color| {
                    if selected_color.is_some() {
                        let selected_color = selected_color.unwrap_ji();
                        return selected_color == color;
                    };
                    false
                })))
                .event(clone!(color, state => move |_:events::Click| {
                    set_selected(Rc::clone(&state), Some(color));
                }))
                .attribute("deletable", "")
                .child(html!("button-icon", {
                    .property("slot", "delete-button")
                    .property("icon", "circle-x-blue")
                    .event(clone!(state => move |_:events::Click| {
                        let index: usize = index.lock_ref().unwrap_or_default();
                        spawn_local(clone!(state => async move {
                            delete_user_color(state.clone(), index).await;
                        }));
                    }))
                }))
            })
        })))
    })
}

fn render_add_color(state: Rc<State>) -> Dom {
    html!("input-color", {
        .property("slot", "add-color")
        .child(html!("button-rect", {
            .property("kind", "filled")
            .property("color", "blue")
            .text(STR_ADD_COLOR)
        }))
        .event(clone!(state => move |e: events::CustomChange| {
            let color = hex_to_rgba8(&e.value());
            spawn_local(clone!(state => async move {
                let _ = add_user_color(state.clone(), color).await;
            }));

        }))
    })
}
