use std::rc::Rc;
use utils::math::{BoundsF64, bounds, quat, vec2};
use utils::{prelude::*, drag::Drag, resize::get_resize_info};
use super::state::*;
use shared::domain::jig::module::body::_groups::design::Sticker;
use awsm_web::{dom::StyleExt, canvas::get_2d_context};
use web_sys::{HtmlCanvasElement};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

const DEBUGGING_EVALUATION_RESULT:bool = true;
const DEBUGGING_HIT:bool = false;
const DEBUGGING_HIT_AABB:bool = false;
const DEBUGGING_HIT_CLIP:bool = false;

impl PlayState {
    pub fn evaluate(&self, item: &InteractiveItem) {


        let start = web_sys::window().unwrap_ji().performance().unwrap_ji().now();

        if let Some(index) = self.get_hit_index(item) {
            log::info!("hit: {} time: {}", index, web_sys::window().unwrap_ji().performance().unwrap_ji().now() - start);
            if DEBUGGING_EVALUATION_RESULT {
                self.debug_render_hit(index);
            }
        }

    }

    pub fn get_hit_index(&self, item: &InteractiveItem) -> Option<usize> {


        item.size
            .get_cloned()
            .and_then(|size| {
                let resize_info = get_resize_info();

                let canvas:HtmlCanvasElement = web_sys::window()
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
                for (index, target_area) in self.game.base.target_areas.iter().enumerate() {
                    let r = 0xFF & (index >> 16);
                    let g = 0xFF & (index >> 8);
                    let b = 0xFF & index;

                    let color = if DEBUGGING_HIT {
                        if index == 0 {
                            format!("red")
                        } else if index == 1 {
                            format!("green")
                        } else {
                            format!("yellow")
                        }
                    } else {
                        format!("#{:02x}{:02x}{:02x}", r, g, b)
                    };


                    let trace = &target_area.trace;

                    components::traces::canvas::draw_trace(&ctx, &resize_info, &trace);
                    ctx.set_fill_style(&JsValue::from_str(&color));
                    ctx.fill();
                }

                //Next draw our draggable's oobb
                //setting the composite rule so that it will only reveal pre-existing content
                let transform = item.curr_transform.get_cloned();
                let oobb = bounds::oobb_transform_px(true, &transform, Some(size), &resize_info);
                ctx.save();
                if DEBUGGING_HIT {
                    if DEBUGGING_HIT_AABB {
                        oobb.to_aabb().draw_to_canvas(&ctx);
                    } else {
                        oobb.draw_to_canvas(&ctx);
                    }
                    ctx.set_fill_style(&JsValue::from_str("blue"));
                    if DEBUGGING_HIT_CLIP {
                        ctx.set_global_composite_operation(&"destination-in");
                    }
                } else {
                    oobb.draw_to_canvas(&ctx);
                    ctx.set_global_composite_operation(&"destination-in");
                }
                ctx.fill();
                ctx.restore();


                //make this faster by only getting image data around the aabb
                let mut cull = oobb.to_aabb();

                //Use the bottom of the aabb instead of the top for getting the image data
                //just setting invert_y won't work (bug?) - maybe look into that or add an invert()
                //method
                let data = ctx.get_image_data(cull.left(), cull.bottom(), cull.width, cull.height) 
                    .unwrap_ji()
                    .data()
                    .to_vec();


                let width = cull.width as usize;
                let height = cull.height as usize;

                //accumulate the number of hits for each target index
                let mut hits:HashMap<u32, usize> = HashMap::new(); 

                for x in 0..width {
                    for y in 0..height {
                        let offset = y * (width * 4) + x * 4;
                        let r = data[offset + 0] as u32;
                        let g = data[offset + 1] as u32;
                        let b = data[offset + 2] as u32;
                        let a = data[offset + 3];

                        //we use alpha just to check if there _is_ a hit
                        if a != 0 {
                            let index = ((r << 16) | (g << 8) | b);

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
                                if let Some((curr_index, curr_n_hits)) = acc {
                                    if n_hits > curr_n_hits {
                                        true
                                    } else {
                                        false
                                    }
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
                    canvas.set_style("left", &format!("{}px", resize_info.x + resize_info.content_x));
                    canvas.set_style("top", &format!("{}px", resize_info.y + resize_info.content_y));

                    web_sys::window()
                        .unwrap_ji()
                        .document()
                        .unwrap_ji()
                        .body()
                        .unwrap_ji()
                        .append_child(&canvas);
                }

                best_hit
            })
    }

    fn debug_render_hit(&self, index:usize) {
        let resize_info = get_resize_info();

        let canvas:HtmlCanvasElement = web_sys::window()
            .unwrap_ji()
            .document()
            .unwrap_ji()
            .create_element("canvas")
            .unwrap_ji()
            .unchecked_into();

        canvas.set_width(resize_info.width as u32);
        canvas.set_height(resize_info.height as u32);

        let ctx = get_2d_context(&canvas, None).unwrap_ji();

        if let Some(area) = self.game.base.target_areas.get(index) {
            let color = format!("red");
            let trace = &area.trace;

            components::traces::canvas::draw_trace(&ctx, &resize_info, &trace);
            ctx.set_fill_style(&JsValue::from_str(&color));
            ctx.fill();
        }

        //Just for testing - to see the canvas
        canvas.set_style("position", "fixed");
        //canvas.set_style("opacity", "0.5");
        canvas.set_style("pointer-events", "none");
        canvas.set_style("left", &format!("{}px", resize_info.x + resize_info.content_x));
        canvas.set_style("top", &format!("{}px", resize_info.y + resize_info.content_y));

        web_sys::window()
            .unwrap_ji()
            .document()
            .unwrap_ji()
            .body()
            .unwrap_ji()
            .append_child(&canvas);
    }

}

impl InteractiveItem {
    pub fn start_drag(&self, x: i32, y: i32) {
        self.drag.set(Some(Rc::new(Drag::new(x, y, 0.0, 0.0, true))));
    }

    pub fn try_move_drag(&self, x: i32, y: i32) {
        if let Some(drag) = self.drag.lock_ref().as_ref() {
            if let Some((_, diff)) = drag.update(x, y) {
                let resize_info = get_resize_info();
                let (diff_x, diff_y) = resize_info.get_px_normalized(diff.x as f64, diff.y as f64);

                self.curr_transform.replace_with(|t| {
                    let mut t = t.clone();
                    t.add_translation_2d(diff_x * -1.0, diff_y * -1.0);

                    t
                });
            }
        }
    }

    pub fn try_end_drag(&self, x: i32, y: i32) -> bool {
        if self.drag.lock_ref().is_some() {
            let drag = self.drag.lock_mut().take().unwrap_ji();
            //self.curr_offset.set((0.0, 0.0));
            true
        } else {
            false
        }
    }
}
