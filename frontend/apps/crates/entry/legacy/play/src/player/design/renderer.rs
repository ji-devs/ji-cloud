use legacy::*;
use dominator::{Dom, html};
use super::stickers::image::StickerImage;
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;
use crate::player::Player;

pub struct DesignRenderer {
}

impl DesignRenderer {
    pub fn render(player: Rc<Player>, ctx: Rc<CanvasRenderingContext2d>, design: Design) -> Dom {

        let mut children:Vec<Dom> = Vec::new();


        if let Some(bg) = design.bg.as_ref() {
            children.push(
                html!("img-legacy", {
                    .property("jigId", &player.jig_id)
                    .property("moduleId", &player.module_id)
                    .property("path", &format!("layers/{}", bg))
                    .apply_if(player.is_mock, |dom| {
                        dom.property("mock", true)
                    })
                })
            )
        }
       
        for sticker in design.stickers.iter() {
            match sticker {
                Sticker::Text(_) => {},
                Sticker::Image(img) => {
                    children.push(
                        StickerImage::render(&player.jig_id, &player.module_id, &img, player.is_mock)
                    )
                }
            }
        }

        html!("empty-fragment", {
            .children(children)
        })
    }
}
