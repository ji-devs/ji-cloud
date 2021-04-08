use dominator::{html, Dom, clone, with_node};
use web_sys::HtmlElement;
use std::rc::Rc;
use crate::data::state::*;
use super::state::{State as CardState, Media};
use utils::prelude::*;
use components::image::element::ImageJi;
use super::actions;
use futures_signals::signal::SignalExt;
use wasm_bindgen::prelude::*;

pub struct CardDom {
}

impl CardDom {
    pub fn render_main(state: Rc<State>, card: Rc<CardState>) -> Dom {
        let card_id = &card.id;

        html!("play-card", {
            .property_signal("flipped", card.is_flipped(&state))
            .property("theme", state.theme_id.as_str_id())
            .property("side", card.side.as_str())
            .style_signal("visibility", card.is_found().map(|flag| {
                if flag {
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
            .after_inserted(clone!(card => move |elem| {
                *card.main_elem.borrow_mut() = Some(elem);
            }))
            
            
        })
    }

    pub fn render_sidebar(state: Rc<State>, card: Rc<CardState>) -> Dom {
        let card_id = &card.id;

        html!("play-card", {
            .future(card.found_index.signal().for_each(clone!(state, card => move |found_index| {
                if let Some(found_index) = found_index {
                    actions::start_animation(&state, card.clone(), found_index);
                }
                async {}
            })))
            .property("side", card.side.as_str())
            .style_signal("display", card.is_found().map(|flag| {
                if flag {
                    "block"
                } else {
                    "none"
                }
            }))
            .property_signal("translateX", {
                card.transform_signal().map(|t| match t {
                    Some(t) => t.x,
                    None => 0.0
                })
            }) 
            .property_signal("translateY", {
                card.transform_signal().map(|t| match t {
                    Some(t) => t.y,
                    None => 0.0
                })
            }) 
            .property_signal("scale", {
                card.transform_signal().map(|t| match t {
                    Some(t) => t.scale,
                    None => 1.0 
                })
            }) 
            .property("flipped", true) 
            .property("theme", state.theme_id.as_str_id())
            .property("transform", true)
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
