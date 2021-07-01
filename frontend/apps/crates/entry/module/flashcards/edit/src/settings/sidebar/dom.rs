use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::{
    state::*,
    settings::state::*
};
use super::state::*;
use utils::prelude::*;
use shared::domain::jig::module::body::flashcards::DisplayMode;
use components::module::_common::edit::settings::prelude::*;

pub fn render(state: Rc<SidebarSettings>) -> Dom {
    render_settings(Rc::new(ModuleSettings {
        lines: vec![
            (LineKind::CardView, vec![
                make_button(state.clone(), DisplayMode::Single),
                make_button(state.clone(), DisplayMode::Double),
            ])
        ]
    }))
}

pub fn make_button(state: Rc<SidebarSettings>, display_mode:DisplayMode) -> Rc<SettingsButton> {

    SettingsButton::new_click(
        if display_mode == DisplayMode::Single { SettingsButtonKind::CardSingle } else { SettingsButtonKind::CardDouble }, 
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
