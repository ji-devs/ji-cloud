use std::rc::Rc;

use dominator::clone;
use shared::domain::module::body::{ModeExt, StepExt};

use crate::module::{
    _common::edit::entry::prelude::BaseExt, _groups::design::edit::design_ext::DesignExt,
};

use super::{custom_background::CustomBackground, ThemeBackground};

impl<Step, Mode, Base> ThemeBackground<Step, Mode, Base>
where
    Step: StepExt + 'static,
    Mode: ModeExt + 'static,
    Base: BaseExt<Step> + DesignExt<Mode> + 'static,
{
    pub(super) fn open_custom_background(self: &Rc<Self>) {
        let state = self;

        let on_close = Box::new(clone!(state => move || {
            state.custom_background.set(None);
            state.tab_kind.set_neq(None);
        }));

        let custom_background = CustomBackground::new(Rc::clone(state), on_close);

        state.custom_background.set(Some(custom_background));
    }
}
