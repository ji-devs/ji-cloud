use shared::domain::jig::module::body::ThemeId;

use futures_signals::signal::{ReadOnlyMutable, Signal, SignalExt};

/*
 * jig_theme_id is a mutable because we need to set it here, but it doesn't get pushed to history
 * theme_signal is a signal because it is read-only
 * the callback to change the theme is because the module is responsible for pushing history
 */

pub struct ThemeSelector {
    pub(super) theme_id: ReadOnlyMutable<ThemeId>,
    pub(super) callbacks: ThemeSelectorCallbacks,
}

impl ThemeSelector {
    pub fn new(theme_id: ReadOnlyMutable<ThemeId>, callbacks: ThemeSelectorCallbacks) -> Self {
        Self {
            theme_id,
            callbacks,
        }
    }

    pub fn selected_signal(&self, theme_id: ThemeId) -> impl Signal<Item = bool> {
        self.theme_id
            .signal()
            .map(move |selected_theme_id| theme_id == selected_theme_id)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectedState {
    Selected,
    Jig,
    None,
}

pub struct ThemeSelectorCallbacks {
    pub on_change: Box<dyn Fn(ThemeId)>,
}

impl ThemeSelectorCallbacks {
    pub fn new(on_change: impl Fn(ThemeId) + 'static) -> Self {
        Self {
            on_change: Box::new(on_change),
        }
    }
}
