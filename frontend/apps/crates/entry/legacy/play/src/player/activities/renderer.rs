use legacy::*;
use dominator::{html, Dom};
use super::questions::renderer::QuestionsRenderer;
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::CanvasRenderingContext2d;
use crate::player::Player;

pub struct ActivityRenderer {
}

impl ActivityRenderer {
    pub fn render(player: Rc<Player>, ctx: Rc<CanvasRenderingContext2d>, activity: Option<Activity>) -> Dom {
        log::info!("CHANGING ACTIVITY");

        match activity {
            None => None,
            Some(activity) => {
                match activity {
                    Activity::Questions(data) => Some(QuestionsRenderer::render(player.clone(), ctx.clone(), data)),
                    _ => None
                }
            }
        }.unwrap_or(html!("empty-fragment"))

    }
}
