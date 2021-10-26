use crate::base::state::Base;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::{borrow::Borrow, rc::Rc, cell::RefCell};
use utils::{
    math::{bounds, mat4::Matrix4},
    path,
    prelude::*,
    resize::resize_info_signal,
};
use super::state::Sticker;

// http://localhost:4104/module/legacy/play/debug?game_id=17736&slide_index=0&example=true
impl Sticker {
    pub fn render(self: Self) -> Dom {
        match self {
            Self::Image(state) => state.render(),
            Self::Animation(state) => state.render()
        }
    }
}
