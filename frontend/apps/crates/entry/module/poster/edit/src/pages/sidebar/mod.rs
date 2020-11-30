mod layout;
mod images;

use std::rc::Rc;
use {
    layout::LayoutDom,
    images::ImagesDom
};
use crate::data::*;
use dominator::{Dom, clone};
use dominator_helpers::elem;
use futures_signals::signal::SignalExt;
use crate::templates;

pub fn render(state:Rc<State>) -> Dom {

    elem!(templates::sidebar(), {
        .child_signal(state.tool.signal().map(clone!(state => move |tool| {
            match tool {
                Tool::Layout => Some(LayoutDom::render(LayoutDom::new(state.clone()))),
                Tool::Images => Some(ImagesDom::render(ImagesDom::new(state.clone()))),
                _ => None
            }
        })))
    })
}
