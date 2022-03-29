use super::state::*;
use dominator::{clone, html, with_node, Dom, DomBuilder};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

pub fn render_button(state: Rc<SettingsButton>) -> Dom {
    _render_mixin(
        state,
        None::<fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>,
    )
}

pub fn render_button_mixin<F>(state: Rc<SettingsButton>, mixin: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    _render_mixin(state, Some(mixin))
}

fn _render_mixin<F>(state: Rc<SettingsButton>, mixin: Option<F>) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("module-settings-button", {
        .property("kind", state.kind.as_str_id())
        .property_signal("active", (state.active_signal) ())
        .apply_if(state.on_click.is_some(), |dom| {
            dom.event(clone!(state => move |evt:events::Click| {
                // Prevents clicks inside the settings bubble from toggling the state of the
                // setting.
                let should_click = match evt.target() {
                    Some(target) => {
                        let target: JsValue = target.into();
                        let element: HtmlElement = target.into();
                        match element.closest("module-settings-bubble") {
                            Ok(Some(_)) => false,
                            _ => true
                        }
                    },
                    _ => true
                };

                if should_click {
                    (state.on_click.as_ref().unwrap_ji()) ();
                }
            }))
        })
        .apply_if(state.value.is_some(), |dom| {
            let value = state.value.as_ref().unwrap_ji();
            let input_kind = get_input_kind(state.kind);

            dom
                .property_signal("num", value.string_signal())
                .child(html!("module-settings-bubble", {
                    .property("slot", "bubble")
                    .child(html!("module-settings-bubble-content", {
                        .property("kind", state.kind.as_str_id())
                        .property_signal("value", state.value.as_ref().unwrap_ji().string_signal())
                        .apply_if(input_kind.is_some(), |dom| {
                            dom.child(
                                match input_kind.unwrap_ji() {
                                    InputKind::Field => {
                                        render_input_field(state)
                                    },
                                    InputKind::Select(max) => {
                                        render_input_select(state, max)
                                    }
                                }
                            )
                        })
                    }))
                }))
        })
        .apply_if(mixin.is_some(), |dom| {
            dom.apply(mixin.unwrap_ji())
        })
    })
}

pub fn render_input_field(state: Rc<SettingsButton>) -> Dom {
    html!("input" => web_sys::HtmlInputElement, {
        .property_signal("value", state.value.as_ref().unwrap_ji().string_signal())
        .with_node!(elem => {
            .event(clone!(state => move |_evt:events::Change| {
                let value = state.value.as_ref().unwrap_ji();
                value.handle_event(&elem.value());
            }))
        })
    })
}

pub fn render_input_select(state: Rc<SettingsButton>, max: usize) -> Dom {
    html!("select" => web_sys::HtmlSelectElement, {
        .property_signal("value", state.value.as_ref().unwrap_ji().string_signal())
        .children(
            (1..max)
                .map(|index| {
                    let value_str = state.value.as_ref().unwrap_ji().get_select_value(index);

                    html!("option", {
                        .text(&value_str)
                        .property("value", &value_str)
                    })
                })
        )
        .with_node!(elem => {
            .event(clone!(state => move |_evt:events::Change| {
                let value = state.value.as_ref().unwrap_ji();
                value.handle_event(&elem.value());
            }))
        })
    })
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum InputKind {
    Field,
    Select(usize),
}
fn get_input_kind(kind: SettingsButtonKind) -> Option<InputKind> {
    match kind {
        SettingsButtonKind::Attempts => Some(InputKind::Select(6)),
        SettingsButtonKind::NumChoices => Some(InputKind::Select(6)),
        SettingsButtonKind::NumPairs => Some(InputKind::Field),

        SettingsButtonKind::TimeLimit => Some(InputKind::Field),
        SettingsButtonKind::ContinueSome => Some(InputKind::Field),
        SettingsButtonKind::Rounds => Some(InputKind::Field),
        _ => None,
    }
}
