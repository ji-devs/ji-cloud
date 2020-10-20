use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::utils::templates;
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use core::{
    routes::{Route, AdminRoute},
    settings::SETTINGS
};
use shared::domain::{
    user::UserProfile,
    category::Category,
    image::ImageKind,
};

use super::data::*;
#[derive(Clone, Copy, Debug)]
enum Direction {
    Right,
    Left
}
pub struct Sidebar {
    pub jig: Mutable<Option<Jig>>
}

impl Sidebar {
    pub fn new(jig:Mutable<Option<Jig>>) -> Rc<Self> {
        let _self = Rc::new(Self {jig});

        _self
    }

    pub fn render(_self: Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        _self.jig.signal_ref(clone!(_self => move |jig| {
            jig.as_ref().map(|jig| {
                elem!(templates::edit_sidebar_section(), {
                    .with_data_id!("modules", {
                        .children(Self::render_modules(_self.clone(), &jig.modules))
                    })
                })
            })
        }))
    }

    fn render_modules(_self: Rc<Self>, modules: &[Option<Module>]) -> Vec<Dom> {
        modules
            .iter()
            .enumerate()
            .map(|(idx, id)| {
                Self::render_module(
                    _self.clone(),
                    idx,
                    if idx % 2 == 0 {
                        Direction::Left
                    } else {
                        Direction::Right
                    }, 
                    id.as_ref()
                )
            })
            .collect()
                
    }

    //TODO - render differently if there is an ID
    fn render_module(_self: Rc<Self>, index:usize, direction:Direction, module: Option<&Module>) -> Dom {
        let elem = {
            match direction {
                Direction::Right => templates::edit_module_right(),
                Direction::Left => templates::edit_module_left(),
            }
        };
        elem!(elem, {
            .with_data_id!("label", {
                .visible(module.is_none()) 
            })
            .with_data_id!("img", {
                .property("src", {

                    let media_url = unsafe {
                        SETTINGS.get_unchecked().remote_target.media_ui_url()
                    };
                    match module {
                        //TODO - get icon
                        Some(module) => "".to_string(),
                        None => format!("{}/jig-gear-wheel.svg", media_url)
                    }
                })
            })
            .event_preventable(|evt:events::DragOver| {
                if let Some(data_transfer) = evt.data_transfer() {
                    if let Ok(data) = data_transfer.get_data("text/plain") {
                        evt.prevent_default();
                    }
                }
            })
            .event(clone!(_self => move |evt:events::Drop| {
                if let Some(data_transfer) = evt.data_transfer() {
                    if let Ok(data) = data_transfer.get_data("text/plain") {
                        if let Some(kind) = ModuleKind::from_str(&data) {
                            let _self = _self.clone();
                            spawn_local(async move {
                                let module = create_default_module(kind).await;
                                if let Some(jig) = &mut *_self.jig.lock_mut() {
                                    jig.modules[index] = Some(module);
                                }
                            });
                        } else {
                            log::warn!("unsupported module type!");
                        }
                    }
                }
            }))
            
        })
    }
}

async fn create_default_module(kind:ModuleKind) -> Module {
    Module {
        id: "blah_blah".to_string(),
        kind
    }
}

