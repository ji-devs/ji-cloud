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
                        vec![
                            Some(SettingsButtonBuilder::new(
                                    SettingsButtonKind::custom_kind(SettingsButtonKind::Swap, "swap"),
                                    clone!(state => move || {
                                        state.base.extra.settings.swap.signal()
                                    }),
                                )
                                .on_click(clone!(state => move || state.toggle_swap()))
                                .build())
                        ],
                    ),
                    ModuleSettingsLine::new_with_label(
                        "Should student view all pairs?".to_string(),
                        vec![
                            Some(SettingsButtonBuilder::new(
                                    SettingsButtonKind::CardsShowAll,
                                    clone!(state => move || {
                                        state.base.extra.settings.view_all
                                            .signal()
                                    }),
                                )
                                .on_click(clone!(state => move || state.set_view_all(true)))
                                .build()),
                            Some(SettingsButtonBuilder::new(
                                    SettingsButtonKind::CardsShowSome,
                                    clone!(state => move || {
                                        state.base.extra.settings.view_all
                                            .signal()
                                            .map(|view_all| !view_all)
                                    }),
                                )
                                .value(SettingsValue::new_mutable(
                                    state.base.extra.settings.view_pairs.clone(),
                                    clone!(state => move |value| {
                                        state.set_view_pairs(value);
                                    }),
                                ))
                                .on_click(clone!(state => move || state.set_view_all(false)))
                                .build()),
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
    SettingsButtonBuilder::new(
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
    )
    .on_click(clone!(state => move || state.set_display_mode(display_mode)))
    .build()
}
