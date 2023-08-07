use crate::schools::details::school_name::state::SchoolNameState;
use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::events;
use utils::prelude::UnwrapJiExt;
use web_sys::HtmlInputElement;

impl SchoolNameState {
    pub fn render(self: Rc<Self>, slot: String) -> Dom {
        let state = self;

        state.load_data();

        let filtered_names = map_ref! {
            let names = state.school_names.signal_cloned(),
            let filter = state.filter_value.signal_cloned()
            => {
                match names {
                    Some(names) => {
                        names
                            .iter()
                            .cloned()
                            .filter(|school_name| school_name.name.to_lowercase().contains(&filter.to_lowercase()))
                            .take(25)
                            .collect::<Vec<_>>()
                    },
                    None => vec![]
                }
            }
        }
            .map(clone!(state => move |school_names| {
                school_names.into_iter()
                    .map(|school_name| {
                        let school_name_id = school_name.id;
                        html!("div", {
                            .child(html!("button-rect", {
                                .prop("kind", "text")
                                .prop("color", "blue")
                                .prop("size", "small")
                                .text(&school_name.name)
                                .event(clone!(state => move |_evt: events::Click| {
                                    state.change_internal_school_name(school_name_id);
                                }))
                            }))
                        })
                    })
                    .collect::<Vec<_>>()
            }))
            .to_signal_vec();

        html!("div", {
            .prop("slot", slot)
            .child(html!("h2", {
                .text("Create a new internal school name")
            }))
            .child(html!("input-wrapper", {
                .prop("label", "New school name")
                .child(html!("input", {
                    .prop("type", "text")
                    .prop_signal("value", state.new_name.signal().map(|school_name| {
                        school_name.unwrap_or_default()
                    }))
                    .event(clone!(state => move |evt: events::Input| {
                        let new_name = evt.dyn_target::<HtmlInputElement>().unwrap_ji()
                            .value()
                            .to_string();
                        state.new_name.set(Some(new_name));
                    }))
                }))
            }))
            .child_signal(state.current_name.signal().map(|name| name.is_some()).dedupe().map(clone!(state => move |has_school_name| {
                if has_school_name {
                    Some(html!("empty-fragment", {
                        .child(html!("h2", {
                            .text("Or, update the current internal school name")
                        }))
                        .child(html!("input-wrapper", {
                            .prop("label", "Edit current school name")
                            .child(html!("input", {
                                .prop("type", "text")
                                .prop_signal("value", state.current_name.signal().map(|school_name| {
                                    match school_name {
                                        Some(school_name) => school_name.name,
                                        None => String::new(),
                                    }
                                }))
                                .event(clone!(state => move |evt: events::Input| {
                                    let mut current_name = state.current_name.get().unwrap_ji();
                                    current_name.name = evt.dyn_target::<HtmlInputElement>().unwrap_ji()
                                        .value()
                                        .to_string();
                                    state.current_name.set(Some(current_name));
                                    state.new_name.set(None);
                                }))
                            }))
                        }))
                    }))
                } else {
                    None
                }
            })))
            .child(html!("h2", {
                .text("Or, use an existing school name")
            }))
            .child(html!("input-wrapper", {
                .prop("label", "Filter")
                .child(html!("input", {
                    .prop("type", "text")
                    .prop_signal("value", state.filter_value.signal_cloned())
                    .event(clone!(state => move |evt: events::Input| {
                        state.filter_value.set(evt.dyn_target::<HtmlInputElement>().unwrap_ji()
                            .value()
                            .to_string());
                    }))
                }))
            }))
            .children_signal_vec(filtered_names)
        })
    }
}
