use crate::base::sidebar::state::Sidebar;
use std::rc::Rc;

use components::theme_selector::state::{ThemeSelector, ThemeSelectorCallbacks};
use dominator::clone;

pub struct Step1 {
    pub sidebar: Rc<Sidebar>,
    pub theme_selector: Rc<ThemeSelector>,
}

impl Step1 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        let base = sidebar.base.clone();

        let callbacks = ThemeSelectorCallbacks::new(clone!(base => move |theme_choice| {
            base.set_theme(theme_choice);
        }));
        let theme_selector = Rc::new(ThemeSelector::new(
            base.jig_id,
            base.jig_theme_id.clone(),
            base.theme_id.clone(),
            callbacks,
        ));

        Rc::new(Self {
            sidebar,
            theme_selector,
        })
    }
}
