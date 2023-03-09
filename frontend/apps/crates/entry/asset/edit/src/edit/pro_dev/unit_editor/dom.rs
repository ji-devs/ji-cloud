use std::{process::Child, rc::Rc, str::FromStr};

use components::hebrew_buttons::HebrewButtons;
use dominator::{clone, events, html, with_node, Dom, DomBuilder};
use shared::domain::pro_dev::unit::ProDevUnitValue;
use url::{ParseError, Url};
use utils::{component::Component, init::analytics};
use web_sys::{HtmlElement, HtmlInputElement, HtmlTextAreaElement, ShadowRoot};

use super::state::UnitEditor;

const STR_NAME_LABEL: &str = "Unit’s name";
const STR_NAME_PLACEHOLDER: &str = "Add unit name";
const STR_DESCRIPTION_LABEL: &str = "Description";
const STR_DESCRIPTION_PLACEHOLDER: &str = "Add Description";
const STR_URL_PLACEHOLDER: &str = "Insert URL here";
const STR_ADD_TO_COURSE: &str = "Add to course";

impl Component<UnitEditor> for Rc<UnitEditor> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn apply_on_host(&self, dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        dom.class("main").prop("slot", "main")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        dom.child(html!("unit-edit", {
            .children(&mut [
                html!("input-wrapper", {
                    .prop("slot", "url")
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .attr("dir", "auto")
                            .prop("placeholder", format!("{}", STR_URL_PLACEHOLDER))
                            .event(clone!(state => move |_evt: events::Input| {
                                let val = elem.value().trim().to_string();
                                let url = Url::from_str(&val);
                                match url {
                                    Ok(url) => {
                                        let _ = elem.remove_attribute("error");
                                        state.value.set(Some(ProDevUnitValue::Link(url)));
                                    },
                                    Err(err) => {
                                        match err {
                                            ParseError::RelativeUrlWithoutBase => {
                                                let url_with_https = prepend_https_to_url(&val);
                                                let _ = elem.remove_attribute("error");
                                                elem.set_value(url_with_https.as_str());
                                                state.value.set(Some(ProDevUnitValue::Link(url_with_https)));
                                            },
                                            _ => {
                                                let _ = elem.set_attribute("error", "");
                                                state.value.set(None);
                                            },
                                        }
                                    },
                                }
                            }))
                        })
                    }))
                }),
                html!("input-wrapper", {
                    .prop("slot", "name")
                    .prop("label", format!("{}", STR_NAME_LABEL))
                    // .child({
                    //     HebrewButtons::reveal().render(Some("hebrew-inputs"))
                    // })
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .attr("dir", "auto")
                            .prop("placeholder", format!("{}", STR_NAME_PLACEHOLDER))
                            .prop_signal("value", state.display_name.signal_cloned())
                            .event(clone!(state => move |_evt: events::Input| {
                                let value = elem.value();
                                state.display_name.set(value);
                            }))
                        })
                    }))
                }),
                html!("input-wrapper", {
                    .prop("slot", "description")
                    .prop("label", STR_DESCRIPTION_LABEL)
                    .child(html!("textarea" => HtmlTextAreaElement, {
                        .with_node!(elem => {
                            .attr("dir", "auto")
                            .prop("placeholder", format!(
                                "{}",
                                STR_DESCRIPTION_PLACEHOLDER
                            ))
                            .text_signal(state.description.signal_cloned())
                            .event(clone!(state => move |_: events::Input| {
                                let value = elem.value();
                                state.description.set(value);
                            }))
                        })
                    }))
                }),
                html!("button-rect", {
                    .prop("slot", "add")
                    .prop("size", "small")
                    .prop("bold", true)
                    .text(STR_ADD_TO_COURSE)
                    .prop_signal("loading", state.loader.is_loading())
                    .event(clone!(state => move |_: events::Click| {
                        analytics::event("Add Unit to Course", None);
                        state.create_unit();
                    }))
                }),
            ])
        }))
    }
}

fn prepend_https_to_url(url: &str) -> Url {
    let mut fixed_url_string = String::new();
    fixed_url_string.push_str("https://");
    fixed_url_string.push_str(url);
    Url::from_str(&fixed_url_string).unwrap()
}
