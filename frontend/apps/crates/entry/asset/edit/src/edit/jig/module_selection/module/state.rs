use std::rc::Rc;

use futures_signals::{
    map_ref,
    signal::{Mutable, Signal},
};
use shared::domain::module::ModuleKind;
use utils::drag::Drag;

pub struct State {
    pub kind: ModuleKind,
    pub drag: Mutable<Option<Rc<Drag>>>,
    pub hover: Mutable<bool>,
    pub show_autogen_tooltip: Mutable<bool>,
}

impl State {
    pub fn new(kind: ModuleKind) -> Self {
        Self {
            kind,
            drag: Mutable::new(None),
            hover: Mutable::new(false),
            show_autogen_tooltip: Mutable::new(false),
        }
    }

    pub fn hover_or_drag_signal(self: &Rc<Self>) -> impl Signal<Item = bool> {
        map_ref! {
            let hover = self.hover.signal(),
            let drag = self.drag.signal_cloned() => move {
                *hover || drag.is_some()
            }
        }
    }
}
