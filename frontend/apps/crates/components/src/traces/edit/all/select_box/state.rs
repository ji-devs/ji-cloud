use dominator::{html, Dom, clone, svg, class};
use std::rc::Rc;
use utils::{
    math::{vec2, quat, BoundsF64}, 
    prelude::*, 
    resize::{ResizeInfo, resize_info_signal}
};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::jig::module::body::Transform;
use super::{
    super::trace::state::*
};
use crate::traces::utils::TraceExt;

pub struct SelectBox {
    pub bounds: Mutable<Option<BoundsF64>>,
    pub menu_pos: Mutable<Option<(f64, f64)>>,
}

impl SelectBox {
    pub fn new() -> Self {
        Self {
            bounds: Mutable::new(None),
            menu_pos: Mutable::new(None),
        }
    }
    pub fn menu_pos_signal(
        &self, 
        active_signal: impl Signal<Item = bool>
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
