use super::state::SchoolEnd;
use components::file_input::{FileInput, FileInputConfig};
use dominator::{clone, html, with_node, DomBuilder};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::{
    component::Component,
    events, gap,
    routes::{Route, UserRoute},
};
use web_sys::{HtmlInputElement, HtmlTextAreaElement, ShadowRoot};

const STR_ORGANIZATION_TYPES: [&str; 7] = [
    "Jewish day school",
    "Supplementary school",
    "Synagogue",
    "Organization",
    "Community center",
    "Youth group",
    "Other",
];

impl Component<SchoolEnd> for Rc<SchoolEnd> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        dom.child(html!("auth-page", {
            .prop("img", "entry/user/side/main.webp")
            .child(html!("main", {
                .child(html!("h1", {
                    .text("Jigzi School page setup")
                }))
                .child(gap!(8))
                .child(html!("h4", {
                    .text("This info will be public on your page in the Jigzi Community")
                }))
                .child(gap!(24))
                .child(html!("div", {
                    .class("inputs")
                    .child(html!("input-select", {
                        .prop("label", "Organization Type")
                        .prop("placeholder", "Choose one")
                        .prop_signal("value", state.organization_type.signal_cloned().map(|x| x.unwrap_or_default()))
                        .children(STR_ORGANIZATION_TYPES.iter().map(clone!(state => move |org_type| {
                            html!("input-select-option", {
                                .text(org_type)
                                .event(clone!(state => move |evt:events::CustomSelectedChange| {
                                    if evt.selected() {
                                        state.organization_type.set(Some(org_type.to_string()));
                                    }
                                }))
                            })
                        })))
                    }))
                    .child(html!("input-wrapper", {
                        .prop("label", "Website")
                        .child(html!("input" => HtmlInputElement, {
                            .with_node!(elem => {
                                .prop("type", "url")
                                .prop("placeholder", "YourSchool.org")
                                .prop_signal("value", state.website.signal_cloned().map(|x| x.unwrap_or_default()))
                                .event(clone!(state => move |_evt: events::Input| {
                                    let value = string_to_option(elem.value());
                                    state.website.set(value);
                                }))
                            })
                        }))
                    }))
                    .child(html!("input-wrapper", {
                        .prop("label", "Description")
                        .child(html!("textarea" => HtmlTextAreaElement, {
                            .with_node!(elem => {
                                .prop("placeholder", "Tell us about your organization")
                                .prop_signal("value", state.description.signal_cloned().map(|x| x.unwrap_or_default()))
                                .event(clone!(state => move |_evt: events::Input| {
                                    let value = string_to_option(elem.value());
                                    state.description.set(value);
                                }))
                            })
                        }))
                    }))
                    .child(html!("div", {
                        .class("image")
                        .child(FileInput::new(FileInputConfig {
                            on_change: Box::new(clone!(state => move |file| {
                                if let Some(file) = file {
                                    state.profile_image.set_neq(Some(file));
                                }
                            })),
                            preview_images: true,
                            accept: "image/*",
                            ..Default::default()
                        }).render())
                    }))
                }))
                .child(gap!(48))
                .child(html!("div", {
                    .class("actions")
                    .child(html!("button-rect", {
                        .text("Skip this step")
                        .prop("color", "blue")
                        .prop("kind", "text")
                        .prop_signal("disabled", state.loader.is_loading())
                        .event(move |_: events::Click| {
                            dominator::routing::go_to_url(&Route::User(UserRoute::Welcome(Default::default())).to_string());
                        })
                    }))
                    .child(html!("button-rect", {
                        .text("Done")
                        .prop("color", "red")
                        .prop("kind", "filled")
                        .prop_signal("disabled", state.loader.is_loading())
                        .event(clone!(state => move |_: events::Click| {
                            state.save();
                        }))
                    }))
                }))
            }))
        }))
    }
}

// convert empty strings into None
fn string_to_option(s: String) -> Option<String> {
    if s.trim().is_empty() {
        None
    } else {
        Some(s)
    }
}
