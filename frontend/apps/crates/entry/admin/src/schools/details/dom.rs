use crate::schools::details::state::{CurrentAction, SchoolDetails};
use dominator::{clone, html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use std::rc::Rc;
use utils::events;
use utils::routes::AdminSchoolsRoute;
use web_sys::HtmlTextAreaElement;

impl SchoolDetails {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        state.load_data();

        html!("admin-school-details", {
            .child(html!("window-loader-block", {
                .prop("slot", "loader")
                .prop_signal("visible", state.parent.loader.is_loading())
            }))
            .child_signal(state.school.signal_cloned().map(clone!(state => move |school| {
                school.and_then(clone!(state => move |school| {
                    if !school.school_name.verified {
                        Some(html!("button-rect", {
                            .prop("slot", "back")
                            .prop("color", "blue")
                            .prop("size", "small")
                            .style("margin-right", "16px")
                            .text("Verify school")
                            .event(clone!(state => move |_: events::Click| {
                                state.set_verified();
                            }))
                        }))
                    } else {
                        None
                    }
                }))

            })))
            .child(html!("button-rect", {
                .prop("slot", "back")
                .prop("color", "blue")
                .prop("kind", "text")
                .text("Back to schools list")
                .event(clone!(state => move |_: events::Click| {
                    state.parent.navigate_to(AdminSchoolsRoute::Table);
                }))
            }))
            .child_signal(state.school.signal_cloned().map(move |school| {
                school.map(|school| {
                    html!("empty-fragment", {
                        .prop("slot", "inputs")
                        .children(&mut [
                            html!("input-wrapper", {
                                .prop("label", "School name")
                                .child(html!("input", {
                                    .prop("type", "text")
                                    .prop("disabled", true)
                                    .prop("value", school.school_name.name)
                                }))
                            }),
                            html!("input-wrapper", {
                                .prop("label", "Contact email")
                                .child(html!("input", {
                                    .prop("type", "text")
                                    .prop("disabled", true)
                                    .prop("value", school.email)
                                }))
                            }),
                            html!("input-wrapper", {
                                .prop("label", "Description")
                                .child(html!("textarea", {
                                    .prop("disabled", true)
                                    .prop("value", school.description.unwrap_or_default())
                                }))
                            }),
                            html!("input-wrapper", {
                                .prop("label", "Website")
                                .child(html!("input", {
                                    .prop("type", "text")
                                    .prop("disabled", true)
                                    .prop("value", school.website.unwrap_or_default())
                                }))
                            }),
                            html!("input-wrapper", {
                                .prop("label", "Organization type")
                                .child(html!("input", {
                                    .prop("type", "text")
                                    .prop("disabled", true)
                                    .prop("value", school.organization_type.unwrap_or_default())
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
    }
}
