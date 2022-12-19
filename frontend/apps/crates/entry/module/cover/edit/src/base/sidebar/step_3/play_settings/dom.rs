use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};
use std::rc::Rc;

use super::state::PlaySettingsState;
use shared::domain::module::body::cover::Next;

use components::module::_common::edit::settings::prelude::*;

pub fn render(state: Rc<PlaySettingsState>) -> Dom {
    let zipped_audio = clone!(state => move || map_ref! {
        let audio = state.base.audio.signal_cloned(),
        let next = state.base.play_settings.next.signal_cloned()
            => (audio.is_some(), *next)
    });

    let next_signal = clone!(zipped_audio => move || map_ref! {
        let (audio, next) = zipped_audio()
            => {
                if let Next::Auto = next {
                    match audio {
                        true => Next::AfterAudio,
                        false => Next::ClickNext,
                    }
                } else {
                    *next
                }
            }
    });

    let tooltip: Mutable<Option<String>> = Mutable::new(None);

    html!("empty-fragment", {
        .future(zipped_audio().for_each(clone!(state => move |(has_audio, next)| {
            if !has_audio && next == Next::AfterAudio {
                state.set_next(Next::default())
            }
            async {}
        })))
        .child(
            render_settings(Rc::new(ModuleSettings {
                lines: vec![
                    ModuleSettingsLine::new(
                        LineKind::Next,
                        vec![
                            Some(
                                SettingsButtonBuilder::new(
                                    SettingsButtonKind::ContinueClick,
                                    clone!(next_signal => move || next_signal().map(|next| next == Next::ClickNext)),
                                )
                                .on_click(clone!(state => move || state.set_next(Next::ClickNext)))
                                .build()
                            ),
                            Some(
                                SettingsButtonBuilder::new(
                                    SettingsButtonKind::Custom(
                                        SettingsButtonCustomKind::Kind(Box::new(SettingsButtonKind::ContinueAutomatically)),
                                        "after audio",
                                    ),
                                    clone!(next_signal => move || next_signal().map(|next| next == Next::AfterAudio)),
                                )
                                .on_click(clone!(state, tooltip => move || {
                                    if state.base.audio.get_cloned().is_none() {
                                        tooltip.set_neq(Some("This option available only when you have added audio.".to_owned()));
                                    } else {
                                        state.set_next(Next::AfterAudio)
                                    }
                                }))
                                .tooltip(tooltip.clone())
                                .build()
                            ),
                        ],
                    ),
                    // Note - not including scoring until player settings is resolved
                ],
            }))
        )
    })
}
