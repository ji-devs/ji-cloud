use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::data::state::*;
use super::state::{State as CardState, Media};
use utils::events;
use components::image::element::ImageJi;
use super::actions;
use futures_signals::signal::SignalExt;

pub struct CardDom {
}

impl CardDom {
    pub fn render_main(state: Rc<State>, card: Rc<CardState>) -> Dom {
        let card_id = &card.id;

        html!("play-card", {
            .property_signal("flipped", card.is_flipped(&state))
            .property("theme", &state.theme)
            .style_signal("visibility", card.is_found().map(|is_found| {
                if is_found {
                    "hidden"
                } else {
                    "visible"
                }
            }))
            .child(card.media_dom())
            .event(clone!(state, card_id => move |evt:events::Click| {
                if let Some((id_1, id_2)) = actions::card_click(state.clone(), card_id) {
                    actions::evaluate(state.clone(), id_1, id_2);
                }
            }))
            
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
