use dominator::{html, clone, Dom};
use crate::data::state::*;
use std::rc::Rc;
use super::sections::{
    header::dom::HeaderDom,
    main::dom::MainDom,
    sidebar::dom::SidebarDom
};

pub struct PlayerDom {}

impl PlayerDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("play-container", {
            .property("slot", "main")
            .children(&mut [
                HeaderDom::render(state.clone()),
                MainDom::render(state.clone()),
                SidebarDom::render(state.clone()),
            ])
        })
    }
}
