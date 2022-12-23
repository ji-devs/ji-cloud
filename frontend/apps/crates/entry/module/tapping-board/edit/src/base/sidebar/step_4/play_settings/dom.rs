use dominator::{clone, Dom};
use std::rc::Rc;

use super::state::State;
use shared::domain::module::body::tapping_board::{Hint, Next};

use components::module::_common::edit::settings::prelude::*;
pub fn render(state: Rc<State>) -> Dom {
    render_settings(Rc::new(ModuleSettings {
        lines: vec![
            ModuleSettingsLine::new(
                LineKind::Hint,
                vec![
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::HighlightOff,
                            clone!(state => move || {
                                state.base.play_settings.hint.signal_ref(|curr| {
                                    *curr == Hint::None
                                })
                            }),
                        )
                        .on_click(clone!(state => move || state.set_hint(Hint::None)))
                        .build()
                    ),
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::Highlight,
                            clone!(state => move || {
                                state.base.play_settings.hint.signal_ref(|curr| {
                                    *curr == Hint::Highlight
                                })
                            }),
                        )
                        .on_click(clone!(state => move || state.set_hint(Hint::Highlight)))
                        .build()
                    ),
                ],
            ),
            ModuleSettingsLine::new(
                LineKind::Next,
                vec![
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::ContinueClick,
                            clone!(state => move || {
                                state.base.play_settings.next.signal_ref(|curr| {
                                    std::mem::discriminant(curr) == std::mem::discriminant(&Next::Continue)
                                })
                            }),
                        )
                        .on_click(clone!(state => move || state.set_next(Next::Continue)))
                        .build()
                    ),
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::ContinueAll,
                            clone!(state => move || {
                                state.base.play_settings.next.signal_ref(|curr| {
                                    std::mem::discriminant(curr) == std::mem::discriminant(&Next::SelectAll)
                                })
                            }),
                        )
                        .on_click(clone!(state => move || state.set_next(Next::SelectAll)))
                        .build()
                    ),
                ],
            ),
        ],
    }))
}
