use web_sys::CanvasRenderingContext2d;
use utils::prelude::*;
use super::{SpriteData, SpritePhase};

pub struct SpritePlayer {

    pub ctx: CanvasRenderingContext2d, 
    pub data: SpriteData,
    pub phase: SpritePhase
}

impl SpritePlayer {
    pub fn new(ctx: CanvasRenderingContext2d, data: SpriteData, phase: SpritePhase) -> Self {

        ctx.clear_rect(0.0, 0.0, ctx.canvas().unwrap_ji().width() as f64, ctx.canvas().unwrap_ji().height() as f64);

        match &data {
            SpriteData::Static(img) => {
                ctx.draw_image_with_html_image_element(&img, 0.0, 0.0).unwrap_ji();
            }
        }

        Self {
            ctx,
            data,
            phase
        }
    }
}