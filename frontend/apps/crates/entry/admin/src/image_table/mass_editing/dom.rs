use std::rc::Rc;

use dominator::{clone, html, DomBuilder};
use futures_signals::signal::{from_future, SignalExt};
use utils::{
    component::Component,
    events,
    metadata::{get_affiliations, get_age_ranges, get_image_styles, get_image_tags},
};
use web_sys::ShadowRoot;

use super::{MassEditing, Mode};

impl Component<MassEditing> for Rc<MassEditing> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        let selected_counts = state.calculate_selected_counts();

        dom.child(html!("div", {
            .class("main")
            .children(&mut [
                html!("h2", {
                    .text("Editing ")
                    .text_signal(state.images_state.selected_images.signal_ref(|s| s.len().to_string()))
                    .text(" images.")
                }),
                html!("div", {
                    .class("add-remove-wrapper")
                    .child(html!("label", {
                        .child(html!("input", {
                            .prop("name", "mode")
                            .prop("type", "radio")
                            .prop_signal("checked", state.mode.signal().map(|mode| mode == Mode::Add))
                            .event(clone!(state => move |_:events::Input| {
                                state.mode.set(Mode::Add);
                            }))
                        }))
                        .text("Add to all")
                    }))
                    .child(html!("label", {
                        .child(html!("input", {
                            .prop("name", "mode")
                            .prop("type", "radio")
                            .prop_signal("checked", state.mode.signal().map(|mode| mode == Mode::Remove))
                            .event(clone!(state => move |_:events::Input| {
                                state.mode.set(Mode::Remove);
                            }))
                        }))
                        .text("Remove from all")
                    }))
                }),
                html!("table", {
                    .child(html!("tr", {
                        .children(&mut [
                            html!("th", {
                                .text("Style")
                            }),
                            html!("th", {
                                .text("Tags")
                            }),
                            html!("th", {
                                .text("Ages")
                            }),
                            html!("th", {
                                .text("Affiliations")
                            }),
                            // html!("th", {
                            //     .text("Categories")
                            // }),
                        ])
                    }))
                    .child(html!("tr", {
                        .children(&mut [
                            html!("td", {
                                .child(html!("div", {
                                    .class("checkbox-list")
                                    .children_signal_vec(
                                        from_future(get_image_styles())
                                            .map(|x| x.unwrap_or_default())
                                            .map(clone!(state, selected_counts => move |styles| {
                                                styles.iter().map(|style| {
                                                    html!("input-checkbox", {
                                                        .child(html!("span", {
                                                            .prop("slot", "label")
                                                            .text(&style.display_name)
                                                            .child(html!("span", {
                                                                .class("selected-count")
                                                                .text(&selected_counts.get_styles_string(&style.id))
                                                            }))
                                                        }))
                                                        .prop_signal("checked", state.styles.signal_ref(clone!(style => move |styles| {
                                                            styles.contains(&style.id)
                                                        })))
                                                        .event(clone!(state, style => move |evt: events::CustomToggle| {
                                                            if evt.value() {
                                                                state.styles.lock_mut().insert(style.id);
                                                            } else {
                                                                state.styles.lock_mut().remove(&style.id);
                                                            }
                                                        }))
                                                    })
                                                }).collect()
                                            }))
                                            .to_signal_vec()
                                    )
                                }))
                            }),
                            html!("td", {
                                .child(html!("div", {
                                    .class("checkbox-list")
                                    .children_signal_vec(
                                        from_future(get_image_tags())
                                            .map(|x| x.unwrap_or_default())
                                            .map(clone!(state, selected_counts => move |tags| {
                                                tags.iter().map(|tag| {
                                                    html!("input-checkbox", {
                                                        .child(html!("span", {
                                                            .prop("slot", "label")
                                                            .text(&tag.display_name)
                                                            .child(html!("span", {
                                                                .class("selected-count")
                                                                .text(&selected_counts.get_tags_string(&tag.index))
                                                            }))
                                                        }))
                                                        .prop_signal("checked", state.tags.signal_ref(clone!(tag => move |tags| {
                                                            tags.contains(&tag.index)
                                                        })))
                                                        .event(clone!(state, tag => move |evt: events::CustomToggle| {
                                                            if evt.value() {
                                                                state.tags.lock_mut().insert(tag.index);
                                                            } else {
                                                                state.tags.lock_mut().remove(&tag.index);
                                                            }
                                                        }))
                                                    })
                                                }).collect()
                                            }))
                                            .to_signal_vec()
                                    )
                                }))
                            }),
                            html!("td", {
                                .child(html!("div", {
                                    .class("checkbox-list")
                                    .children_signal_vec(
                                        from_future(get_age_ranges())
                                            .map(|x| x.unwrap_or_default())
                                            .map(clone!(state, selected_counts => move |ages| {
                                                ages.iter().map(|age| {
                                                    html!("input-checkbox", {
                                                        .child(html!("span", {
                                                            .prop("slot", "label")
                                                            .text(&age.display_name)
                                                            .child(html!("span", {
                                                                .class("selected-count")
                                                                .text(&selected_counts.get_ages_string(&age.id))
                                                            }))
                                                        }))
                                                        .prop_signal("checked", state.ages.signal_ref(clone!(age => move |ages| {
                                                            ages.contains(&age.id)
                                                        })))
                                                        .event(clone!(state, age => move |evt: events::CustomToggle| {
                                                            if evt.value() {
                                                                state.ages.lock_mut().insert(age.id);
                                                            } else {
                                                                state.ages.lock_mut().remove(&age.id);
                                                            }
                                                        }))
                                                    })
                                                }).collect()
                                            }))
                                            .to_signal_vec()
                                    )
                                }))
                            }),
                            html!("td", {
                                .child(html!("div", {
                                    .class("checkbox-list")
                                    .children_signal_vec(
                                        from_future(get_affiliations())
                                            .map(|x| x.unwrap_or_default())
                                            .map(clone!(state, selected_counts => move |affiliations| {
                                                affiliations.iter().map(|affiliation| {
                                                    html!("input-checkbox", {
                                                        .child(html!("span", {
                                                            .prop("slot", "label")
                                                            .text(&affiliation.display_name)
                                                            .child(html!("span", {
                                                                .class("selected-count")
                                                                .text(&selected_counts.get_affiliations_string(&affiliation.id))
                                                            }))
                                                        }))
                                                        .prop_signal("checked", state.affiliations.signal_ref(clone!(affiliation => move |affiliations| {
                                                            affiliations.contains(&affiliation.id)
                                                        })))
                                                        .event(clone!(state, affiliation => move |evt: events::CustomToggle| {
                                                            if evt.value() {
                                                                state.affiliations.lock_mut().insert(affiliation.id);
                                                            } else {
                                                                state.affiliations.lock_mut().remove(&affiliation.id);
                                                            }
                                                        }))
                                                    })
                                                }).collect()
                                            }))
                                            .to_signal_vec()
                                    )
                                }))
                            }),
                            // html!("td", {
                            //     .child(html!("div", {
                            //         .class("checkbox-list")
                            //         .children_signal_vec(
                            //             from_future(get_category_label_lookup())
                            //                 .map(|x| x.unwrap_or_default())
                            //                 .map(clone!(state => move |categories| {
                            //                     categories.iter().map(|(id, name)| {
                            //                         html!("input-checkbox", {
                            //                             .child(html!("span", {
                            //                                 .prop("slot", "label")
                            //                                 .text(&tag.display_name)
                            //                                 .child(html!("span", {
                            //                                     .class("selected-count")
                            //                                     .text(&selected_counts.get_tags_string(&tag.index))
                            //                                 }))
                            //                             }))
                            //                             .prop_signal("checked", state.categories.signal_ref(clone!(id => move |categories| {
                            //                                 categories.contains(&id)
                            //                             })))
                            //                             .event(clone!(state, id => move |evt: events::CustomToggle| {
                            //                                 if evt.value() {
                            //                                     state.categories.lock_mut().insert(id);
                            //                                 } else {
                            //                                     state.categories.lock_mut().remove(&id);
                            //                                 }
                            //                             }))
                            //                         })
                            //                     }).collect()
                            //                 }))
                            //                 .to_signal_vec()
                            //         )
                            //     }))
                            // }),
                        ])
                    }))
                }),
                html!("div", {
                    .class("actions")
                    .children(&mut [
                        html!("button-rect", {
                            .prop("color", "blue")
                            .prop("kind", "text")
                            .prop("size", "regular")
                            .text("Cancel")
                            .event(clone!(state => move |_: events::Click| {
                                state.images_state.mass_editing.set(false);
                            }))
                        }),
                        html!("button-rect", {
                            .prop("color", "blue")
                            .prop("kind", "filled")
                            .prop("size", "regular")
                            .text("Apply")
                            .event(clone!(state => move |_: events::Click| {
                                state.save_changes();
                            }))
                        }),
                    ])
                })
            ])
        }))
    }
}
