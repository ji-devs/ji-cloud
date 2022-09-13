use super::state::*;
use dominator::{clone, html, with_node, Dom, DomBuilder};
use futures_signals::signal::SignalExt;
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
        .property_signal("bubbleOpen", state.bubble_open.signal())
        .event(clone!(state => move |_evt: events::Close| {
            state.bubble_open.set_neq(false);
        }))
        .apply(clone!(state => move |dom| {
            if let SettingsButtonKind::Custom(_, label) = state.kind {
                dom.property("label", label)
            } else {
                dom
            }
        }))
        .event(clone!(state => move |evt: events::Click| {
            // Prevents clicks inside the settings bubble from toggling the state of the
            // setting.
            let can_click = match evt.target() {
                Some(target) => {
                    let target: JsValue = target.into();
                    let element: HtmlElement = target.into();

                    !matches!(
                        element.closest("module-settings-bubble"),
                        Ok(Some(_))
                    )
                },
                _ => true
            };

            if can_click {
                if let Some(on_click) = &state.on_click {
                    on_click();
                }

                if state.value.is_some() {
                    let is_open = state.bubble_open.get();
                    state.bubble_open.set_neq(!is_open);
                }
            }
        }))
        .apply_if(state.value.is_some(), |dom| {
            let value = state.value.as_ref().unwrap_ji();
            let input_kind = get_input_kind(state.kind);

            dom
                .property_signal("num", value.string_signal())
                .child_signal(state.bubble_open.signal_cloned().map(clone!(state => move |bubble_open| {
                    if bubble_open {
                        Some(html!("module-settings-bubble", {
                            .property("slot", "bubble")
                            .event(clone!(state => move |_evt: events::Close| {
                                state.bubble_open.set_neq(false);
                            }))
                            .child(html!("module-settings-bubble-content", {
                                .property("kind", state.kind.as_str_id())
                                .property_signal("value", state.value.as_ref().unwrap_ji().string_signal())
                                .apply_if(input_kind.is_some(), clone!(state => move |dom| {
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
                                }))
                            }))
                        }))
                    } else {
                        None
                    }
                })))
        })
        .apply_if(mixin.is_some(), |dom| {
            dom.apply(mixin.unwrap_ji())
        })
    })
}

pub fn render_input_field(state: Rc<SettingsButton>) -> Dom {
    html!("input" => web_sys::HtmlInputElement, {
        .property_signal("value", state.value.as_ref().unwrap_ji().string_signal())
        .after_inserted(|elem| {
            wasm_bindgen_futures::spawn_local(clone!(elem => async move {
                gloo_timers::future::TimeoutFuture::new(0).await;
                // Automatically focus so that blur works
                let _ = elem.focus();
            }));
        })
        .after_removed(clone!(state => move |elem| {
            // Whenever this element is removed, handle the new value
            let value = state.value.as_ref().unwrap_ji();
            value.handle_event(&elem.value());
        }))
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
