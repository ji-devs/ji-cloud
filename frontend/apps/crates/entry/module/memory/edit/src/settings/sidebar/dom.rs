use dominator::{clone, html, Dom};
use shared::config::MAX_LIST_WORDS;
use std::rc::Rc;

use super::state::*;
use components::module::_common::edit::settings::prelude::*;
use futures_signals::{
    signal::{Mutable, SignalExt},
    signal_vec::SignalVecExt,
};

pub fn render(state: Rc<SidebarSettings>) -> Dom {
    if state.base.extra.settings.show_all_pairs.get() {
        state.set_default_pairs();
    }

    let tooltip = Mutable::new(None);

    html!("empty-fragment", {
        .child_signal(state.base.pairs.signal_vec_cloned().len().map(clone!(state, tooltip => move |len| {
            let show_all_kind = if len > MAX_LIST_WORDS {
                SettingsButtonKind::custom_kind(SettingsButtonKind::CardsShowAll, "show max")
            } else {
                SettingsButtonKind::CardsShowAll
            };
            Some(render_settings(Rc::new(ModuleSettings {
                lines: vec![
                    ModuleSettingsLine::new_with_label(
                        "Should student view all pairs?".into(),
                        vec![Some(
                            SettingsButtonBuilder::new(
                                show_all_kind,
                                clone!(state => move || state.base.extra.settings.show_all_pairs.signal()),
                            )
                            .on_click(clone!(state => move || state.toggle_use_default_pairs()))
                            .build()),
                            Some(SettingsButtonBuilder::new(
                                SettingsButtonKind::CardsShowSome,
                                clone!(state => move || {
                                    state.base.extra.settings.show_all_pairs.signal().map(|v| !v)
                                }),
                            )
                            .value(SettingsValue::new_mutable(
                                state.settings().pairs_to_display.clone(),
                                clone!(state, tooltip => move |value| {
                                    if state.set_pairs_to_display(value) {
                                        tooltip.set(Some(format!("We can only show a maximum of {MAX_LIST_WORDS} pairs in a memory game.")))
                                    }
                                }),
                            ))
                            .on_click(clone!(state => move || {
                                if state.settings().show_all_pairs.get() {
                                    state.settings().show_all_pairs.set(false);
                                    state.set_default_pairs();
                                }
                            }))
                            .tooltip(tooltip.clone())
                            .build(),
                        )],
                    ),
                    ModuleSettingsLine::new(
                        LineKind::TimeLimit,
                        vec![
                            Some(
                                SettingsButtonBuilder::new(
                                    SettingsButtonKind::TimeLimitOff,
                                    clone!(state => move || {
                                        state.base.extra.settings.has_time_limit
                                            .signal()
                                            .map(|flag| !flag)
                                    }),
                                )
                                .on_click(clone!(state => move || state.set_has_time_limit(false)))
                                .build(),
                            ),
                            Some(
                                SettingsButtonBuilder::new(
                                    SettingsButtonKind::TimeLimit,
                                    clone!(state => move || {
                                        state.base.extra.settings.has_time_limit
                                            .signal()
                                    }),
                                )
                                .value(SettingsValue::new(
                                    state.settings().time_limit.get(),
                                    clone!(state => move |value| {
                                        state.set_time_limit(value);
                                    }),
                                ))
                                .on_click(clone!(state => move || state.set_has_time_limit(true)))
                                .build(),
                            ),
                        ],
                    ),
                    // NOTE - not including score until player/jig story is resolved
                ],
            })))
        })))
    })
}
