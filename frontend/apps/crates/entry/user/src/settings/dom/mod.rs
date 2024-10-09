use crate::settings::{
    dom::options_popup::PopupCallbacks,
    state::{ActivePopup, IndividualOrSchool, ResetPasswordStatus},
};
use components::{
    editable_profile_image::{EditableProfileImage, EditableProfileImageConfig},
    page_header::{PageHeader, PageHeaderConfig},
};
use dominator::{clone, html, with_node, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::meta::{Affiliation, AffiliationId, AgeRange, AgeRangeId, Subject, SubjectId};
use std::rc::Rc;
use utils::prelude::get_user_cloned;
use utils::{
    component::Component,
    events,
    languages::{Language, EMAIL_LANGUAGES},
    unwrap::UnwrapJiExt,
};
use web_sys::{HtmlElement, HtmlInputElement};

use super::state::SettingsPage;

mod options_popup;
mod plan_info;

const STR_EDIT: &str = " Edit";
const STR_RESET: &str = "Reset";

const STR_RESET_PASSWORD_SENT: &str = "Email sent to ";

const STR_AFFILIATION_HEADER: &str = "Affiliation";
const STR_AFFILIATION_SUBHEADER: &str = "What type of content do you want to access?";

const STR_SUBJECT_HEADER: &str = "Relevant Subjects";
const STR_SUBJECT_SUBHEADER: &str = "Which subjects are you interested in?";

const STR_AGE_HEADER: &str = "Relevant Age Group";
const STR_AGE_SUBHEADER: &str = "Which age group are you interested in?";

impl SettingsPage {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        state.load_initial_data();

        let account_summary = get_user_cloned().unwrap().account_summary;

        let is_admin = account_summary
            .as_ref()
            .clone()
            .and_then(|summary| Some(summary.is_admin));
        let show_expanded_details = account_summary
            .as_ref()
            .and_then(|summary| summary.plan_type)
            .map(|plan_type| plan_type.is_individual_plan() || matches!(is_admin, Some(true)))
            .unwrap_or_default();

        html!("user-profile", {
            .child(EditableProfileImage::new(
                state.user.profile_image.read_only(),
                state.user.given_name.read_only(),
                state.user.family_name.read_only(),
                EditableProfileImageConfig {
                    save_changes: Box::new(clone!(state => move |user| {
                        state.user.profile_image.set(user);
                        state.save_profile();
                    })),
            }).render())
            // .child(page_header::dom::render(Rc::new(page_header::state::PageHeader::new()), Some("page-header"), None, true))
            .child(PageHeader::new(PageHeaderConfig {
                slot: Some("page-header"),
                ..Default::default()
            }).render())
            .prop("showExpandedDetails", show_expanded_details)
            .prop_signal("email", state.user.email.signal_cloned())
            .prop_signal("name", state.full_name_signal())
            .children(&mut [
                html!("input-wrapper", {
                    .prop("slot", "email")
                    .child(html!("input" => HtmlInputElement, {
                        .prop_signal("value", state.user.email.signal_cloned())
                        .prop("readOnly", true)
                    }))
                    .child(html!("img-ui", {
                        .prop("slot", "icon")
                        .prop("path", "entry/user/profile/lock-blue.svg")
                        .style("width", "14px")
                    }))
                }),
                html!("input-wrapper", {
                    .prop("slot", "first-name")
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .prop_signal("value", state.user.given_name.signal_cloned())
                            .event(clone!(state => move |_: events::Input| {
                                state.user.given_name.set(elem.value());
                                state.save_profile();
                            }))
                        })
                    }))
                    .child(html!("img-ui", {
                        .prop("slot", "icon")
                        .prop("path", "core/inputs/pencil-blue-darker.svg")
                    }))
                }),
                html!("input-wrapper", {
                    .prop("slot", "family-name")
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .prop_signal("value", state.user.family_name.signal_cloned())
                            .event(clone!(state => move |_: events::Input| {
                                state.user.family_name.set(elem.value());
                                state.save_profile();
                            }))
                        })
                    }))
                    .child(html!("img-ui", {
                        .prop("slot", "icon")
                        .prop("path", "core/inputs/pencil-blue-darker.svg")
                    }))
                }),
                html!("input-wrapper" => HtmlElement, {
                    .with_node!(wrapper => {
                        .prop("slot", "username")
                        .child(html!("input", {
                            .prop_signal("value", state.user.username.signal_cloned())
                            .attr("readonly", "")
                            .event(move |_: events::KeyDown| {
                                let _ = wrapper.set_attribute("error", "");
                            })
                        }))
                        .child(html!("img-ui", {
                            .prop("slot", "icon")
                            .prop("path", "entry/user/profile/lock-blue.svg")
                            .style("width", "14px")
                        }))
                    })
                }),
                html!("input-select", {
                    .prop("slot", "preferred-language")
                    .prop_signal("value", state.user.language_emails.signal_cloned().map(|code| {
                        Language::code_to_display_name(&code)
                    }))
                    .children(EMAIL_LANGUAGES.iter().map(|lang| {
                        html!("input-select-option", {
                            .text(lang.display_name())
                            .prop_signal("selected", state.user.language_emails.signal_ref(clone!(lang => move |language_emails| {
                                language_emails == lang.code()
                            })))
                            .event(clone!(state => move |_: events::CustomSelectedChange| {
                                state.user.language_emails.set(lang.code().to_string());
                                state.save_profile();
                            }))
                        })
                    }))
                }),
                html!("empty-fragment", {
                    .style("display", "contents")
                    .prop("slot", "age-groups")
                    .children_signal_vec(state.user.age_ranges.signal_vec_cloned().map(clone!(state => move|age_range_id| {
                        html!("pill-close", {
                            .prop_signal("label", state.metadata.signal_ref(clone!(age_range_id => move |metadata| {
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
                    .prop("kind", "outline")
                    .prop("color", "blue")
                    .prop("size", "regular")
                    .prop("slot", "age-groups-edit")
                    .text(STR_EDIT)
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(ActivePopup::Age)
                    }))
                }),
                html!("empty-fragment", {
                    .style("display", "contents")
                    .prop("slot", "relevant-subjects")
                    .children_signal_vec(state.user.subjects.signal_vec_cloned().map(clone!(state => move|subject_id| {
                        html!("pill-close", {
                            .prop_signal("label", state.metadata.signal_ref(clone!(subject_id => move |metadata| {
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
                    .prop("kind", "outline")
                    .prop("color", "blue")
                    .prop("size", "regular")
                    .prop("slot", "relevant-subjects-edit")
                    .text(STR_EDIT)
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(ActivePopup::Subjects)
                    }))
                }),
                html!("empty-fragment", {
                    .style("display", "contents")
                    .prop("slot", "affiliations")
                    .children_signal_vec(state.user.affiliations.signal_vec_cloned().map(clone!(state => move|affiliation_id| {
                        html!("pill-close", {
                            .prop_signal("label", state.metadata.signal_ref(clone!(affiliation_id => move |metadata| {
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
                    .prop("kind", "outline")
                    .prop("color", "blue")
                    .prop("size", "regular")
                    .prop("slot", "affiliations-edit")
                    .text(STR_EDIT)
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(ActivePopup::Affiliation)
                    }))
                }),

            ])
            .child_signal(state.reset_password_status.signal().map(clone!(state => move |status| {
                Some(match status {
                    ResetPasswordStatus::Ready | ResetPasswordStatus::Loading => {
                        html!("div", {
                            .prop("slot", "reset-password")
                            .child(html!("button-rect", {
                                .prop("kind", "outline")
                                .prop("color", "blue")
                                .prop("size", "regular")
                                .prop("slot", "relevant-subjects-edit")
                                .prop("disabled", status == ResetPasswordStatus::Loading)
                                .text(STR_RESET)
                                .event(clone!(state => move |_: events::Click| {
                                    state.send_reset_password();
                                }))
                            }))
                        })
                    },
                    ResetPasswordStatus::Sent => {
                        html!("p", {
                            .prop("slot", "reset-password")
                            .text(STR_RESET_PASSWORD_SENT)
                            .text(&state.user.email.get_cloned())
                        })
                    },
                })
            })))
            .apply_if(account_summary.is_some(), clone!(account_summary => |dom| {
                dom.prop("showPlan", account_summary.unwrap_ji().plan_type.is_some())
            }))
            .prop_signal("isTrial", state.plan_info.signal_ref(|plan| {
                plan.as_ref().map_or(false, |plan| plan.is_trial)
            }))
            .prop_signal("planSectionTitle", state.plan_info.signal_ref(|plan| {
                plan.as_ref().map(|plan| {
                    match plan.individual_or_school {
                        IndividualOrSchool::Individual => "My current plan",
                        IndividualOrSchool::School(_) => "School plan",
                    }
                })
            }))
            .apply_if(!show_expanded_details, |dom| {
                dom.children(state.render_partial_plan_section())
            })
            .children_signal_vec(state.plan_info.signal_ref(clone!(state => move |plan_info| {
                if show_expanded_details {
                    match plan_info {
                        Some(plan_info) => {
                            state.render_full_plan_section(plan_info)
                        },
                        None => vec![],
                    }
                } else {
                    vec![]
                }
            })).to_signal_vec())
            .child_signal(state.render_popups())
        })
    }

    fn render_popups(self: &Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        let state = self;
        state.active_popup.signal_cloned().map(clone!(state => move|active_popup| {
            match active_popup {
                ActivePopup::None => None,
                _ => {
                    Some(html!("dialog-overlay", {
                        .prop("slot", "popup")
                        .prop("open", true)
                        .prop("autoClose", false)
                        .event(clone!(state => move |_: events::Close| {
                            state.active_popup.set(ActivePopup::None);
                        }))
                        .apply(|dom| {
                            let child = match active_popup {
                                ActivePopup::None => unreachable!(),
                                ActivePopup::Affiliation => {
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
                                    options_popup::render::<AffiliationId, Affiliation>(Rc::clone(&state), STR_AFFILIATION_HEADER, STR_AFFILIATION_SUBHEADER, callbacks)
                                },
                                ActivePopup::Subjects => {
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

                                    options_popup::render::<SubjectId, Subject>(Rc::clone(&state), STR_SUBJECT_HEADER, STR_SUBJECT_SUBHEADER, callbacks)
                                },
                                ActivePopup::Age => {
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

                                    options_popup::render::<AgeRangeId, AgeRange>(Rc::clone(&state), STR_AGE_HEADER, STR_AGE_SUBHEADER, callbacks)
                                },
                            };

                            dom.child(child)
                        })
                    }))
                },
            }
        }))
    }

    fn full_name_signal(self: &Rc<Self>) -> impl Signal<Item = String> {
        (map_ref! {
            let given_name = self.user.given_name.signal_cloned(),
            let family_name = self.user.family_name.signal_cloned() =>
                (given_name.clone(), family_name.clone())
        })
        .map(move |(given_name, family_name)| format!("{} {}", given_name, family_name))
    }
}
