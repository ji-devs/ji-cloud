use super::super::state::*;
use components::module::_groups::cards::lookup::Side;
use dominator::animation::{easing, MutableAnimation, Percentage};
use futures_signals::signal::{Signal, SignalExt};
use utils::resize::get_resize_info;
use web_sys::HtmlElement;

pub struct Animation {
    pub animation: MutableAnimation,
    pub orig_x: f64,
    pub orig_y: f64,
    pub dest_x: f64,
    pub dest_y: f64,
    pub dest_scale: f64,
    pub side: Side,
}

pub struct AnimationState {
    pub x: f64,
    pub y: f64,
    pub scale: f64,
    pub finished: bool,
}

impl Animation {
    pub fn new(_base: &Base, element: &HtmlElement, found_index: usize, side: Side) -> Self {
        let animation = MutableAnimation::new(crate::config::TRANISITION_DURATION);
        animation.animate_to(Percentage::new(1.0));

        let (orig_x, orig_y) = get_resize_info().get_element_pos_rem(element);

        let mut dest_x = ((found_index % 2) * 280) as f64;
        let mut dest_y = 100.0 + (((found_index as f64) / 2.0).floor() * 280.0);

        if side == Side::Right {
            dest_x += 10.0;
            dest_y += 10.0;
        }

        let dest_scale = 0.5;

        Self {
            animation,
            orig_x,
            orig_y,
            dest_x,
            dest_y,
            dest_scale,
            side,
        }
    }

    pub fn ended_signal(&self) -> impl Signal<Item = bool> {
        self.state_signal().map(|t| t.finished).dedupe()
    }

    pub fn state_signal(&self) -> impl Signal<Item = AnimationState> {
        let orig_x = self.orig_x;
        let orig_y = self.orig_y;
        let dest_x = self.dest_x;
        let dest_y = self.dest_y;
        let dest_scale = self.dest_scale;
        self.animation
            .signal()
            .map(move |t| easing::in_out(t, easing::cubic))
            .map(move |t| AnimationState {
                x: t.range_inclusive(orig_x, dest_x),
                y: t.range_inclusive(orig_y, dest_y),
                scale: t.range_inclusive(1.0, dest_scale),
                finished: t.into_f64() == 1.0,
            })
    }
}
