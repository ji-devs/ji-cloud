use std::rc::Rc;

use components::hebrew_buttons::HebrewButtons;
use dominator::{clone, events, html, with_node, DomBuilder};
use futures_signals::{map_ref, signal::SignalExt};
use utils::{component::Component, init::analytics};
use web_sys::{HtmlElement, HtmlInputElement, HtmlTextAreaElement, ShadowRoot};

use crate::edit::pro_dev::unit_editor::UnitValue;

use super::{add_unit_value::AddUnitValue, state::UnitEditor};

const STR_NAME_LABEL: &str = "Unit’s name";
const STR_NAME_PLACEHOLDER: &str = "Add unit name";
const STR_DESCRIPTION_LABEL: &str = "Description";
const STR_DESCRIPTION_PLACEHOLDER: &str = "Add Description";
const STR_URL_PLACEHOLDER: &str = "Insert URL here";
const STR_ADD_TO_COURSE: &str = "Add to course";
const STR_ADD_LINK: &str = " Add Link";
const STR_UPLOAD_FILE: &str = " Upload file";
const STR_ADD_YOUTUBE: &str = " Video";

impl Component<UnitEditor> for Rc<UnitEditor> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn apply_on_host(&self, dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        dom.class("main").prop("slot", "main")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        state.load_unit();

        let is_valid = map_ref! {
            let display_name = state.display_name.signal_cloned(),
            let description = state.description.signal_cloned(),
            let value = state.value.signal_cloned()

            => {
                !display_name.trim().is_empty() && !description.trim().is_empty() && UnitValue::is_some(value)
            }
        };

        dom.child(html!("unit-edit", {
            .children(&mut [
                html!("label", {
                    .prop("slot", "link-select")
                    .child(html!("input", {
                        .prop("slot", "input")
                        .prop("label", STR_URL_PLACEHOLDER)
                        .prop("type", "radio")
                        .prop("name", "type")
                        .prop("value", "input-link")
                        .prop_signal("checked", state.value.signal_ref(|value| {
                            matches!(value, UnitValue::Link(_))
                        }))
                        .event(clone!(state => move |_: events::Click| {
                            state.value.set(UnitValue::Link(None))
                        }))
                    }))
                    .text(STR_ADD_LINK)
                }),
                html!("label", {
                    .prop("slot", "file-select")
                    .child(html!("input", {
                        .prop("type", "radio")
                        .prop("name", "type")
                        .prop("value", "input-file")
                        .prop_signal("checked", state.value.signal_ref(|value| {
                            matches!(value, UnitValue::File(_))
                        }))
                        .event(clone!(state => move |_: events::Click| {
                            state.value.set(UnitValue::File(None));
                        }))
                    }))
                    .text(STR_UPLOAD_FILE)
                }),
                html!("label", {
                    .prop("slot", "youtube-select")
                    .child(html!("input", {
                        .prop("type", "radio")
                        .prop("name", "type")
                        .prop("value", "input-youtube")
                        .prop("label", "Add a Youtube link")
                        .prop_signal("checked", state.value.signal_ref(|value| {
                            matches!(value, UnitValue::Video(_))
                        }))
                        .event(clone!(state => move |_: events::Click| {
                            state.value.set(UnitValue::Video(None));
                        }))
                    }))
                    .text(STR_ADD_YOUTUBE)
                }),
            ])
            .child({
                AddUnitValue::new(state.clone()).render(Some("body-input"))
            })
            .children(&mut [
                html!("input-wrapper", {
                    .prop("slot", "name")
                    .prop("label", format!("{}", STR_NAME_LABEL))
                    .child({
                        HebrewButtons::reveal().render(Some("hebrew-inputs"))
                    })
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
                    .child({
                        HebrewButtons::reveal().render(Some("hebrew-inputs"))
                    })
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
                    .prop_signal("disabled", is_valid.map(|value| !value))
                    .prop("slot", "add")
                    .prop("size", "small")
                    .prop("bold", true)
                    .text(STR_ADD_TO_COURSE)
                    .event(clone!(state => move |_: events::Click| {
                        analytics::event("Add Unit to Course", None);
                        match state.unit_id {
                            Some(_) => {
                                state.clone().update_unit()
                            }
                            None => {
                                state.clone().create_unit()
                            }
                        }
                    }))
                }),
            ])
        }))
    }
}
