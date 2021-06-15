use dominator_helpers::futures::AsyncLoader;
use shared::domain::jig::{JigId, module::body::{ThemeId, ThemeChoice}};
use std::rc::Rc;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt}
};
use std::pin::Pin;
use dominator::clone;

/*
 * jig_theme_id is a mutable because we need to set it here, but it doesn't get pushed to history
 * theme_signal is a signal because it is read-only
 * the callback to change the theme is because the module is responsible for pushing history
 */

pub struct ThemeSelector {
    pub(super) jig_id: JigId,
    pub(super) jig_theme_id: Mutable<ThemeId>,
    pub(super) _theme_signal: Box<dyn Fn() -> Pin<Box<dyn Signal<Item = ThemeChoice>>>>,
    pub(super) callbacks: ThemeSelectorCallbacks,
    pub(super) jig_id_saver: AsyncLoader
}

impl ThemeSelector {
    pub fn new<ThemeSigFn, ThemeSig>(
        jig_id: JigId, 
        jig_theme_id: Mutable<ThemeId>, 
        theme_signal: ThemeSigFn, 
        callbacks: ThemeSelectorCallbacks
    ) -> Self
        where
            ThemeSigFn: Fn() -> ThemeSig + 'static,
            ThemeSig: Signal<Item = ThemeChoice> + 'static
    {
        Self {
            jig_id,
            jig_theme_id,
            _theme_signal: Box::new(move || Box::pin(theme_signal())),
            callbacks,
            jig_id_saver: AsyncLoader::new()
        }
    }

    pub fn theme_signal(&self) -> impl Signal<Item = ThemeChoice> {
        (self._theme_signal) ()
    }

    pub fn theme_id_signal(&self) -> impl Signal<Item = ThemeId> {
        map_ref! {
            let jig_theme_id = self.jig_theme_id.signal(),
            let theme = self.theme_signal()
            => {
                match *theme {
                    ThemeChoice::Jig => *jig_theme_id,
                    ThemeChoice::Override(theme_id) => theme_id
                }
            }
        }
    }

    pub fn selected_state_signal(&self, theme_id: ThemeId) -> impl Signal<Item = SelectedState> {
        map_ref! {
            let jig_theme_id = self.jig_theme_id.signal(),
            let selected_theme_id = self.theme_id_signal()
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
    None
}

pub struct ThemeSelectorCallbacks {
    pub on_change: Box<dyn Fn(ThemeChoice)>
}

impl ThemeSelectorCallbacks {
    pub fn new(on_change: impl Fn(ThemeChoice) + 'static) -> Self {
        Self {
            on_change: Box::new(on_change)
        }
    }
}
