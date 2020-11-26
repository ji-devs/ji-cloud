use wasm_bindgen::prelude::*;

use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use futures_signals::{
    map_ref,map_mut,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use dominator::animation::{easing, Percentage, OnTimestampDiff};

pub const MOUSE_SCROLL_THRESHOLD: f64 = 200.0; // Number of pixels before it starts scrolling
pub const MOUSE_SCROLL_SPEED: f64 = 0.5; // Number of pixels to move per millisecond

pub fn normalize(value: f64, min: f64, max: f64) -> f64 {
    if min == max {
        0.0
    } else {
        ((value - min) * (1.0 / (max - min))).max(0.0).min(1.0)
    }
}
pub fn window_height() -> f64 {
    web_sys::window()
        .unwrap_throw()
        .inner_height()
        .unwrap_throw()
        .as_f64()
        .unwrap_throw()
}

#[derive(Debug)]
pub struct Scrolling {
    pub on_timestamp_diff: Mutable<Option<OnTimestampDiff>>,
    pub y: Mutable<f64>,
}

impl Scrolling {
    pub fn new() -> Self {
        Self {
            on_timestamp_diff: Mutable::new(None),
            y: Mutable::new(0.0),
        }
    }

    pub fn scroll_top_signal(&self) -> impl Signal<Item = Option<i32>> {
        map_ref! {
            let active = self.is_active_signal(),
            let y = self.y.signal()
            => {
                if *active {
                    Some(*y as i32)
                } else {
                    None
                }
            }
        }
    }
    pub fn is_active_signal(&self) -> impl Signal<Item = bool> {
        self.on_timestamp_diff.signal_ref(|x| x.is_some())
    }

    pub fn is_active(&self) -> bool {
        self.on_timestamp_diff.lock_ref().is_some()
    }

    //"borrowed" from https://github.com/Pauan/tab-organizer/blob/cde6e851b398afd4248e0bf3de3c56baada7fb28/src/sidebar/src/scrolling.rs#L7-L38
    //
    pub fn start(&self, mouse_y: i32, top: f64, bottom:f64) {
        let threshold = MOUSE_SCROLL_THRESHOLD / (bottom - top).abs();
        let percentage = normalize(mouse_y as f64, top, bottom);
        let percentage = percentage - 0.5;
        let sign = percentage.signum();
        let percentage = easing::cubic(Percentage::new(normalize(percentage.abs(), 0.5 - threshold, 0.5))).into_f64() * sign;

        if percentage == 0.0 {
            self.on_timestamp_diff.set(None);
        } else {
            let percentage = percentage * MOUSE_SCROLL_SPEED;

            let y = self.y.clone();

            // TODO initialize this inside of the OnTimestampDiff callback ?
            let starting_y = y.get();

            self.on_timestamp_diff.set(Some(OnTimestampDiff::new(move |diff| {
                y.set_neq(starting_y + (diff * percentage));
            })));
        }
    }

    pub fn stop(&self) {
        self.on_timestamp_diff.set(None);
    }
}

