/*
    The collision detection here is pixel-perfect even with advanced shapes
    The way we do it is, instead of creating a polygon for geometric tests,
    we render each shape to an offscreen canvas, where the color
    is derived directly from the shape's index.

    For example, shape 0's R value is 0, shape 1's R value is 1
    and this goes on, filling RGB sequentially

    Then, when we want to know which shape is at a coordinate,
    we read the pixel and reverse the process to get the shape index

    This can be used for anything, but so far here we then check how much
    of the target's bounding box overlaps here.

    The reason for not doing the same process on the target to be pixel-perfect
    in both directions, is that the use case was for the "drag and drop"
    module where the target is an HtmlImageElement with various levels of transparency.

    There might be an opportunity to support that, but it could get very tricky and/or slow.

    Note: it _might_ be possible to use alpha, but it's unclear whether this
    skews the internal data with blending, canvas composite mode, etc.

    So we only support up to 16,777,216 shapes... should be more than enough ;)

    See Puzzle in Legacy Player for an isolated example of the same technique, but for click only
*/
use awsm_web::{canvas::get_2d_context, dom::StyleExt};
use shared::domain::jig::module::body::{
    Transform,
    _groups::design::{Sticker, Trace},
};
use std::borrow::Cow;
use std::collections::HashMap;
use utils::{math::bounds, prelude::*, resize::get_resize_info};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

cfg_if::cfg_if! {
    if #[cfg(debug_assertions)] {
        const DEBUGGING_HIT:bool = false;
        const DEBUGGING_HIT_AABB:bool = false;
        const DEBUGGING_HIT_CLIP:bool = false;
        const DEBUGGING_HIT_SOURCE_COLOR:bool = false;
        const DEBUGGING_HIT_PERFORMANCE: bool = true;
    } else {
        const DEBUGGING_HIT:bool = false;
        const DEBUGGING_HIT_AABB:bool = false;
        const DEBUGGING_HIT_CLIP:bool = false;
        const DEBUGGING_HIT_SOURCE_COLOR:bool = false;
        const DEBUGGING_HIT_PERFORMANCE: bool = false;
    }
}

//Sticker size isn't necessarily needed for sprites?
//but it is for text atm and it serves as a sanity check that
//the sprite as loaded too
//

pub struct StickerHitSource<'a> {
    pub sticker: Cow<'a, Sticker>,
    pub size: (f64, f64),
    pub bounds_kind: StickerBoundsKind,
    pub transform_override: Option<Cow<'a, Transform>>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StickerBoundsKind {
    Media,
    BoundingBox,
    Auto,
}

