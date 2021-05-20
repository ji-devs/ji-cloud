use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::signal::SignalExt;

pub fn render(state: Rc<Step4>) -> Dom {
    html!("div")
}

