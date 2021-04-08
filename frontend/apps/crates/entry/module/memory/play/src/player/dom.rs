use dominator::{html, clone, Dom};
use crate::data::state::*;
use std::rc::Rc;
use super::sections::{
    main::dom::MainDom,
    sidebar::dom::SidebarDom,
};
use futures_signals::{
    signal::SignalExt,
    signal_vec::SignalVecExt
};

pub struct PlayerDom {}

impl PlayerDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("play-container", {
            .property("slot", "main")
            .children(&mut [
                MainDom::render(state.clone()),
                SidebarDom::render(state.clone()),
            ])
        })
    }
}
