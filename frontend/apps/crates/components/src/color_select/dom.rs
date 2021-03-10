use dominator::{Dom, html, clone};
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use utils::events;
use futures_signals::signal::SignalExt;
use super::actions::add_user_color;
use super::state::State;

pub fn render(state: State) -> Dom {
    let state = Rc::new(state);

    html!("color-select", {
        .children(&mut [
            html!("input-color", {
                .property("slot", "add-color")
                .child(html!("button-text", {
                    .text("+ Add color")
                }))
                .event(clone!(state => move |e: events::CustomChange| {
                    add_user_color(state.clone(), e.value());
                }))
            }),
            html!("div", {
                .style("display", "contents")
                .property("slot", "items")
                .children_signal_vec(state.system_colors.signal_vec_cloned().map(clone!(state => move |color_option| {
                    html!("color-select-item", {
                        .property("color", &*color_option)
                        .property_signal("selected", state.value.signal_cloned().map(clone!(color_option => move |selected_color| {
                            match selected_color {
                                Some(selected_color) => *color_option == selected_color,
                                None => false
                            }
                        })))
                        .event(clone!(color_option, state => move |_:events::Click| {
                            state.value.set(Some((*color_option).clone()));
                        }))
                    })
                })))
            }),
            html!("div", {
                .style("display", "contents")
                .property("slot", "items")
                .children_signal_vec(state.user_colors.signal_vec_cloned().map(clone!(state => move |color_option| {
                    let color_option = color_option.as_ref().clone();
                    html!("color-select-item", {
                        .apply_if(color_option.is_some(), clone!(color_option => move |dom| {
                            dom.property("color", color_option.unwrap().color)
                        }))
                        .property("disabled", color_option.is_none())
                        .property_signal("selected", state.value.signal_cloned().map(clone!(color_option => move |selected_color| {
                            if selected_color.is_some() && color_option.is_some() {
                                let color_option = color_option.clone().unwrap();
                                let selected_color = selected_color.unwrap();
                                return selected_color == color_option.color;
                            };
                            false
                        })))
                        .event(clone!(color_option, state => move |_:events::Click| {
                            if color_option.is_some() {
                                state.value.set(Some(color_option.clone().unwrap().color));
                            }
                        }))
                    })
                })))
            })
        ])
    })
}
