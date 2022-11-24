use dominator::{clone, Dom};
use std::rc::Rc;

use super::state::State;
use shared::domain::module::body::tapping_board::{Hint, Next};

use components::module::_common::edit::settings::prelude::*;
pub fn render(state: Rc<State>) -> Dom {
    render_settings(Rc::new(ModuleSettings {
        lines: vec![
            (
                LineKind::Hint,
                vec![
                    Some(SettingsButton::new_click(
                        SettingsButtonKind::HighlightOff,
                        clone!(state => move || {
                            state.base.play_settings.hint.signal_ref(|curr| {
                                *curr == Hint::None
                            })
                        }),
                        clone!(state => move || {
                            state.set_hint(Hint::None);
                        }),
                    )),
                    Some(SettingsButton::new_click(
                        SettingsButtonKind::Highlight,
                        clone!(state => move || {
                            state.base.play_settings.hint.signal_ref(|curr| {
                                *curr == Hint::Highlight
                            })
                        }),
                        clone!(state => move || {
                            state.set_hint(Hint::Highlight);
                        }),
                    )),
                ],
            ),
            (
                LineKind::Next,
                vec![
                    Some(SettingsButton::new_click(
                        SettingsButtonKind::ContinueClick,
                        clone!(state => move || {
                            state.base.play_settings.next.signal_ref(|curr| {
                                std::mem::discriminant(curr) == std::mem::discriminant(&Next::Continue)
                            })
                        }),
                        clone!(state => move || {
                            state.set_next(Next::Continue);
                        }),
                    )),
                    Some(SettingsButton::new_click(
                        SettingsButtonKind::ContinueAll,
                        clone!(state => move || {
                            state.base.play_settings.next.signal_ref(|curr| {
                                std::mem::discriminant(curr) == std::mem::discriminant(&Next::SelectAll)
                            })
                        }),
                        clone!(state => move || {
                            state.set_next(Next::SelectAll);
                        }),
                    )),
                    Some(SettingsButton::new_value_click(
                        SettingsButtonKind::ContinueSome,
                        clone!(state => move || {
                            state.base.play_settings.next.signal_ref(|curr| {
                                std::mem::discriminant(curr) == std::mem::discriminant(&Next::SelectSome(0))
                            })
                        }),
                        SettingsValue::new(
                            state.base.play_settings.next_value.get(),
                            clone!(state => move |value| {
                                state.set_next_value(value);
                            }),
                        ),
                        clone!(state => move || {
                            state.set_next_some();
                        }),
                    )),
                ],
            ),
        ],
    }))
}
/*
pub fn render(state: Rc<State>) -> Dom {
    html!("section", {
        .style("display", "flex")
        .style("flex-direction", "column")
        .children(&mut [
            html!("div", {
                .text(crate::strings::step_4::STR_HEADER_HINT)
            }),
            render_hint_choice(state.clone(), Hint::Highlight),
            render_hint_choice(state.clone(), Hint::None),
            html!("div", {
                .text(crate::strings::step_4::STR_HEADER_NEXT)
            }),
            render_next_choice_simple(state.clone(), Next::Continue),
            render_next_choice_simple(state.clone(), Next::SelectAll),
            render_next_select_some(state.clone())
        ])
    })
}

fn render_hint_choice(state: Rc<State>, hint:Hint) -> Dom {
    html!("label", {
        .child(html!("input", {
            .attr("type", "radio")
            .attr("name", "hint")
            .event(clone!(state, hint => move |evt:events::Change| {
                if evt.checked().unwrap_or_default() {
                    state.set_hint(hint.clone());
                }
            }))
            .prop_signal("checked", state.base.play_settings.hint.signal_ref(clone!(hint => move |curr| *curr == hint)))
        }))
        .text(match hint {
            Hint::None => crate::strings::step_4::STR_HINT_NONE,
            Hint::Highlight => crate::strings::step_4::STR_HINT_HIGHLIGHT
        })
    })
}


fn render_next_choice_simple(state: Rc<State>, next:Next) -> Dom {
    html!("label", {
        .child(html!("input", {
            .attr("type", "radio")
            .attr("name", "next")
            .event(clone!(state, next => move |evt:events::Change| {
                if evt.checked().unwrap_or_default() {
                    state.set_next(next.clone());
                }
            }))
            .prop_signal("checked", state.base.play_settings.next.signal_ref(clone!(next => move |curr| {
                std::mem::discriminant(curr) == std::mem::discriminant(&next)
            })))
        }))
        .text(match next {
            Next::Continue => crate::strings::step_4::STR_NEXT_CONTINUE,
            Next::SelectAll => crate::strings::step_4::STR_NEXT_SELECT_ALL,
            _ => ""
        })
    })
}
fn render_next_select_some(state: Rc<State>) -> Dom {
    html!("label", {
        .child(html!("input", {
            .attr("type", "radio")
            .attr("name", "next")
            .event(clone!(state => move |evt:events::Change| {
                if evt.checked().unwrap_or_default() {
                    state.set_next_amount();
                }
            }))
            .prop_signal("checked", state.base.play_settings.next.signal_ref(|curr| {
                std::mem::discriminant(curr) == std::mem::discriminant(&Next::SelectSome(0))
            }))
        }))
        .children(&mut [
            html!("span", {
                .text(crate::strings::step_4::STR_NEXT_SELECT_SOME_PREFIX)
            }),
            html!("input" => web_sys::HtmlInputElement, {
                .prop("type", "text")
                .prop("size", "3")
                .prop("value", state.some_amount.borrow().to_string())
                .with_node!(elem => {
                    .event(clone!(state => move |evt:events::Input| {
                        let value = evt.value().and_then(|value| value.parse::<usize>().ok());

                        if let Some(value) = value {
                            state.on_next_amount_value(value);
                        }

                    }))
                })
            }),
            html!("span", {
                .text(crate::strings::step_4::STR_NEXT_SELECT_SOME_SUFFIX)
            }),
        ])
    })
}
//<input @change=${this.onRadioChange} type="radio" name="img_kind" value="sticker" .checked=${imageKind === "sticker"} />
//
*/
