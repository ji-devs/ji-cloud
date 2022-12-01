use dominator::{clone, Dom};
use std::rc::Rc;

use super::state::*;

use components::module::_common::edit::settings::prelude::*;
use shared::domain::module::body::flashcards::DisplayMode;

pub fn render(state: Rc<SidebarSettings>) -> Dom {
    render_settings(Rc::new(ModuleSettings {
        lines: vec![
            ModuleSettingsLine::new_with_label(
                "How should card pairs be displayed?".to_string(),
                vec![
                    Some(make_button(state.clone(), DisplayMode::Double)),
                    Some(make_button(state.clone(), DisplayMode::Single)),
                ],
            ),
            ModuleSettingsLine::new_with_label(
                "Which card should be face-up?".to_string(),
                vec![Some(SettingsButton::new_click(
                    SettingsButtonKind::Swap,
                    clone!(state => move || {
                        state.base.extra.settings.swap.signal()
                    }),
                    clone!(state => move || {
                        state.toggle_swap();
                    }),
                ))],
            ),
        ],
    }))
}

pub fn make_button(state: Rc<SidebarSettings>, display_mode: DisplayMode) -> Rc<SettingsButton> {
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
