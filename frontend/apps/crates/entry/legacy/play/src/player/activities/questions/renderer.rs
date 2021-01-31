use dominator::{Dom, html, clone};
use legacy::*;
use super::state::*;
use wasm_bindgen::prelude::*;
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use std::cell::RefCell;
use utils::{events, resize::get_resize_info};
use web_sys::CanvasRenderingContext2d;
use crate::player::{
    Player,
    hotspot::Hotspot
};

pub struct QuestionsRenderer{}

impl QuestionsRenderer {
    pub fn render(player: Rc<Player>, ctx:Rc<CanvasRenderingContext2d>, data: Questions) -> Dom {
        let state = Rc::new(State::new(data));

        html!("empty-fragment", {
            .future(state.hotspot_signal().for_each(clone!(state, ctx => move |hotspot| {
                    if let Some((index, resize_info)) = hotspot {
                        let path = state.get_question_path(index);
                        Hotspot::render(&ctx, path, resize_info);
                    }
                    async {}
                }))
            )
            .event(clone!(state => move |evt:events::Click| {
                state.handle_click(evt.mouse_x(), evt.mouse_y());
            }))
            .after_inserted(clone!(state => move |_| {
                state.init_question(1);
            }))
        })
    }
}
