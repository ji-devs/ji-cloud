use std::rc::Rc;

use crate::{
    backgrounds::actions::Layer,
    module::{
        _common::edit::entry::prelude::BaseExt, _groups::design::edit::design_ext::DesignExt,
    },
};
use shared::domain::module::body::{ModeExt, StepExt};

use super::state::CustomBackground;

impl<Step, Mode, Base> CustomBackground<Step, Mode, Base>
where
    Step: StepExt + 'static,
    Mode: ModeExt + 'static,
    Base: BaseExt<Step> + DesignExt<Mode> + 'static,
{
    pub fn remove_overlay(self: &Rc<Self>) {
        self.base.get_backgrounds().delete_layer(Layer::Two);
    }
}
