/*
 * Drag is merely a lightweight state machine to track:
 * 1. Whether the pointer has moved enough to start dragging
 * 2. The current drag position
 *
 * The consumer is expected to:
 * 1. create/stash it on mouse down
 * 2. update it on global mouse move
 * 3. drop it on global mouse up
 *
 * In addition to the _tracking_ it provides signals of all the required state
 */

use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};
use std::{cell::RefCell, sync::atomic::AtomicBool};
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
use crate::{math::*, resize::get_resize_info};
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::SeqCst;

const MOVE_THRESHHOLD:i32 = 3;


pub struct Drag {
    state: Mutable<DragState>,
    pos: Mutable<PointI32>,
    mouse_x: AtomicI32,
    mouse_y: AtomicI32,
    immediate: bool,
}

impl Drag {
    pub fn new(mouse_x: i32, mouse_y: i32, anchor_x: f64, anchor_y: f64, immediate: bool) -> Self {
        let _self = Self { 
            state: Mutable::new(DragState::Waiting(DragWait {
                anchor: PointF64::new(anchor_x, anchor_y),
                accum: PointI32::new(0, 0),
            })),
            pos: Mutable::new(PointI32::new(0, 0)),
            mouse_x: AtomicI32::new(mouse_x),
            mouse_y: AtomicI32::new(mouse_y),
            immediate,
        };
        
        if _self.immediate {
            _self.update(mouse_x, mouse_y);
        }

        _self
    }
    pub fn new_anchor_element_resize(mouse_x: i32, mouse_y: i32, elem:&HtmlElement, immediate: bool) -> Self {

        let resize_info = get_resize_info();

        let (elem_x, elem_y) = resize_info.get_element_pos_px(&elem);

        let anchor_x = (mouse_x as f64) - elem_x;
        let anchor_y = (mouse_y as f64) - elem_y;

        Self::new(mouse_x, mouse_y, anchor_x, anchor_y, immediate)
    }
}

#[derive(Debug, Clone)]
pub enum DragState {
    Waiting(DragWait),
    Active
}

#[derive(Debug, Clone)]
pub struct DragWait {
    pub anchor: PointF64,
    pub accum: PointI32,
}

impl Drag {

    //Top-level state changes
    pub fn get_active(&self) -> bool { 
        match *self.state.lock_ref() {
            DragState::Active => true,
            _ => false 
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



    pub fn get_mouse(&self) -> PointI32 {
        PointI32::new(self.mouse_x.load(SeqCst), self.mouse_y.load(SeqCst))
    }

    pub fn set_mouse(&self, mouse: PointI32) {
        self.mouse_x.store(mouse.x, SeqCst);
        self.mouse_y.store(mouse.y, SeqCst);
    }
    pub fn update(&self, x:i32, y:i32) -> Option<(PointI32, PointI32)> {

        let prev_mouse = self.get_mouse();
        let next_mouse = PointI32::new(x, y);
        let diff = prev_mouse - next_mouse;
        let (next_state, next_pos) = match &mut *self.state.lock_mut() {
            DragState::Waiting(wait) => {
                wait.accum += diff;
                let next_state = {
                    if self.immediate || wait.accum.x.abs() > MOVE_THRESHHOLD || wait.accum.y.abs() > MOVE_THRESHHOLD {
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
                self.set_mouse(next_mouse);
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

        next_pos.map(|next_pos| (next_pos, diff))
    }
}
