use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, with_node, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, futures::AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::shell::Shell;
use awsm_web::loaders::fetch::fetch_url;
use legacy::*;
use utils::{resize::*, math::transform_2d};

pub struct StickerImage {
}

impl StickerImage {
    pub fn render(jig_id: &str, module_id: &str, img: &Image, mock: bool) -> Dom {

        html!("div", {
            .style("position", "absolute")
            .style("top", "0px") 
            .style("left", "0px") 
            .style("width", &format!("{}px", img.width))
            .style("height", &format!("{}px", img.height))
            .style("transform-origin", "center") 
            .style_signal("transform", transform_signal(img.transform, img.width, img.height)) 
            .child(html!("img-legacy", {
                .property("jigId", jig_id)
                .property("moduleId", module_id)
                .property("path", &format!("layers/{}", img.src))
                .property("mock", mock)
            }))
        })
    }
}

//backend Ji -> TT example: https://github.com/ji-devs/jitap-backend/blob/master/create/src/endpoints/playerMeta/PlayerMeta-Common.ts
//see https://jitap.net/media/webplayer/player.js?v=1.3.6
//previous player example just used our format so not viable as a reference
fn transform_signal(src:[f64;6], img_width: f64, img_height: f64) -> impl Signal<Item = String> {
    //TODO - adapt transform based on resize signal
    get_resize_info()
        .signal_ref(move |info| {
            let mut transform = [ 1.0, 0.0, 0.0, 1.0, 0.0, 0.0 ];
            let stage_width = info.width;
            let stage_height = info.height;
            let scale = info.scale;

            //Move origin to center of stage
            transform_2d::translate_mut(&mut transform, 
                (stage_width - img_width)/2.0,
                (stage_height - img_height)/2.0,
            );
            //Scale to stage size
            transform_2d::scale_mut(&mut transform, scale, scale);

            //Apply saved transform (which assumes coordinates in stage center)
            transform_2d::mul_mut(&mut transform, &src);

            format!("matrix({}, {}, {}, {}, {}, {})",
                transform[0],
                transform[1],
                transform[2],
                transform[3],
                transform[4],
                transform[5],
            )
        })
}
