use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::data::state::*;
use crate::player::card::dom::CardDom;

pub struct SidebarDom {
}

impl SidebarDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("play-sidebar", {
            .property("slot", "sidebar")
            .children(

                //It's simpler to just always render but hide via CSS until we start the animation
                state.cards
                    .iter()
                    .map(clone!(state => move |card| {
                        CardDom::render_sidebar(state.clone(), card.clone())
                    }))
            )
        })
    }
}

