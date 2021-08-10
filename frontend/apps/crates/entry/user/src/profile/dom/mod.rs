use std::rc::Rc;
use components::page_header;
use dominator::{Dom, clone, html, with_node};
use futures_signals::{map_ref, signal::{Signal, SignalExt}, signal_vec::SignalVecExt};
use shared::domain::meta::{Affiliation, AffiliationId, AgeRange, AgeRangeId, Subject, SubjectId};
use utils::{events, languages::{LANGUAGES, Language}, unwrap::UnwrapJiExt};
use web_sys::{HtmlElement, HtmlInputElement};

use crate::profile::{change_password, dom::options_popup::PopupCallbacks, state::ActivePopup};

use super::{actions, state::State};

mod options_popup;

const STR_EDIT: &'static str = " Edit";
const STR_REMOVE_IMAGE: &'static str = "remove image";

pub struct ProfilePage {
}

impl ProfilePage {
    pub fn render() -> Dom {
        let state = Rc::new(State::new());

        actions::load_initial_data(state.clone());

        html!("user-profile", {
            .child(page_header::dom::render(Rc::new(page_header::state::State::new()), Some("page-header")))
            .property_signal("email", state.user.email.signal_cloned())
            .property_signal("name", full_name_signal(Rc::clone(&state)))
            .children(&mut [
                html!("img-ji", {
                    .property("slot", "profile-image")
                    .property("lib", "mock")
                    .property("id", "face-round.webp")
                    .property("size", "original")
                }),
                html!("img-ji", {
                    .property("slot", "editable-profile-image")
                    .property("lib", "mock")
                    .property("id", "face-round.webp")
                    .property("size", "original")
                }),
                html!("button-empty", {
                    .property("slot", "profile-image-edit")
                    .text("✎")
                }),
                html!("button-rect", {
                    .property("kind", "text")
                    .property("color", "blue")
                    .property("slot", "profile-image-delete")
                    .text(STR_REMOVE_IMAGE)
                }),
                html!("input-wrapper", {
                    .property("slot", "email")
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .property_signal("value", state.user.email.signal_cloned())
                            .event(clone!(state => move |_: events::Input| {
                                state.user.email.set(elem.value());
                                actions::save_profile(Rc::clone(&state));
                            }))
                        })
                    }))
                    .child(html!("img-ui", {
                        .property("slot", "icon")
                        .property("path", "core/inputs/pencil-blue-darker.svg")
                    }))
                }),
                html!("div", {
                    .property("slot", "password-edit")
                    .child(html!("button-rect", {
                        .property("kind", "outline")
                        .property("color", "blue")
                        .property("size", "small")
                        .property("slot", "relevant-subjects-edit")
                        .text(STR_EDIT)
                        .event(clone!(state => move |_: events::Click| {
                            state.active_popup.set(ActivePopup::ResetPassword)
                        }))
                    }))
                }),
                html!("input-wrapper", {
                    .property("slot", "first-name")
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .property_signal("value", state.user.given_name.signal_cloned())
                            .event(clone!(state => move |_: events::Input| {
                                state.user.given_name.set(elem.value());
                                actions::save_profile(Rc::clone(&state));
                            }))
                        })
                    }))
                    .child(html!("img-ui", {
                        .property("slot", "icon")
                        .property("path", "core/inputs/pencil-blue-darker.svg")
                    }))
                }),
                html!("input-wrapper", {
                    .property("slot", "family-name")
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .property_signal("value", state.user.family_name.signal_cloned())
                            .event(clone!(state => move |_: events::Input| {
                                state.user.family_name.set(elem.value());
                                actions::save_profile(Rc::clone(&state));
                            }))
                        })
                    }))
                    .child(html!("img-ui", {
                        .property("slot", "icon")
                        .property("path", "core/inputs/pencil-blue-darker.svg")
                    }))
                }),
                html!("input-wrapper" => HtmlElement, {
                    .with_node!(wrapper => {
                        .property("slot", "username")
                        .child(html!("input", {
                            .property_signal("value", state.user.username.signal_cloned())
                            .attribute("readonly", "")
                            .event(move |_: events::KeyDown| {
                                let _ = wrapper.set_attribute("error", "");
                            })
                        }))
                        .child(html!("img-ui", {
                            .property("slot", "icon")
                            .property("path", "entry/user/profile/lock-blue.svg")
                            .style("width", "14px")
                        }))
                    })
                }),
                html!("input-wrapper", {
                    .property("slot", "location")
                    .child(html!("input-location", {
                        .property_signal("locationAsString", state.user.location.signal_cloned().map(|location| {
                            location.unwrap_or_default()
                                .as_str()
                                .unwrap_or_default()
                                .to_owned()
                        }))
                        .event(clone!(state => move |evt: events::GoogleLocation| {
                            let raw = serde_json::to_value(evt.raw_json()).unwrap_ji();
                            state.user.location.set(Some(raw));
                            actions::save_profile(Rc::clone(&state));
                        }))
                    }))
                    .child(html!("img-ui", {
                        .property("slot", "icon")
                        .property("path", "core/inputs/pencil-blue-darker.svg")
                    }))
                }),
                html!("input-select", {
                    .property("slot", "preferred-language")
                    .property_signal("value", state.user.language.signal_cloned().map(|code| {
                        Language::code_to_display_name(&code)
                    }))
                    .children(LANGUAGES.iter().map(|lang| {
                        html!("input-select-option", {
                            .text(lang.display_name())
                            .event(clone!(state => move |_: events::CustomSelectedChange| {
                                state.user.language.set(lang.code().to_string());
                                actions::save_profile(Rc::clone(&state));
                            }))
                        })
                    }))
                }),
                html!("input-wrapper", {
                    .property("slot", "school-organization")
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .property_signal("value", state.user.organization.signal_cloned().map(|i| i.unwrap_or_default()))
                            .event(clone!(state => move |_: events::Input| {
                                state.user.organization.set(Some(elem.value()));
                                actions::save_profile(Rc::clone(&state));
                            }))
                        })
                    }))
                    .child(html!("img-ui", {
                        .property("slot", "icon")
                        .property("path", "core/inputs/pencil-blue-darker.svg")
                    }))
                }),
                html!("empty-fragment", {
                    .style("display", "contents")
                    .property("slot", "age-groups")
                    .children_signal_vec(state.user.age_ranges.signal_vec_cloned().map(clone!(state => move|age_range_id| {
                        html!("pill-close", {
                            .property("label", age_range_id.0.to_string())
                            .property_signal("label", state.metadata.signal_ref(clone!(age_range_id => move |metadata| {
                                match metadata {
                                    None => String::new(),
                                    Some(metadata) => {
                                        metadata
                                            .age_ranges
                                            .iter()
                                            .find(|age_range| age_range.id == age_range_id)
                                            .unwrap_ji()
                                            .display_name
                                            .clone()
                                    }
                                }
                            })))
                        })
                    })))
                }),
                html!("button-rect", {
                    .property("kind", "outline")
                    .property("color", "blue")
                    .property("size", "small")
                    .property("slot", "age-groups-edit")
                    .text(STR_EDIT)
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(ActivePopup::Age)
                    }))
                }),
                html!("empty-fragment", {
                    .style("display", "contents")
                    .property("slot", "relevant-subjects")
                    .children_signal_vec(state.user.subjects.signal_vec_cloned().map(clone!(state => move|subject_id| {
                        html!("pill-close", {
                            .property("label", subject_id.0.to_string())
                            .property_signal("label", state.metadata.signal_ref(clone!(subject_id => move |metadata| {
                                match metadata {
                                    None => String::new(),
                                    Some(metadata) => {
                                        metadata
                                            .subjects
                                            .iter()
                                            .find(|subject| subject.id == subject_id)
                                            .unwrap_ji()
                                            .display_name
                                            .clone()
                                    }
                                }
                            })))
                        })
                    })))
                }),
                html!("button-rect", {
                    .property("kind", "outline")
                    .property("color", "blue")
                    .property("size", "small")
                    .property("slot", "relevant-subjects-edit")
                    .text(STR_EDIT)
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(ActivePopup::Subjects)
                    }))
                }),
                html!("empty-fragment", {
                    .style("display", "contents")
                    .property("slot", "affiliations")
                    .children_signal_vec(state.user.affiliations.signal_vec_cloned().map(clone!(state => move|affiliation_id| {
                        html!("pill-close", {
                            .property("label", affiliation_id.0.to_string())
                            .property_signal("label", state.metadata.signal_ref(clone!(affiliation_id => move |metadata| {
                                match metadata {
                                    None => String::new(),
                                    Some(metadata) => {
                                        metadata
                                            .affiliations
                                            .iter()
                                            .find(|affiliation| affiliation.id == affiliation_id)
                                            .unwrap_ji()
                                            .display_name
                                            .clone()
                                    }
                                }
                            })))
                        })
                    })))
                }),
                html!("button-rect", {
                    .property("kind", "outline")
                    .property("color", "blue")
                    .property("size", "small")
                    .property("slot", "affiliations-edit")
                    .text(STR_EDIT)
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(ActivePopup::Affiliation)
                    }))
                }),
            ])
            .child_signal(render_popups(Rc::clone(&state)))
        })
    }
}

