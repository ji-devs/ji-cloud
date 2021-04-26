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
use utils::prelude::*;

pub struct PlayerDom { }

impl PlayerDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .child(state.instructions_player.render())
            .child(
                html!("play-container", {
                    .property("theme", state.theme_id.as_str_id())
                    .children(&mut [
                        MainDom::render(state.clone()),
                        SidebarDom::render(state.clone()),
                    ])
                })
            )
        })
    }
}
