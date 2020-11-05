use wasm_bindgen::prelude::*;
use web_sys::Element;
use std::cell::RefCell;
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use futures_signals::{
    map_ref,map_mut,
    signal::{Mutable, MutableSignal, SignalExt, Signal, always},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use std::pin::Pin;
use std::marker::Unpin;
use std::future::Future;
use std::task::{Context, Poll};
use super::data::*;

const MOVE_THRESHHOLD:i32 = 3;


#[derive(Debug, Clone)]
pub struct Dragging {
    pub state: Mutable<DragState>
}

impl Dragging {
    pub fn new() -> Self {
        Self { state: Mutable::new(DragState::None) }
    }
}

#[derive(Debug, Clone)]
pub enum DragState {
    None,
    Waiting(DragWait),
    Active(DragActive)
}

impl DragState {
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub struct DragWait {
    pub src_index: usize, 
    pub src_size: DragSize,
    pub mouse: DragPoint,
    pub accum: DragPoint,
    pub module: Module,
    pub module_elements: Vec<Element> 
}

#[derive(Debug, Clone)]
pub struct DragActive {
    pub src_index: usize, 
    pub dest_index: Mutable<usize>, 
    pub pos: Mutable<DragPoint>,
    pub mouse: DragPoint,
    pub module: Module,
    pub module_elements: Vec<Element> 
}

#[derive(Debug, Clone, Copy)]
pub struct DragPoint {
    x: i32,
    y: i32
}
impl DragPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }
}
impl std::ops::Sub for DragPoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::SubAssign for DragPoint {
    fn sub_assign(&mut self, other: Self) {
        *self = DragPoint {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::Add for DragPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::AddAssign for DragPoint {
    fn add_assign(&mut self, other: Self) {
        *self = DragPoint {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DragSize {
    width: f64,
    height: f64, 
}
impl DragSize {
    pub fn new(width: f64, height: f64) -> Self {
        Self {width, height }
    }
}

impl std::ops::Div for DragSize {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            width: self.width / other.width,
            height: self.height / other.height,
        }
    }
}

pub struct StyleSignal {
    pos: Option<MutableSignal<DragPoint>>,
    prop: StyleSignalProp,
    none_has_fired: bool
}

enum StyleSignalProp {
    X,
    Y
}

impl StyleSignal {
    fn new(pos: Option<MutableSignal<DragPoint>>, prop: StyleSignalProp) -> Self {
        Self {
            pos,
            prop,
            none_has_fired: false,
        }
    }
}

/*
 *
 * a Signal must always return Poll::Ready(Some(...)) the first time it is called
always
after that it can return either Poll::Ready(Some(...)), Poll::Pending, or Poll::Ready(None)
and if it returns Poll::Ready(None), then from that point forward it must always return Poll::Ready(None)
*/
impl Signal for StyleSignal {
    type Item = String;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.pos {
            None => {
                if self.none_has_fired {
                    Poll::Ready(None)
                } else {
                    self.none_has_fired = true;
                    Poll::Ready(Some("0px".to_string()))
                }
            }
            Some(pos) => {
                Pin::new(pos)
                    .poll_change(cx)
                    .map(|pos| {
                        pos.map(|pos| {
                            let value = match self.prop {
                                StyleSignalProp::X => pos.x,
                                StyleSignalProp::Y => pos.y,
                            };
                            format!("{}px", value)
                        })
                    })
            }
        }

    }
}


pub struct IndexSignal {
    index: Option<MutableSignal<usize>>,
    none_has_fired: bool
}

impl IndexSignal {
    fn new(index: Option<MutableSignal<usize>>) -> Self {
        Self {
            index,
            none_has_fired: false,
        }
    }
}

impl Signal for IndexSignal {
    type Item = Option<usize>;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.index{
            None => {
                if self.none_has_fired {
                    Poll::Ready(None)
                } else {
                    self.none_has_fired = true;
                    Poll::Ready(Some(None))
                }
            }
            Some(index) => {
                Pin::new(index)
                    .poll_change(cx)
                    .map(|value| Some(value))
            }
        }

    }
}

impl Dragging {
    pub fn active_signal(&self) -> impl Signal<Item = bool> {
        self.state.signal_ref(move |state| {
            match state {
                DragState::None | DragState::Waiting(_) => false,
                _ => true
            }
        })
    }

    pub fn module_signal(&self) -> impl Signal<Item = Option<Module>> {
        self.state.signal_ref(move |state| {
            match state {
                DragState::None | DragState::Waiting(_) => None,
                DragState::Active(active) => Some(active.module.clone()), 
            }
        })
    }
    pub fn src_index_signal(&self) -> impl Signal<Item = Option<usize>> {
        self.state.signal_ref(move |state| {
            match state {
                DragState::None | DragState::Waiting(_) => None,
                DragState::Active(active) => Some(active.src_index), 
            }
        })
    }
    pub fn dest_index_signal(&self) -> impl Signal<Item = Option<usize>> {
        self.state.signal_ref(move |state| {
            match state {
                DragState::None | DragState::Waiting(_) => {
                    IndexSignal::new(None)
                }, 

                DragState::Active(state) => {
                    IndexSignal::new(Some(state.dest_index.signal()))
                }
            }
        })
        .flatten()
    }

    pub fn top_style_signal(&self) -> impl Signal<Item = String> {
        self.state.signal_ref(move |state| {
            match state {
                DragState::None | DragState::Waiting(_) => {
                    StyleSignal::new(None, StyleSignalProp::Y)
                }, 

                DragState::Active(state) => StyleSignal::new(Some(state.pos.signal()), StyleSignalProp::Y)
            }
        })
        .flatten()
    }
    pub fn left_style_signal(&self) -> impl Signal<Item = String> {
        self.state.signal_ref(move |state| {
            match state {
                DragState::None | DragState::Waiting(_) => {
                    StyleSignal::new(None, StyleSignalProp::X)
                }, 

                DragState::Active(state) => StyleSignal::new(Some(state.pos.signal()), StyleSignalProp::X)
            }
        })
        .flatten()
    }

    pub fn start_drag(
        &self, 
        src_index: usize, 
        mouse_x: i32, mouse_y: i32, 
        src_width: f64, src_height: f64, 
        module: Module,
        module_elements: Vec<Element> 
    ) {
        self.state.set(DragState::Waiting(DragWait {
            src_index,
            src_size: DragSize::new(src_width, src_height),
            mouse: DragPoint::new(mouse_x, mouse_y),
            accum: DragPoint::new(0, 0),
            module,
            module_elements
        }))
    }

    pub fn stop_drag(&self) {
        self.state.set(DragState::None);
    }

    pub fn on_move(&mut self, x:i32, y:i32) {
        let curr_mouse = DragPoint::new(x, y);
        let state = &mut *self.state.lock_mut();

        match state {
            DragState::Waiting(wait) => {
                let diff = wait.mouse - curr_mouse;
                wait.accum += diff;
                if wait.accum.x > MOVE_THRESHHOLD || wait.accum.y > MOVE_THRESHHOLD {

                    let offset = wait.src_size / DragSize::new(2.0, 2.0);
                    let pos = DragPoint::new(
                        curr_mouse.x - offset.width as i32, 
                        curr_mouse.y - offset.height as i32
                    );

                    *state = DragState::Active(DragActive {
                        src_index: wait.src_index,
                        dest_index: Mutable::new(wait.src_index),
                        pos: Mutable::new(pos),
                        mouse: curr_mouse,
                        module: wait.module.clone(),
                        module_elements: wait.module_elements.clone()
                    });
                }
            },

            DragState::Active(active) => {
                let diff = active.mouse - curr_mouse;
                active.mouse = curr_mouse;
                active.pos.replace_with(|pos| DragPoint::new(pos.x - diff.x, pos.y - diff.y));
            },

            DragState::None => {},
        }
    }
}
