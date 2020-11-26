use wasm_bindgen::prelude::*;
use web_sys::Element;
use std::cell::RefCell;
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use futures_signals::{
    map_ref,map_mut,
    signal::{Mutable, MutableSignal, SignalExt, Signal, always, Map},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use std::pin::Pin;
use std::marker::Unpin;
use std::future::Future;
use std::task::{Context, Poll};
use super::data::*;
use shared::domain::jig::ModuleKind;
use utils::{
    signals::*,
    math::*,
    drag::*,
};
use std::rc::Rc;

pub struct ReorderDrag {
    pub drag: BasicDrag,
    init: Rc<RefCell<Option<Init>>>,
    swap_to_index: Mutable<Option<usize>>,
}

struct Init {
    pub original_index: usize, 
    pub swap_from_index: usize, 
    pub src_size: RectF64,
    pub module: Module,
    pub module_elements: Vec<Element>,
    pub module_kinds: Vec<Option<ModuleKind>>,
}

impl ReorderDrag {
    pub fn new() -> Self {
        Self {
            drag: BasicDrag::new(),
            init: Rc::new(RefCell::new(None)),
            swap_to_index: Mutable::new(None),
        }
    }

    pub fn start(
        &mut self, 
        src_index: usize, 
        mouse_x: i32, mouse_y: i32, 
        src_width: f64, src_height: f64, 
        module: Module,
        module_elements: Vec<Element> ,
        module_kinds: Vec<Option<ModuleKind>>
    ) {
        let init = Init {
            original_index: src_index,
            swap_from_index: src_index,
            src_size: RectF64::new(src_width, src_height),
            module,
            module_elements,
            module_kinds
        };
        *self.init.borrow_mut() = Some(init);
        self.drag.start(mouse_x, mouse_y, src_width / 2.0, src_height / 2.0);
    }


    pub fn stop(&self) -> Option<(usize, usize)> {

        let result:Option<(usize, usize)> = self.init.borrow().as_ref().and_then(|init| {
            self.swap_to_index.get().and_then(|swap_to_index| {
                Some((init.original_index, swap_to_index))
            })
        });

        *self.init.borrow_mut() = None; 
        self.drag.stop();
        self.swap_to_index.set(None);

        result
    }

    //proprietary to re-order
    pub fn hover_src_signal(&self) -> impl Signal<Item = String> {
        let init = self.init.clone();

        self.drag.active_signal().map(clone!(init => move |active| {
            {
                if !active {
                    None
                } else {
                    init.borrow().as_ref().map(|init| {
                        init.module.kind.get_thumbnail()
                    })
                }
            }
            .unwrap_or("".to_string())
        }))
    }

    pub fn target_kind_signal(&self, default_kind: Option<ModuleKind>, index:usize) -> impl Signal<Item = Option<ModuleKind>> {
        let init = self.init.clone();

        self.swap_to_index.signal()
            .map(move |swap_to_index| {
                init.borrow().as_ref().and_then(|init| {
                    //we're only using the swap_to_index signal
                    //in order to determine IF we're using the local
                    //module_kinds list. The index is consistent from the outside
                    swap_to_index.map(|_| {
                        init.module_kinds[index]
                    })
                })
                .unwrap_or(default_kind)
            })
    }

    fn update_index(&mut self) {
        if let Some(init) = self.init.borrow_mut().as_mut() {
            if let Some(drag_bounds) = self.drag.get_bounds(
                init.src_size.width / 2.0, 
                init.src_size.height / 2.0, 
                true
            ) {
                for (idx, element) in init.module_elements.iter().enumerate() {
                    if Some(idx) == self.swap_to_index.get() {
                        continue;
                    }
                    let target_bounds:BoundsF64 = element.into();
                    if target_bounds.contains(drag_bounds) {
                        let kind = init.module_kinds.remove(init.swap_from_index);
                        init.module_kinds.insert(idx, kind);
                        init.swap_from_index = idx;
                        self.swap_to_index.set_neq(Some(idx));
                        break;
                    }
                }
            }
        }
    }

    //just passing on the underlying drag callbacks
    pub fn on_move(&mut self, mouse_x: i32, mouse_y: i32) -> Option<PointI32> {
        let pos = self.drag.on_move(mouse_x, mouse_y);
        if pos.is_some() {
            self.update_index();
        }
        pos
    }

    pub fn active_signal(&self) -> impl Signal<Item = bool> {
        self.drag.active_signal()
    }
    pub fn transform_signal(&self) -> impl Signal<Item = String> {
        self.drag.transform_signal()
    }
}
