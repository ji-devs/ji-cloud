use std::rc::Rc;

use super::super::ModuleSelection;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal},
};
use shared::domain::module::ModuleKind;

pub struct ModuleSelectionItem {
    pub kind: ModuleKind,
    pub hover: Mutable<bool>,
    pub show_autogen_tooltip: Mutable<bool>,
    pub module_selection_state: Rc<ModuleSelection>,
}

impl ModuleSelectionItem {
    pub fn new(kind: ModuleKind, module_selection_state: &Rc<ModuleSelection>) -> Rc<Self> {
        Rc::new(Self {
            kind,
            hover: Mutable::new(false),
            show_autogen_tooltip: Mutable::new(false),
            module_selection_state: Rc::clone(module_selection_state),
        })
    }

    pub fn hover_or_drag_signal(self: &Rc<Self>) -> impl Signal<Item = bool> {
        let state = Rc::clone(self);

        map_ref! {
            let hover = self.hover.signal(),
            let drag = self.module_selection_state.drag.signal_cloned() => move {
                let dragging = matches!(
                    drag,
                    Some(drag) if drag.data == state.kind
                );

                *hover || dragging
            }
        }
    }
}
