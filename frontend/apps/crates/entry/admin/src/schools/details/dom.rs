use crate::schools::details::state::{CurrentAction, SchoolDetails};
use dominator::{clone, html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use std::rc::Rc;
use utils::events;
use utils::prelude::*;
use utils::routes::AdminSchoolsRoute;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

impl SchoolDetails {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        state.load_data();

        html!("empty-fragment", {
            .child_signal(state.school.signal_cloned().map(clone!(state => move |school| {
                school.map(|school| {
                    html!("admin-school-details", {
                        .child(html!("window-loader-block", {
                            .prop("slot", "loader")
                            .prop_signal("visible", state.parent.loader.is_loading())
                        }))
                        .apply_if(!school.verified, clone!(state => move |dom| {
                            dom.child(html!("button-rect", {
                                .prop("slot", "back")
                                .prop("kind", "filled")
                                .prop("color", "blue")
                                .text("Verify school")
                                .event(clone!(state => move |_: events::Click| {
                                    state.set_verified();
                                }))
                            }))
                        }))
                        .child(html!("div", {
                            .prop("slot", "buttons")
                            .child(html!("button-rect", {
                                .prop("kind", "text")
                                .prop("color", "blue")
                                .text("Cancel")
                                .event(clone!(state => move |_: events::Click| {
                                    state.parent.navigate_to(AdminSchoolsRoute::Table);
                                }))
                            }))
                            .child(html!("button-rect", {
                                .prop("kind", "filled")
                                .prop("color", "blue")
                                .prop_signal("disabled", school.changed_signal().map(|changed| !changed))
                                .text("Save school")
                                .event(clone!(state => move |_: events::Click| {
                                    state.save_school();
                                }))

                            }))
                        }))
                        .child(html!("empty-fragment", {
                            .prop("slot", "inputs")
                            .children(&mut [
                                html!("input-wrapper", {
                                    .prop("label", "Internal School name")
                                    .child(html!("input", {
                                        .prop("type", "text")
                                        .prop("disabled", true)
                                        .prop("value", school.internal_school_name.clone()
                                            .map(|school_name| school_name.name)
                                            .unwrap_or_default()
                                        )
                                    }))
                                }),
                                html!("input-wrapper", {
                                    .prop("label", "School name")
                                    .child(html!("input", {
                                        .prop("type", "text")
                                        .prop_signal("value", school.school_name.signal())
                                        .event(clone!(school => move |evt: events::Change| {
                                            school.school_name.set(evt.dyn_target::<HtmlInputElement>().unwrap_ji()
                                                    .value()
                                                    .trim()
                                                    .to_string());
                                        }))
                                    }))
                                }),
                                html!("input-wrapper", {
                                    .prop("label", "Contact email")
                                    .child(html!("input", {
                                        .prop("type", "text")
                                        .prop_signal("value", school.email.signal())
                                        .event(clone!(school => move |evt: events::Change| {
                                            school.email.set(evt.dyn_target::<HtmlInputElement>().unwrap_ji()
                                                .value()
                                                .trim()
                                                .to_string());
                                        }))
                                    }))
                                }),
                                html!("input-wrapper", {
                                    .prop("label", "Description")
                                    .child(html!("textarea", {
                                        .prop_signal(
                                            "value",
                                            school.description.signal().map(|value| value.unwrap_or_default())
                                        )
                                        .event(clone!(school => move |evt: events::Change| {
                                            let value: String = evt.dyn_target::<HtmlTextAreaElement>().unwrap_ji()
                                                .value()
                                                .trim()
                                                .to_string();

                                            if value.is_empty() {
                                                school.description.set(None);
                                            } else {
                                                school.description.set(Some(value));
                                            }
                                        }))
                                    }))
                                }),
                                html!("input-wrapper", {
                                    .prop("label", "Website")
                                    .child(html!("input", {
                                        .prop("type", "text")
                                        .prop_signal(
                                            "value",
                                            school.website.signal().map(|value| value.unwrap_or_default())
                                        )
                                        .event(clone!(school => move |evt: events::Change| {
                                            let value: String = evt.dyn_target::<HtmlInputElement>().unwrap_ji()
                                                .value()
                                                .trim()
                                                .to_string();

                                            if value.is_empty() {
                                                school.website.set(None);
                                            } else {
                                                school.website.set(Some(value));
                                            }
                                        }))
                                    }))
                                }),
                                html!("input-wrapper", {
                                    .prop("label", "Organization type")
                                    .child(html!("input", {
                                        .prop("type", "text")
                                        .prop_signal(
                                            "value",
                                            school.organization_type.signal().map(|value| value.unwrap_or_default())
                                        )
                                        .event(clone!(school => move |evt: events::Change| {
                                            let value: String = evt.dyn_target::<HtmlInputElement>().unwrap_ji()
                                                .value()
                                                .trim()
                                                .to_string();

                                            if value.is_empty() {
                                                school.organization_type.set(None);
                                            } else {
                                                school.organization_type.set(Some(value));
                                            }
                                        }))
                                    }))
                                }),
                            ])
                        }))
                        .child_signal(state.account.signal_cloned().map(|account| {
                            account.map(|account| {
                                html!("empty-fragment", {
                                    .prop("slot", "account")
                                    .children(&mut [
                                        html!("input-wrapper", {
                                            .prop("label", "Stripe Customer ID")
                                            .child(html!("input", {
                                                .prop("type", "text")
                                                .prop("disabled", true)
                                                .prop(
                                                    "value",
                                                    account.stripe_customer_id
                                                        .map(|id| id.to_string())
                                                        .unwrap_or_default()
                                                )
                                            }))
                                        }),
                                    ])
                                })
                            })
                        }))
                        .child_signal(state.current_action.signal_cloned().map(clone!(state => move |action| {
                            match action {
                                CurrentAction::Viewing => {
                                    Some(html!("div", {
                                        .prop("slot", "users")
                                        .style("margin", "0 0 24rem 0")
                                        .children(&mut [
                                            html!("button", {
                                                .text("Add users")
                                                .event(clone!(state => move |_: events::Click| {
                                                    state.current_action.set(CurrentAction::AddUsers(Mutable::new(None)));
                                                }))
                                            })
                                        ])
                                    }))
                                },
                                CurrentAction::AddUsers(input_elem) => {
                                    Some(html!("div", {
                                        .prop("slot", "users")
                                        .style("margin", "0 0 24rem 0")
                                        .children(&mut [
                                            html!("input-wrapper", {
                                                .prop("label", "User email addresses to add")
                                                .children(&mut [
                                                    html!("textarea" => HtmlTextAreaElement, {
                                                        .after_inserted(clone!(input_elem => move |elem| {
                                                            input_elem.set(Some(Rc::new(elem)));
                                                        }))
                                                    }),
                                                ])
                                            }),
                                            html!("div", {
                                                .style("display", "flex")
                                                .style("flex-direction", "row")
                                                .style("margin", "24rem 0 0 0")
                                                .children(&mut [
                                                    html!("button", {
                                                        .text("Add users")
                                                        .event(clone!(state, input_elem => move |_: events::Click| {
                                                            if let Some(elem) = input_elem.get_cloned() {
                                                                state.invite_school_users(elem.value().into());
                                                            }

                                                        }))
                                                    }),
                                                    html!("button", {
                                                        .text("Cancel")
                                                        .event(clone!(state => move |_: events::Click| {
                                                            state.current_action.set(CurrentAction::Viewing)
                                                        }))
                                                    })
                                                ])
                                            })
                                        ])
                                    }))
                                },
                                CurrentAction::AddingUsers => {
                                    Some(html!("div", {
                                        .prop("slot", "users")
                                        .style("margin", "0 0 24rem 0")
                                        .text("Adding users...")
                                    }))
                                },
                                CurrentAction::Results(failures) => {
                                    Some(html!("div", {
                                        .prop("slot", "users")
                                        .style("margin", "0 0 24rem 0")
                                        .child(html!("div", {
                                            .child(html!("strong", {
                                                .text("Results:")
                                            }))
                                        }))
                                        .children(failures.into_iter().map(|failure| {
                                            html!("div", {
                                                .children(&mut [
                                                    html!("span", {
                                                        .text(&failure.email)
                                                    }),
                                                    html!("span", {
                                                        .text(&format!(" - {}", failure.reason))
                                                    })
                                                ])
                                            })
                                        }))
                                        .child(html!("button", {
                                            .text("Close")
                                            .event(clone!(state => move |_: events::Click| {
                                                state.current_action.set(CurrentAction::Viewing)
                                            }))
                                        }))
                                    }))
                                }
                            }
                        })))
                        .children(&mut [
                            html!("admin-school-user-table", {
                                .prop("slot", "users")
                                .children_signal_vec(state.users.signal_vec_cloned().map(move |user| {
                                    html!("admin-table-line", {
                                        .children(&mut [
                                            html!("span", {
                                                .text(&user.user.email)
                                            }),
                                            html!("input-checkbox", {
                                                .prop("checked", user.verified)
                                                .prop("disabled", true)
                                            }),
                                            html!("input-checkbox", {
                                                .prop("checked", user.is_admin)
                                                .prop("disabled", true)
                                            }),
                                            html!("div", {
                                                .style("display", "flex")
                                                .style("flex-direction", "row")
                                                .text("TODO: Actions")
                                                // .children(&mut [
                                                //     html!("button", {
                                                //         .text("Remove")
                                                //         .prop("disabled", "true")
                                                //         .event(clone!(state => move |_: events::Click| {
                                                //             todo!()
                                                //         }))
                                                //     })
                                                // ])
                                            }),
                                        ])
                                    })
                                }))
                            })
                        ])
                    })
                })
            })))
        })
    }
}
