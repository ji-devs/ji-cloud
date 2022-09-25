use std::{rc::Rc, str::FromStr};

use dominator::{clone, html, with_node, Dom};
use futures_signals::{
    map_ref,
    signal::{not, Signal},
};
use url::{ParseError, Url};
use utils::events;
use web_sys::HtmlTextAreaElement;

use crate::edit::publish::add_additional_resource::ActivePopup;

use super::state::AddLink;

const STR_SAVE: &str = "Save";
const STR_CANCEL: &str = "Cancel";
const STR_BACK: &str = "Back";

impl AddLink {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = Rc::clone(self);
        html!("jig-edit-publish-resource-add-link", {
            .children(&mut [
                html!("button-rect", {
                    .property("slot", "back")
                    .property("color", "blue")
                    .property("kind", "text")
                    .child(html!("fa-icon", {
                        .property("icon", "fa-solid fa-angle-left")
                    }))
                    .text(STR_BACK)
                    .event(clone!(state => move|_: events::Click| {
                        state.add_resources_state.active_popup.set(Some(ActivePopup::Main));
                    }))
                }),
                html!("fa-button", {
                    .property("icon", "fa-light fa-xmark")
                    .property("slot", "close")
                    .event(clone!(state => move|_: events::Click| {
                        state.add_resources_state.active_popup.set(None);
                    }))
                }),
                html!("textarea" => HtmlTextAreaElement, {
                    .with_node!(elem => {
                        .property("slot", "textarea")
                        .property("spellcheck", "false")
                        .event(clone!(state, elem => move |_: events::Change| {
                            let val = elem.value().trim().to_string();
                            let url = Url::from_str(&val);

                            match url {
                                Ok(url) => {
                                    let _ = elem.remove_attribute("error");
                                    state.url.set(Some(url));
                                },
                                Err(err) => {
                                    match err {
                                        ParseError::RelativeUrlWithoutBase => {
                                            let url_with_https = prepend_https_to_url(&val);
                                            let _ = elem.remove_attribute("error");
                                            elem.set_value(url_with_https.as_str());
                                            state.url.set(Some(url_with_https));
                                        },
                                        _ => {
                                            let _ = elem.set_attribute("error", "");
                                            state.url.set(None);
                                        },
                                    }
                                },
                            }
                        }))
                    })
                }),
                html!("button-rect", {
                    .property("slot", "actions")
                    .property("color", "blue")
                    .property("kind", "text")
                    .text(STR_CANCEL)
                    .event(clone!(state => move|_: events::Click| {
                        state.add_resources_state.active_popup.set(None);
                    }))
                }),
                html!("button-rect", {
                    .property("slot", "actions")
                    .property("color", "blue")
                    .property("kind", "filled")
                    .text(STR_SAVE)
                    .property_signal("disabled", not(state.form_filled_out()))
                    .event(clone!(state => move|_: events::Click| {
                        state.save();
                    }))
                }),
            ])
            .child_signal(state.add_resources_state.publish_state.resource_types.signal_ref(clone!(state => move|resource_types| {
                Some(html!("input-select", {
                    .property("slot", "type")
                    .property("label", "Select type")
                    .property("placeholder", "Select one")
                    .property_signal("value", state.resource_type.signal_ref(|resource_type| {
                        match resource_type {
                            Some(resource_type) => resource_type.display_name.clone(),
                            None => String::new(),
                        }
                    }))
                    .children(resource_types.iter().map(clone!(state => move |resource_type| {
                        html!("input-select-option", {
                            .text(&resource_type.display_name)
                            .event(clone!(state, resource_type => move |evt:events::CustomSelectedChange| {
                                if evt.selected() {
                                    state.resource_type.set(Some(resource_type.clone()));
                                }
                            }))
                        })
                    })))
                }))
            })))
        })
    }

    fn form_filled_out(self: &Rc<Self>) -> impl Signal<Item = bool> {
        map_ref! {
            let url = self.url.signal_cloned(),
            let resource_type = self.resource_type.signal_cloned()
                => move {
                    url.is_some() && resource_type.is_some()
                }
        }
    }
}

fn prepend_https_to_url(url: &str) -> Url {
    let mut fixed_url_string = String::new();
    fixed_url_string.push_str("https://");
    fixed_url_string.push_str(url);
    Url::from_str(&fixed_url_string).unwrap()
}