pub async fn get_hit_index<'a, V: AsRef<Trace>>(
    source: StickerHitSource<'a>,
    traces: &[V],
) -> Option<usize> {
    let start = if !DEBUGGING_HIT_PERFORMANCE {
        0.0
    } else {
        web_sys::window()
            .unwrap_ji()
            .performance()
            .unwrap_ji()
            .now()
    };

    let resize_info = get_resize_info();

    let canvas: HtmlCanvasElement = web_sys::window()
        .unwrap_ji()
        .document()
        .unwrap_ji()
        .create_element("canvas")
        .unwrap_ji()
        .unchecked_into();

    canvas.set_width(resize_info.width as u32);
    canvas.set_height(resize_info.height as u32);

    let ctx = get_2d_context(&canvas, None).unwrap_ji();

    //playground for bit shifting: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=daf560b49dcbbbc86e7348b837e957f0
    //First draw the target areas
    for (index, trace) in traces.iter().enumerate() {
        let r = 0xFF & (index >> 16);
        let g = 0xFF & (index >> 8);
        let b = 0xFF & index;

        let color = if DEBUGGING_HIT {
            if index == 0 {
                "red".to_string()
            } else if index == 1 {
                "green".to_string()
            } else {
                "yellow".to_string()
            }
        } else {
            format!("#{:02x}{:02x}{:02x}", r, g, b)
        };

        crate::traces::canvas::draw_trace(&ctx, &resize_info, trace.as_ref());

        ctx.set_fill_style(&JsValue::from_str(&color));
        ctx.fill();
    }

    let StickerHitSource {
        sticker,
        size,
        bounds_kind,
        transform_override,
    } = source;

    let sticker = &*sticker;

    let transform = &*transform_override.unwrap_or(Cow::Borrowed(sticker.transform()));
    let oobb = bounds::oobb_transform_px(true, transform, Some(size), &resize_info);
    let aabb = oobb.to_aabb();

    ctx.save();

    let bounds_kind = match bounds_kind {
        StickerBoundsKind::Auto => {
            match sticker {
                Sticker::Text(_) => StickerBoundsKind::BoundingBox,
                Sticker::Video(_) => StickerBoundsKind::BoundingBox,
                Sticker::Sprite(_) => StickerBoundsKind::BoundingBox, //TODO - change to media when sticker.draw_to_canvas() exists
            }
        }
        StickerBoundsKind::BoundingBox => StickerBoundsKind::BoundingBox,
        StickerBoundsKind::Media => {
            match sticker {
                Sticker::Text(_) => {
                    //TODO - kick this down to sticker.draw_to_canvas() when it exists
                    panic!("can't get bounds for text media!");
                }
                Sticker::Sprite(_) => StickerBoundsKind::Media,
                Sticker::Video(_) => StickerBoundsKind::Media,
            }
        }
    };

    //Next draw our draggable's oobb
    //setting the composite rule so that it will only reveal pre-existing content
    if bounds_kind == StickerBoundsKind::BoundingBox {
        if DEBUGGING_HIT && DEBUGGING_HIT_AABB {
            aabb.draw_to_canvas(&ctx);
        } else {
            oobb.draw_to_canvas(&ctx);
        }

        if DEBUGGING_HIT && DEBUGGING_HIT_SOURCE_COLOR {
            ctx.set_fill_style(&JsValue::from_str("blue"));
        }

        if !DEBUGGING_HIT || DEBUGGING_HIT_CLIP {
            let _ = ctx.set_global_composite_operation("destination-in");
        }
    } else {
        panic!("sticker.draw_to_canvas() doesn't exist yet!");
    }

    ctx.fill();
    ctx.restore();

    //Use the bottom of the aabb instead of the top for getting the image data
    //just setting invert_y won't work (bug?) - maybe look into that or add an invert()
    //method
    let data = ctx
        .get_image_data(aabb.left(), aabb.bottom(), aabb.width, aabb.height)
        .unwrap_ji()
        .data()
        .to_vec();

    let width = aabb.width as usize;
    let height = aabb.height as usize;

    //accumulate the number of hits for each target index
    let mut hits: HashMap<u32, usize> = HashMap::new();

    for x in 0..width {
        for y in 0..height {
            let offset = y * (width * 4) + x * 4;
            let r = data[offset + 0] as u32;
            let g = data[offset + 1] as u32;
            let b = data[offset + 2] as u32;
            let a = data[offset + 3];

            //we use alpha just to check if there _is_ a hit
            if a != 0 {
                let index = (r << 16) | (g << 8) | b;

                let counter = hits.entry(index).or_insert(0);
                *counter += 1;
            }
        }
    }

    //Find the target with the greatest overlap
    let best_hit = hits
        .iter()
        .fold(None::<(u32, usize)>, |acc, (index, n_hits)| {
            let index = *index;
            let n_hits = *n_hits;

            let better = {
                if n_hits > 0 {
                    if let Some((_curr_index, curr_n_hits)) = acc {
                        n_hits > curr_n_hits
                    } else {
                        true
                    }
                } else {
                    false
                }
            };

            if better {
                Some((index, n_hits))
            } else {
                acc
            }
        })
        .map(|(index, _)| index as usize);

    if DEBUGGING_HIT {
        //Just for testing - to see the canvas
        canvas.set_style("position", "fixed");
        //canvas.set_style("opacity", "0.5");
        canvas.set_style("pointer-events", "none");
        canvas.set_style(
            "left",
            &format!("{}px", resize_info.x + resize_info.content_x),
        );
        canvas.set_style(
            "top",
            &format!("{}px", resize_info.y + resize_info.content_y),
        );

        let _ = web_sys::window()
            .unwrap_ji()
            .document()
            .unwrap_ji()
            .body()
            .unwrap_ji()
            .append_child(&canvas);
    }

    if DEBUGGING_HIT_PERFORMANCE {
        log::info!(
            "hit detection took {}ms",
            web_sys::window()
                .unwrap_ji()
                .performance()
                .unwrap_ji()
                .now()
                - start
        );
    };
    best_hit
}

pub fn debug_render_hit_trace<V: AsRef<Trace>>(index: usize, traces: &[V]) {
    let resize_info = get_resize_info();

    let canvas: HtmlCanvasElement = web_sys::window()
        .unwrap_ji()
        .document()
        .unwrap_ji()
        .create_element("canvas")
        .unwrap_ji()
        .unchecked_into();

    canvas.set_width(resize_info.width as u32);
    canvas.set_height(resize_info.height as u32);

    let ctx = get_2d_context(&canvas, None).unwrap_ji();

    if let Some(trace) = traces.get(index) {
        let color = "red".to_string();

        crate::traces::canvas::draw_trace(&ctx, &resize_info, trace.as_ref());
        ctx.set_fill_style(&JsValue::from_str(&color));
        ctx.fill();
    }

    //Just for testing - to see the canvas
    canvas.set_style("position", "fixed");
    //canvas.set_style("opacity", "0.5");
    canvas.set_style("pointer-events", "none");
    canvas.set_style(
        "left",
        &format!("{}px", resize_info.x + resize_info.content_x),
    );
    canvas.set_style(
        "top",
        &format!("{}px", resize_info.y + resize_info.content_y),
    );

    let _ = web_sys::window()
        .unwrap_ji()
        .document()
        .unwrap_ji()
        .body()
        .unwrap_ji()
        .append_child(&canvas);
}
