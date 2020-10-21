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
    pub jig: Mutable<Option<Jig>>,
    pub menu_index: Mutable<Option<usize>>,
}

impl Sidebar {
    pub fn new(jig:Mutable<Option<Jig>>) -> Rc<Self> {
        let _self = Rc::new(Self {
            jig,
            menu_index: Mutable::new(None)
        });

        _self
    }

    pub fn render(_self: Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        _self.jig.signal_ref(clone!(_self => move |jig| {
            jig.as_ref().map(|jig| {
                elem!(templates::edit_sidebar_section(), {
                    .with_data_id!("add-btn-first", {
                        .event(clone!(_self => move |evt:events::Click| {
                            Self::add_empty_module(_self.clone(), 0);
                        }))
                    })
                    .with_data_id!("menu-btn-first", {
                        .event(clone!(_self => move |evt:events::Click| {
                            log::warn!("implement cover page menu thingie");
                        }))
                    })
                    .with_data_id!("modules", {
                        .children(Self::render_modules(_self.clone(), &jig.modules))
                    })
                })
            })
        }))
    }

    fn add_empty_module(_self: Rc<Self>, before_index:usize) {
        spawn_local(async move {
            let module = Module::load_new().await;
            if let Some(jig) = &mut *_self.jig.lock_mut() {
                //TODO - sync with backend!
                jig.modules.insert(before_index, module);
            }
        });
    }

    fn render_modules(_self: Rc<Self>, modules: &[Module]) -> Vec<Dom> {
        modules
            .iter()
            .enumerate()
            .map(|(idx, module)| {
                Self::render_module(
                    _self.clone(),
                    idx,
                    if idx % 2 == 0 {
                        Direction::Left
                    } else {
                        Direction::Right
                    }, 
                    module
                )
            })
            .collect()
    }

    //TODO - render differently if there is an ID
    fn render_module(_self: Rc<Self>, index:usize, direction:Direction, module: &Module) -> Dom {
        let module_id = module.id.to_string();

        let elem = {
            match direction {
                Direction::Right => templates::edit_module_right(),
                Direction::Left => templates::edit_module_left(),
            }
        };
        elem!(elem, {
            .with_data_id!("label", {
                .visible(module.kind.is_none()) 
            })

            .with_data_id!("add-btn", {
                .event(clone!(_self => move |evt:events::Click| {
                    Self::add_empty_module(_self.clone(), index+1);
                }))
            })
            .with_data_id!("menu", {
                .child_signal(_self.menu_index.signal_ref(clone!(_self, module_id => move |menu_index| {
                    match menu_index {
                        None => None,
                        Some(menu_index) => {
                            if *menu_index == index {
                                let el = elem!(templates::edit_menu_section(), {
                                    .with_data_id!("delete", {
                                        .event(clone!(_self, module_id => move |evt:events::Click| {
                                            let module_id = module_id.clone();
                                            if let Some(jig) = &mut *_self.jig.lock_mut() {
                                                spawn_local(async move {
                                                    delete_module(module_id).await;
                                                });

                                                jig.modules.remove(index);
                                            }
                                            _self.menu_index.set(None);
                                        }))
                                    })
                                    .global_event(clone!(_self => move |evt:events::Click| {
                                        if let Some(target) = evt.target() {
                                            let element:Element = target.unchecked_into();
                                            if !element.closest_data_id("menu").is_some() {
                                                _self.menu_index.set(None);
                                            }
                                        }
                                    }))
                                });
                                Some(el)
                            } else {
                                None
                            }
                        }
                    }
                })))
            })
            .with_data_id!("menu-btn", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.menu_index.set(Some(index));
                }))
            })
            .with_data_id!("img", {
                .property("src", {

                    let media_url = unsafe {
                        SETTINGS.get_unchecked().remote_target.media_ui_url()
                    };
                    let icon_path = match &module.kind {
                        //TODO - get icon
                        Some(kind) => {
                            match kind {
                                ModuleKind::Poster => "icn-module-poster2.png",
                                ModuleKind::MemoryGame => "module-memory-game.svg",
                                _ => {
                                    panic!("don't have the icon for that module kind!");
                                }
                            }
                        },
                        None => "jig-gear-wheel.svg"
                    };

                    format!("{}/{}", media_url, icon_path)
                })
            })
            .event_preventable(clone!(_self => move |evt:events::DragOver| {
                if let Some(data_transfer) = evt.data_transfer() {
                    if let Ok(data) = data_transfer.get_data("text/plain") {
                        if let Some(jig) = &*_self.jig.lock_ref() {
                            if jig.modules[index].kind.is_none() { 
                                evt.prevent_default();
                            }
                        }
                    }
                }
            }))
            .event(clone!(_self => move |evt:events::Drop| {
                if let Some(data_transfer) = evt.data_transfer() {
                    if let Ok(data) = data_transfer.get_data("text/plain") {
                        if let Some(kind) = ModuleKind::from_str(&data) {
                            let _self = _self.clone();
                            spawn_local(async move {
                                if let Some(jig) = &mut *_self.jig.lock_mut() {
                                    let mut module = &mut jig.modules[index];
                                    module.change_kind(kind).await;
                                }
                            });
                        } else {
                            log::warn!("unsupported module type {}!", data);
                        }
                    }
                }
            }))
            
        })
    }
}

