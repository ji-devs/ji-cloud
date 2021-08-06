use dominator_helpers::futures::AsyncLoader;
use shared::domain::jig::{
    module::body::{ThemeChoice, ThemeId},
    JigId,
};

use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable, Signal},
};

/*
 * jig_theme_id is a mutable because we need to set it here, but it doesn't get pushed to history
 * theme_signal is a signal because it is read-only
 * the callback to change the theme is because the module is responsible for pushing history
 */

pub struct ThemeSelector {
    pub(super) jig_id: JigId,
    pub(super) jig_theme_id: Mutable<ThemeId>,
    pub(super) theme_id: ReadOnlyMutable<ThemeId>,
    pub(super) callbacks: ThemeSelectorCallbacks,
    pub(super) jig_id_saver: AsyncLoader,
}

impl ThemeSelector {
    pub fn new(
        jig_id: JigId,
        jig_theme_id: Mutable<ThemeId>,
        theme_id: ReadOnlyMutable<ThemeId>,
        callbacks: ThemeSelectorCallbacks,
    ) -> Self {
        Self {
            jig_id,
            jig_theme_id,
            theme_id,
            callbacks,
            jig_id_saver: AsyncLoader::new(),
        }
    }

    pub fn selected_state_signal(&self, theme_id: ThemeId) -> impl Signal<Item = SelectedState> {
        map_ref! {
            let jig_theme_id = self.jig_theme_id.signal(),
            let selected_theme_id = self.theme_id.signal()
                => move {
                //The current brief is that there is no UI difference
                //between selected and de-selected jig id
                if theme_id == *jig_theme_id {
                    SelectedState::Jig
                } else if theme_id == *selected_theme_id {
                    SelectedState::Selected
                } else {
                    SelectedState::None
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectedState {
    Selected,
    Jig,
    None,
}

pub struct ThemeSelectorCallbacks {
    pub on_change: Box<dyn Fn(ThemeChoice)>,
}

impl ThemeSelectorCallbacks {
    pub fn new(on_change: impl Fn(ThemeChoice) + 'static) -> Self {
        Self {
            on_change: Box::new(on_change),
        }
    }
}
