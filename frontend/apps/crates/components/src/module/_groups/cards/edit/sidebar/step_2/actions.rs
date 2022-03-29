use std::rc::Rc;

use dominator::clone;

use crate::{
    module::_groups::cards::edit::state::{ExtraExt, RawDataExt},
    tabs::MenuTabKind,
};

use super::{custom_background::CustomBackground, state::Step2};

impl<RawData: RawDataExt, E: ExtraExt> Step2<RawData, E> {
    pub(super) fn open_custom_background(self: &Rc<Self>) {
        let state = self;

        let on_close = Box::new(clone!(state => move || {
            state.custom_background.set(None);
            state.tab_kind.set_neq(None);
        }));

        let custom_background =
            CustomBackground::new(Rc::clone(&state.base), state.tab_kind.clone(), on_close);

        state.custom_background.set(Some(custom_background));
        state.tab_kind.set_neq(Some(MenuTabKind::BackgroundImage));
    }
}
