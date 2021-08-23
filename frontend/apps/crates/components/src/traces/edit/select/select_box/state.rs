use futures_signals::{map_ref, signal::{Mutable, ReadOnlyMutable, Signal}};
use shared::domain::jig::module::body::Transform;
use utils::{drag::Drag, math::{BoundsF64, PointI32}};
use std::rc::Rc;
use dominator::clone;
use std::cell::RefCell;

pub struct SelectBox {
    pub menu_pos: Mutable<Option<(f64, f64)>>,
    pub drag: Mutable<Option<Rc<Drag>>>,
    pub transform_override: Mutable<Transform>,
    pub bounds: Mutable<Option<BoundsF64>>,
    pub elem: RefCell<Option<web_sys::SvgElement>>,
}

impl SelectBox {
    pub fn new(transform: Transform) -> Self {
        Self {
            menu_pos: Mutable::new(None),
            drag: Mutable::new(None),
            transform_override: Mutable::new(transform),
            bounds: Mutable::new(None),
            elem: RefCell::new(None),
        }
    }

    pub fn menu_pos_signal(
        &self,
        active_signal: impl Signal<Item = bool>,
    ) -> impl Signal<Item = Option<(f64, f64)>> {
        map_ref! {
            let active = active_signal,
            let pos = self.menu_pos.signal_cloned()
                => {
                    if !*active {
                        None
                    } else {
                        *pos
                    }
                }
        }
    }
}
