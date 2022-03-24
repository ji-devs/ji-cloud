use std::{rc::Rc, cell::RefCell};

use futures_signals::{signal::{Mutable, Signal}, map_ref};
use shared::domain::jig::ModuleKind;
use utils::drag::Drag;
use web_sys::HtmlElement;

pub struct State {
    pub kind: ModuleKind,
    pub drag: Mutable<Option<Rc<Drag>>>,
    pub element_hovered: Rc<RefCell<Option<HtmlElement>>>,
    pub hover: Mutable<bool>,
}

impl State {
    pub fn new(kind: ModuleKind) -> Self {
        Self {
            kind,
            drag: Mutable::new(None),
            hover: Mutable::new(false),
            element_hovered: Rc::new(RefCell::new(None)),
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
