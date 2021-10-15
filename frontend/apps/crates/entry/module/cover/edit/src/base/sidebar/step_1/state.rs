use std::rc::Rc;

use components::theme_selector::state::{ThemeSelector, ThemeSelectorCallbacks};
use dominator::clone;

use super::super::state::Sidebar;

pub struct Step1 {
    pub sidebar: Rc<Sidebar>,
    pub theme_selector: Rc<ThemeSelector>,
}

impl Step1 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        let callbacks = ThemeSelectorCallbacks::new(clone!(sidebar => move |theme_choice| {
            sidebar.base.set_theme(theme_choice);
        }));
        let theme_selector = Rc::new(ThemeSelector::new(
            sidebar.base.jig_id,
            sidebar.base.jig_theme_id.clone(),
            sidebar.base.theme_id.clone(),
            callbacks,
        ));

        Rc::new(Self {
            sidebar,
            theme_selector,
        })
    }
}
