use crate::base::sidebar::state::StickerPhase;

use super::state::*;
use components::module::_groups::design::edit::theme_background::ThemeBackground;
use dominator::Dom;
use std::rc::Rc;

pub fn render_step_1(state: Rc<Step1>) -> Dom {
    state
        .sidebar
        .sticker_phase
        .set_neq(Some(StickerPhase::Scene));
    state.sidebar.trace_phase.set_neq(None);

    let theme_background =
        ThemeBackground::new(state.sidebar.base.clone(), state.sidebar.tab_kind.clone());

    theme_background.render()
}