fn render_popups(state: Rc<State>) -> impl Signal<Item = Option<Dom>> {
    state.active_popup.signal_cloned().map(clone!(state => move|active_popup| {
        match active_popup {
            ActivePopup::None => None,
            _ => {
                Some(html!("dialog-overlay", {
                    .property("slot", "popup")
                    .property("open", true)
                    .property("autoClose", false)
                    .event(clone!(state => move |_: events::Close| {
                        log::info!("hay");
                        state.active_popup.set(ActivePopup::None);
                    }))
                    .apply(|dom| {
                        let child = match active_popup {
                            ActivePopup::None => unreachable!(),
                            ActivePopup::Affiliation => {
                                let header = "Affiliation";
                                let subheader = "What type of content do you want to access?";

                                let callbacks = PopupCallbacks {
                                    get_options_list: Box::new(|meta| {
                                        &meta.affiliations
                                    }),
                                    get_selected_list: Box::new(|user| {
                                        &user.affiliations
                                    }),
                                    get_id_from_struct: Box::new(|affiliation: &Affiliation| {
                                        &affiliation.id
                                    }),
                                    get_display_name: Box::new(|affiliation: &Affiliation| {
                                        &affiliation.display_name
                                    }),
                                };

                                options_popup::render::<AffiliationId, Affiliation>(Rc::clone(&state), header, subheader, callbacks)
                            },
                            ActivePopup::Subjects => {
                                let header = "??";
                                let subheader = "???";

                                let callbacks = PopupCallbacks {
                                    get_options_list: Box::new(|meta| {
                                        &meta.subjects
                                    }),
                                    get_selected_list: Box::new(|user| {
                                        &user.subjects
                                    }),
                                    get_id_from_struct: Box::new(|subject: &Subject| {
                                        &subject.id
                                    }),
                                    get_display_name: Box::new(|subject: &Subject| {
                                        &subject.display_name
                                    }),
                                };

                                options_popup::render::<SubjectId, Subject>(Rc::clone(&state), header, subheader, callbacks)
                            },
                            ActivePopup::Age => {
                                let header = "Relevant Age Group";
                                let subheader = "Which age group are you interested in?";

                                let callbacks = PopupCallbacks {
                                    get_options_list: Box::new(|meta| {
                                        &meta.age_ranges
                                    }),
                                    get_selected_list: Box::new(|user| {
                                        &user.age_ranges
                                    }),
                                    get_id_from_struct: Box::new(|age_range: &AgeRange| {
                                        &age_range.id
                                    }),
                                    get_display_name: Box::new(|age: &AgeRange| {
                                        &age.display_name
                                    }),
                                };

                                options_popup::render::<AgeRangeId, AgeRange>(Rc::clone(&state), header, subheader, callbacks)
                            },
                            ActivePopup::ResetPassword => {
                                // let state = Rc::new(change_password::state::State::new());
                                change_password::dom::render(Rc::clone(&state))
                            },
                        };

                        dom.child(child)
                    })
                }))
            },
        }
    }))
}

fn full_name_signal(state: Rc<State>) -> impl Signal<Item = String> {
    (map_ref! {
        let given_name = state.user.given_name.signal_cloned(),
        let family_name = state.user.family_name.signal_cloned() =>
            (given_name.clone(), family_name.clone())
    }).map(move |(given_name, family_name)| {
        format!("{} {}", given_name, family_name)
    })
}
