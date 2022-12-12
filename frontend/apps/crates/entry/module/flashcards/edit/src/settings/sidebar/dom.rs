use dominator::{clone, html, Dom};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use std::rc::Rc;

use super::state::*;

use components::module::_common::edit::settings::prelude::*;
use shared::domain::module::body::flashcards::DisplayMode;

pub fn render(state: Rc<SidebarSettings>) -> Dom {
    html!("empty-fragment", {
        .future(state.base.pairs.signal_vec_cloned().to_signal_cloned().for_each(clone!(state => move |pairs| {
            let view_pairs = state.base.extra.settings.view_pairs.get_cloned();

            if view_pairs > pairs.len() as u32 {
                state.set_view_pairs(pairs.len() as u32);
            }
            async {}
        })))
        .child(
            render_settings(Rc::new(ModuleSettings {
                lines: vec![
                    ModuleSettingsLine::new_with_label(
                        "How should card pairs be displayed?".to_string(),
                        vec![
                            Some(make_display_mode_button(state.clone(), DisplayMode::Double)),
                            Some(make_display_mode_button(state.clone(), DisplayMode::Single)),
                        ],
                    ),
                    ModuleSettingsLine::new_with_label(
                        "Which card should be face-up?".to_string(),
                        vec![Some(SettingsButton::new_click(
                            SettingsButtonKind::custom_kind(SettingsButtonKind::Swap, "swap"),
                            clone!(state => move || {
                                state.base.extra.settings.swap.signal()
                            }),
                            clone!(state => move || {
                                state.toggle_swap();
                            }),
                        ))],
                    ),
                    ModuleSettingsLine::new_with_label(
                        "Should student view all pairs?".to_string(),
                        vec![
                            Some(SettingsButton::new_click(
                                SettingsButtonKind::CardsShowAll,
                                clone!(state => move || {
                                    state.base.extra.settings.view_all
                                        .signal()
                                }),
                                clone!(state => move || {
                                    state.set_view_all(true);
                                }),
                            )),
                            Some(SettingsButton::new_value_click(
                                SettingsButtonKind::CardsShowSome,
                                clone!(state => move || {
                                    state.base.extra.settings.view_all
                                        .signal()
                                        .map(|view_all| !view_all)
                                }),
                                SettingsValue::new_mutable(
                                    state.base.extra.settings.view_pairs.clone(),
                                    clone!(state => move |value| {
                                        state.set_view_pairs(value);
                                    }),
                                ),
                                clone!(state => move || {
                                    state.set_view_all(false);
                                }),
                            )),
                        ],
                    ),
                ],
            }))
        )
    })
}

pub fn make_display_mode_button(
    state: Rc<SidebarSettings>,
    display_mode: DisplayMode,
) -> Rc<SettingsButton> {
    SettingsButton::new_click(
        if display_mode == DisplayMode::Single {
            SettingsButtonKind::CardSingle
        } else {
            SettingsButtonKind::CardDouble
        },
        clone!(state => move || {
            state.base.extra.settings.display_mode.signal_ref(move |curr| {
                *curr == display_mode
            })
        }),
        clone!(state => move || {
            state.set_display_mode(display_mode);
        }),
    )
}
