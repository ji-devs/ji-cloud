use dominator_helpers::signals::{box_signal_fn, BoxSignalFn};
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::jig::module::body::Transform;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use utils::math::{bounds::BoundsF64, transform_signals};

pub struct BoxOutline {
    pub aabb_signal: BoxSignalFn<BoundsF64>,
    pub style: BoxOutlineStyle,
    pub top_right_hover_only: AtomicBool,
    pub top_left_hover_only: AtomicBool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoxOutlineStyle {
    Regular,
    Hidden,
}

impl BoxOutlineStyle {
    pub const fn line_hidden(&self) -> bool {
        match self {
            Self::Hidden => true,
            _ => false,
        }
    }
}

impl BoxOutline {
    pub fn new_transform_size<F, FSig, S, SSig>(
        style: BoxOutlineStyle,
        transform_signal: F,
        size_signal: S,
    ) -> Rc<Self>
    where
        F: Fn() -> FSig + 'static,
        FSig: Signal<Item = Transform> + 'static,
        S: Fn() -> SSig + 'static,
        SSig: Signal<Item = Option<(f64, f64)>> + 'static,
    {
        Self::new(style, move || {
            transform_signals::aabb_bounds_px(true, transform_signal(), size_signal()).map(
                |mut bounds| {
                    bounds.set_invert_y(true);
                    bounds
                },
            )
        })
    }
    pub fn new<F, FSig>(style: BoxOutlineStyle, aabb_signal: F) -> Rc<Self>
    where
        F: Fn() -> FSig + 'static,
        FSig: Signal<Item = BoundsF64> + 'static,
    {
        Rc::new(Self {
            style,
            aabb_signal: box_signal_fn(aabb_signal),
            top_right_hover_only: AtomicBool::new(false),
            top_left_hover_only: AtomicBool::new(false),
        })
    }

    pub fn get_top_right_hover_only(&self) -> bool {
        self.top_right_hover_only.load(Ordering::SeqCst)
    }

    pub fn set_top_right_hover_only(&self, flag: bool) {
        self.top_right_hover_only.store(flag, Ordering::SeqCst);
    }

    pub fn get_top_left_hover_only(&self) -> bool {
        self.top_left_hover_only.load(Ordering::SeqCst)
    }

    pub fn set_top_left_hover_only(&self, flag: bool) {
        self.top_left_hover_only.store(flag, Ordering::SeqCst);
    }
    pub fn left_style_signal(&self) -> impl Signal<Item = String> {
        (self.aabb_signal)().map(|bounds| format!("{}px", bounds.x))
    }
    pub fn top_style_signal(&self) -> impl Signal<Item = String> {
        (self.aabb_signal)().map(|bounds| format!("{}px", bounds.y))
    }
    pub fn width_style_signal(&self) -> impl Signal<Item = String> {
        (self.aabb_signal)().map(|bounds| format!("{}px", bounds.width))
    }
    pub fn height_style_signal(&self) -> impl Signal<Item = String> {
        (self.aabb_signal)().map(|bounds| format!("{}px", bounds.height))
    }
}
