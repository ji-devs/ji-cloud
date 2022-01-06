use super::state::CurationJig;
use components::{player_popup::{PlayerPopup, PreviewPopupCallbacks}, module::_common::thumbnail::ModuleThumbnail};
use dominator::{html, Dom, clone, with_node};
use futures_signals::signal::SignalExt;
use utils::{events, routes::AdminCurationRoute, jig::JigPlayerOptions};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use std::rc::Rc;

mod affiliation;
mod age;
// mod categories_select;
// mod category_pills;
mod language;

impl CurationJig {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("admin-curation-jig-details", {
            .property("slot", "jig-details")
            .child(html!("window-loader-block", {
                .property("slot", "loader")
                .property_signal("visible", state.loader.is_loading())
            }))
            .children(&mut [
                html!("button-rect", {
                    .property("slot", "back")
                    .property("color", "blue")
                    .property("kind", "text")
                    .text("Back")
                    .event(clone!(state => move |_: events::Click| {
                        let route = AdminCurationRoute::Table;
                        state.curation_state.navigate_to(route);
                    }))
                }),
                html!("div", {
                    .property("slot", "buttons")
                    .children(&mut [
                        html!("button-rect", {
                            .property("kind", "text")
                            .property("color", "blue")
                            .text("Cancel")
                            .event(clone!(state => move |_: events::Click| {
                                state.curation_state.navigate_to(AdminCurationRoute::Table);
                            }))
                        }),
                        html!("button-rect", {
                            .property("kind", "filled")
                            .property("color", "blue")
                            .text("Save draft")
                            .event(clone!(state => move |_: events::Click| {
                                state.save_draft();
                            }))
                        }),
                        html!("button-rect", {
                            .property("kind", "outline")
                            .property("color", "blue")
                            .text("Publish")
                            .event(clone!(state => move |_: events::Click| {
                                state.publish();
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .property("slot", "inputs")
                    .children(&mut [
                        html!("input-wrapper", {
                            .property("label", "JIG's name")
                            .children(&mut [
                                html!("input" => HtmlInputElement, {
                                    .with_node!(elem => {
                                        .property_signal("value", state.jig.display_name.signal_cloned())
                                        .event(clone!(state => move |_evt: events::Input| {
                                            let value = elem.value();
                                            state.jig.display_name.set(value);
                                        }))
                                    })
                                }),
                            ])
                        }),
                        html!("input-wrapper", {
                            .property("label", "Author name")
                            .children(&mut [
                                html!("input", {
                                    .property("readOnly", true)
                                    .property("value", &state.jig.author_name)
                                }),
                            ])
                        }),
                        state.render_languages(),
                        state.render_ages(),
                        state.render_affiliations(),
                        html!("input-wrapper", {
                            .property("label", "JIG teacher's description")
                            .children(&mut [
                                html!("textarea" => HtmlTextAreaElement, {
                                    .with_node!(elem => {
                                        .property_signal("value", state.jig.description.signal_cloned())
                                        .event(clone!(state => move |_evt: events::Input| {
                                            let value = elem.value();
                                            state.jig.description.set(value);
                                        }))
                                    })
                                }),
                            ])
                        }),
                        html!("input-wrapper", {
                            .property("label", "Additional keywords")
                            .children(&mut [
                                html!("textarea" => HtmlTextAreaElement, {
                                    .with_node!(elem => {
                                        .property_signal("value", state.jig.other_keywords.signal_cloned())
                                        .event(clone!(state => move |_evt: events::Input| {
                                            let value = elem.value();
                                            state.jig.other_keywords.set(value);
                                        }))
                                    })
                                }),
                            ])
                        }),
                    ])
                }),
            ])
            .child(ModuleThumbnail::render(
                Rc::new(ModuleThumbnail {
                    jig_id: state.jig_id,
                    module: state.jig.modules.lock_ref().get(0).cloned(),
                    is_jig_fallback: true,
                }),
                Some("player")
            ))
            .child(html!("fa-button", {
                .property("slot", "player")
                .property("icon", "fa-duotone fa-circle-play")
                .event(clone!(state => move |_: events::Click| {
                    state.player_open.set(true);
                }))
            }))
            .child_signal(state.player_open.signal().map(clone!(state => move |player_open| {
                match player_open {
                    false => None,
                    true => {
                        let on_close = clone!(state => move|| {
                            state.player_open.set(false);
                        });
                        Some(PlayerPopup::new(
                            state.jig_id,
                            JigPlayerOptions::default(),
                            PreviewPopupCallbacks::new(Box::new(on_close)),
                        ).render(Some("player")))
                    }
                }
            })))
        })
    }
}
