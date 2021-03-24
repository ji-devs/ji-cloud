use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::data::state::*;
use super::state::{State as CardState, Media};
use utils::events;
use components::image::element::ImageJi;

pub struct CardDom {
}

impl CardDom {
    pub fn render_main(state: Rc<State>, card: Rc<CardState>) -> Dom {
        html!("play-card", {
            .property_signal("flipped", card.flip.signal())
            .property("theme", &state.theme)
            .event(clone!(state, card => move |evt:events::Click| {
                card.flip.set_neq(true);
            }))
            .child(card.media_dom())
            
        })
    }
}

impl CardState {
    pub fn media_dom(&self) -> Dom {
        match &self.media {
            Media::Text(s) => {
                html!("card-text", {
                    .property("value", s)
                })
            },
            Media::Image(id, lib) => {
                ImageJi::render(id, *lib, None)
            },
            Media::Audio(id, lib) => {
                unimplemented!("can't add audio!")
            },
        }
    }
}
