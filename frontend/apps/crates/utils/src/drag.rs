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
use shared::domain::jig::ModuleKind;
use crate::math::*;

const MOVE_THRESHHOLD:i32 = 3;


#[derive(Debug, Clone)]
pub struct BasicDrag {
    state: Mutable<DragState>,
    pos: Mutable<PointI32>,
    mouse: PointI32,
}

impl BasicDrag {
    pub fn new() -> Self {
        Self { 
            state: Mutable::new(DragState::None),
            pos: Mutable::new(PointI32::new(0, 0)),
            mouse: PointI32::new(0, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DragState {
    None,
    Waiting(DragWait),
    Active
}

#[derive(Debug, Clone)]
pub struct DragWait {
    pub anchor: PointF64,
    pub accum: PointI32,
}

impl BasicDrag {

    //Top-level state changes
    pub fn get_active(&self) -> bool { 
        match *self.state.lock_ref() {
            DragState::Active => true,
            _ => false 
        }
    }
    pub fn get_listening(&self) -> bool { 
        match *self.state.lock_ref() {
            DragState::Active | DragState::Waiting(_) => true,
            DragState::None => false 
        }
    }

    pub fn active_signal(&self) -> impl Signal<Item = bool> {
        self.state.signal_ref(move |state| {
            match state {
                DragState::Active => true,
                _ => false 
            }
        })
    }

    //position
    pub fn get_pos(&self) -> Option<PointI32> { 
        if self.get_active() {
            Some(self.pos.get())
        } else {
            None
        }
    }

    pub fn pos_signal(&self) -> impl Signal<Item = Option<PointI32>> {
        map_ref! {
            let active = self.active_signal(),
            let pos = self.pos.signal()
            => {
                if !*active {
                    None
                } else {
                    Some(*pos)
                }
            }
        }
    }

    //bounds
    pub fn get_bounds(&self, width: f64, height: f64, invert_y: bool) -> Option<BoundsF64> { 
        self.get_pos()
            .map(|pos| {
                BoundsF64 {x: pos.x as f64, y: pos.y as f64, width, height, invert_y}
            })
    }

    //when the width,height is static, use this. otherwise - see `bounds_signal()` below
    pub fn get_bounds_signal(&self, width: f64, height: f64, invert_y:bool) -> impl Signal<Item = Option<BoundsF64>> {
        self.pos_signal()
            .map(move |pos| {
                pos.map(|pos| {
                    BoundsF64 {x: pos.x as f64, y: pos.y as f64, width, height, invert_y}
                })
            })
    }


    // a signal of width,height and a signal of pos combined
    pub fn bounds_signal<WH: Signal<Item = (f64, f64)>>(&self, width_height_signal:WH, invert_y: bool) -> impl Signal<Item = Option<BoundsF64>> {
        map_ref! {
            let (width, height) = width_height_signal,
            let pos = self.pos_signal()
            => move {
                let width = *width;
                let height = *height;
                pos.map(|pos| {
                    BoundsF64 {x: pos.x as f64, y: pos.y as f64, width, height, invert_y}
                })
            }
        }
    }



    pub fn transform_signal(&self) -> impl Signal<Item = String> {
        self.pos_signal()
            .map(|pos| {
                match pos {
                    None => "none".to_string(),
                    Some(pos) => format!("translate({}px, {}px)", pos.x, pos.y)
                }
            })
    }



    // Engine start/update/stop
    pub fn start(
        &mut self, 
        mouse_x: i32, mouse_y: i32, 
        anchor_x: f64, anchor_y: f64, 
    ) {

        self.mouse = PointI32::new(mouse_x, mouse_y);

        self.state.set(DragState::Waiting(DragWait {
            anchor: PointF64::new(anchor_x, anchor_y),
            accum: PointI32::new(0, 0),
        }))
    }

    pub fn stop(&self) {
        *self.state.lock_mut() = DragState::None;
    }

    pub fn on_move(&mut self, x:i32, y:i32) -> Option<PointI32> {

        if self.get_listening() {
            let prev_mouse = self.mouse;
            let next_mouse = PointI32::new(x, y);
            let diff = prev_mouse - next_mouse;
            let (next_state, next_pos) = match &mut *self.state.lock_mut() {
                DragState::Waiting(wait) => {
                    wait.accum += diff;
                    let next_state = {
                        if wait.accum.x > MOVE_THRESHHOLD || wait.accum.y > MOVE_THRESHHOLD {
                            self.pos.set(PointI32::new(
                                next_mouse.x - wait.anchor.x as i32, 
                                next_mouse.y - wait.anchor.y as i32
                            ));
                            Some(DragState::Active)
                        } else {
                            None
                        }
                    };
                    (next_state, None)
                },

                DragState::Active => {
                    self.mouse = next_mouse;
                    let pos = self.pos.get();
                    let next_pos = Some(PointI32::new(pos.x - diff.x, pos.y - diff.y));
                    (None, next_pos)
                },

                _ => (None,None)
            };

            if let Some(next_state) = next_state {
                self.state.set(next_state);
            }

            if let Some(next_pos) = next_pos {
                self.pos.set(next_pos);
            }

            next_pos
        } else {
            None
        }
    }
}
