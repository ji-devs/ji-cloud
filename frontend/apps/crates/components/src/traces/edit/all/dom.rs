use dominator::{html, Dom, clone, svg, class};
use std::rc::Rc;
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use crate::traces::{
    svg, 
    trace::state::*,
    edit::state::*
};

use web_sys::HtmlCanvasElement;
use awsm_web::canvas::get_2d_context;
use once_cell::sync::Lazy;
use std::fmt::Write;

pub fn render(state:Rc<Edit>) -> Dom { 
    //TODO render all traces
    html!("div")
}
