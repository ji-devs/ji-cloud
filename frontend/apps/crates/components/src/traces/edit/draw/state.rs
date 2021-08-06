use std::rc::Rc;

use super::{menu::state::*, trace::state::*};
use dominator::clone;
use shared::domain::jig::module::body::_groups::design::Trace as RawTrace;

use futures_signals::{
    signal::{Mutable, Signal},
};
use utils::drag::Drag;

pub struct Draw {
    pub trace: DrawTrace,
    pub draw_points: Mutable<Vec<(f64, f64)>>,
    pub display_trace: Mutable<bool>,
    pub drag: Mutable<Option<Drag>>,
    pub menu: Mutable<Option<Menu>>,
    pub on_finished: Box<dyn Fn(Option<RawTrace>)>,
    pub init_index: Option<usize>,
}

impl Draw {
    pub fn new(
        init: Option<(usize, RawTrace)>,
        on_finished: impl Fn(Option<RawTrace>) + 'static,
    ) -> Self {
        let draw_points = Mutable::new(Vec::new());

        let menu: Mutable<Option<Menu>> = Mutable::new(None);

        let (init_index, init_trace, has_init) = match init {
            Some((index, trace)) => (Some(index), Some(trace), true),
            _ => (None, None, false),
        };

        let _self = Self {
            trace: DrawTrace::new(
                init_trace.clone(),
                Rc::new(Box::new(clone!(menu => move || {
                    //this will trigger menu re-positioning
                    if let Some(curr) = menu.get_cloned() {
                        menu.set(Some(curr));
                    }
                }))),
            ),
            menu,
            drag: Mutable::new(None),
            draw_points,
            display_trace: Mutable::new(false),
            on_finished: Box::new(on_finished),
            init_index,
        };

        if has_init {
            _self.display_trace.set_neq(true);
            _self.recreate_deco();
        }

        _self
    }

    pub fn reshape_menu_options_signal(&self) -> impl Signal<Item = bool> {
        self.draw_points.signal_ref(|points| points.len() > 2)
    }
}
