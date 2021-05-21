use dominator::{html, Dom, clone, svg, class};
use std::rc::Rc;
use utils::{prelude::*, resize::resize_info_signal};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
    signal_vec::SignalVecExt,
};
use super::state::*;
use shared::domain::jig::module::body::{TraceShape, Sticker as RawSticker};
use web_sys::HtmlCanvasElement;
use awsm_web::canvas::get_2d_context;
use once_cell::sync::Lazy;

static SVG_CLASS: Lazy<String> = Lazy::new(|| class! {
    .style("position", "absolute")
    .style("top", "0")
});
static BG_CLASS: Lazy<String> = Lazy::new(|| class! {
    .style("fill", "black")
    .style("fill-opacity", "0.5")
});

static FILL_CLASS: Lazy<String> = Lazy::new(|| class! {
    .style("fill", "white")
});
pub fn render(state:Rc<Edit>) -> Dom {
    let trace_signal = map_ref! {
        let resize_info = resize_info_signal(),
        let shape = state.trace.shape.signal_cloned(),
        let transform = state.trace.transform.get_inner_signal_cloned()
            => {
                (resize_info.clone(), shape.clone(), transform.clone())
            }
    };

    svg!("svg", {
        .class(&*SVG_CLASS)
        .attribute_signal("width", resize_info_signal().map(|info| {
            format!("{}px", info.width)
        }))
        .attribute_signal("height", resize_info_signal().map(|info| {
            format!("{}px", info.height)
        }))
        .child(svg!("rect", {
            .attribute("x", "0")
            .attribute("y", "0")
            .attribute_signal("width", resize_info_signal().map(|info| {
                format!("{}px", info.width)
            }))
            .attribute_signal("height", resize_info_signal().map(|info| {
                format!("{}px", info.height)
            }))
            .class(&*BG_CLASS)
            .event(clone!(state => move |evt:events::MouseDown| {
                state.start_draw(evt.x() as i32, evt.y() as i32);
            }))
        }))

        .child_signal(trace_signal.map(|(resize_info, shape, transform)| {
            match shape {
                TraceShape::Path(path) => {
                    Some(svg!("path", {
                        .class(&*FILL_CLASS)
                        .attribute("d", &path_to_string(
                            &path
                                .iter()
                                .map(|(x, y)| {
                                    resize_info.get_pos_denormalized(*x, *y)
                                })
                                .collect::<Vec<(f64, f64)>>()
                        ))
                    }))
                }
                _ => None
            }
        }))
        .global_event_preventable(clone!(state => move |evt:events::MouseUp| {
            state.end_draw(evt.x() as i32, evt.y() as i32);
        }))
        .global_event_preventable(clone!(state => move |evt:events::MouseMove| {
            state.move_draw(evt.x() as i32, evt.y() as i32);
        }))
    })
}

fn path_to_string(path:&[(f64, f64)]) -> String {
    let mut output = String::new();

    if let Some((x, y)) = path.first() {
        output.push_str(&format!("M{} {}", x, y));
        for (x, y) in &path[1..] {
            output.push_str(&format!(" L{} {}", x, y));
        }
    } else {
        output.push_str("M0 0");
    }

    output
}
