use std::rc::Rc;

use crate::{
    backgrounds::actions::Layer,
    module::{
        _common::edit::entry::prelude::BaseExt, _groups::design::edit::design_ext::DesignExt,
    },
};
use shared::domain::jig::module::body::StepExt;

use super::state::CustomBackground;

impl<Step, Base> CustomBackground<Step, Base>
where
    Step: StepExt + 'static,
    Base: BaseExt<Step> + DesignExt + 'static,
{
    pub fn remove_overlay(self: &Rc<Self>) {
        self.base.get_backgrounds().delete_layer(Layer::Two);
    }
}
