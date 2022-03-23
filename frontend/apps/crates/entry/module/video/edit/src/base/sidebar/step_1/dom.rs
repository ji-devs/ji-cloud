use super::state::*;
use components::module::_groups::design::edit::theme_background::ThemeBackground;
use dominator::Dom;
use std::rc::Rc;

pub fn render(state: Rc<Step1>) -> Dom {
    let theme_background = ThemeBackground::new(
        state.sidebar.base.clone(),
        state.sidebar.tab_kind.clone(),
    );

    theme_background.render()
}
