use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,map_mut,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
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
    pub id: Id,
    pub title: Mutable<String>,
    pub cover: Mutable<Option<Id>>,
    pub ending: Mutable<Option<Id>>,
    pub modules: Rc<MutableVec<Module>>,
    pub menu_index: Mutable<Option<usize>>, //which menu is currently open
    pub drag_index: Mutable<Option<usize>>, //target of current drag
    pub drop_index: Mutable<Option<usize>>, //target of current drag
}

impl Sidebar {
    pub fn new(jig:Jig) -> Rc<Self> {
        let _self = Rc::new(Self {
            id: jig.id,
            title: Mutable::new(jig.title),
            cover: Mutable::new(jig.cover),
            ending: Mutable::new(jig.ending),
            modules: Rc::new(MutableVec::new_with_values(jig.modules)),
            menu_index: Mutable::new(None),
            drag_index: Mutable::new(None),
            drop_index: Mutable::new(None),
        });

        _self
    }

    pub fn render(_self: Rc<Self>) -> Dom { 
        elem!(templates::edit_sidebar_section(), {
            .with_data_id!("title" => HtmlInputElement, {
                .with_node!(input => {
                    .event(clone!(_self => move |evt:events::Input| {
                        let value = input.value();

                        log::warn!("TODO - save new title: {}", value);
                    }))
                })
            })
            .with_data_id!("add-btn-first", {
                .event(clone!(_self => move |evt:events::Click| {
                    add_empty_module(_self.modules.clone(), 0);
                }))
            })
            .with_data_id!("menu-btn-first", {
                .event(clone!(_self => move |evt:events::Click| {
                    log::warn!("implement cover page menu thingie");
                }))
            })
            .with_data_id!("modules", {
                .children_signal_vec(
                    Self::renderable_modules(_self.clone())
                        .map(|module_dom| {
                            ModuleDom::render(module_dom.clone())
                        })
                )
            })
        })
    }

    //Converts the modules into the renderable list
    //
    //It does this
    //By removing the module at the drag_index
    //And setting the space at the drop_index to a None
    fn renderable_modules(_self: Rc<Self>) -> impl SignalVec<Item = Rc<ModuleDom>> {
        let drag_index = _self.drag_index.clone();
        let drop_index = _self.drop_index.clone();
        
        _self.modules
            .signal_vec_cloned()
            .enumerate()
            .filter_signal_cloned(move |(index, module)| {
                map_ref! (
                    let index = index.signal(),
                    let drag_index = drag_index.signal() =>
                    (index.clone(), drag_index.clone())
                )
                .map(|indices| {
                    match indices {
                        (None, _) => true,
                        (_, None) => true,
                        (Some(index), Some(drag_index)) => drag_index != index
                    }
                })
            })
            .map_signal(move |(index, module)| {
                index.signal().map(clone!(_self => move |index| {
                    let direction = {
                        if let Some(index) = index {
                            if index % 2 == 0 {
                                Direction::Left
                            } else {
                                Direction::Right
                            }
                        } else {
                            Direction::Right
                        }
                    };

                    Rc::new(ModuleDom { 
                        module: module.clone(), 
                        direction, 
                        index: index.unwrap_or(0),
                        sidebar: _self.clone(),
                    })
                }))
            })
    }
}

struct ModuleDom {
    pub module: Module,
    pub direction: Direction,
    pub index: usize,
    pub sidebar: Rc<Sidebar>
}

impl ModuleDom {
    fn render(_self: Rc<Self>) -> Dom {
        let elem = {
            match _self.direction {
                Direction::Right => templates::edit_module_right(),
                Direction::Left => templates::edit_module_left(),
            }
        };
        elem!(elem, {
            .with_data_id!("add-btn", {
                .event(clone!(_self => move |evt:events::Click| {
                    add_empty_module(_self.sidebar.modules.clone(), _self.index+1);
                }))
            })
            .with_data_id!("menu", {
                .child_signal(_self.sidebar.menu_index.signal_ref(clone!(_self => move |menu_index| {
                    menu_index.and_then(|menu_index| {
                        if menu_index == _self.index {
                            Some(MenuDom::render(Rc::new(MenuDom {
                                module_id: _self.module.id.clone(),
                                index: _self.index,
                                sidebar: _self.sidebar.clone()
                            })))
                        } else {
                            None
                        }
                    })
                })))
            })
            .with_data_id!("menu-btn", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.sidebar.menu_index.set(Some(_self.index));
                }))
            })
            .with_data_id!("label", {
                .visible(_self.module.kind.is_none()) 
            })
            .with_data_id!("img", {
                .property("src", {

                    let media_url = unsafe {
                        SETTINGS.get_unchecked().remote_target.media_ui_url()
                    };
                    let icon_path = match &_self.module.kind {
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
                    if data_transfer.types().index_of(&JsValue::from_str("module_kind"), 0) != -1 
                        && _self.module.kind.is_none() {
                        evt.prevent_default();
                    } 
                }
            }))
            .event(clone!(_self => move |evt:events::Drop| {
                if let Some(data_transfer) = evt.data_transfer() {
                    let module_kind:Option<ModuleKind> = 
                        data_transfer.get_data("module_kind")
                            .ok()
                            .and_then(|data| ModuleKind::from_str(&data));

                    if let Some(kind) = module_kind {
                        let _self = _self.clone();
                        spawn_local(async move {
                            let id = _self.module.id.clone();
                            let mut modules = _self.sidebar.modules.lock_mut();
                            modules.set_cloned(
                                _self.index, 
                                Module {
                                    id: id.clone(),
                                    kind: Some(kind)
                                }
                            );
                            spawn_local(async move {
                                module_service::change_kind(id, kind).await;
                            });
                        });
                    }
                }
            }))
        })
    }
}

struct MenuDom {
    pub module_id: Id,
    pub index: usize,
    pub sidebar: Rc<Sidebar>
}

impl MenuDom {
    fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::edit_menu_section(), {
            .with_data_id!("delete", {
                .event(clone!(_self => move |evt:events::Click| {
                    let module_id = _self.module_id.clone();
                    spawn_local(async move {
                        module_service::delete(module_id).await;
                    });

                    _self.sidebar.modules.lock_mut().remove(_self.index);
                    _self.sidebar.menu_index.set(None);
                }))
            })
            .global_event(clone!(_self => move |evt:events::Click| {
                if let Some(target) = evt.target() {
                    let element:Element = target.unchecked_into();
                    if !element.closest_data_id("menu").is_some() {
                        _self.sidebar.menu_index.set(None);
                    }
                }
            }))
        })
    }
}

//Actions
fn add_empty_module(modules:Rc<MutableVec<Module>>, before_index:usize) {
    spawn_local(async move {
        let module = module_service::add().await; 
        modules.lock_mut().insert_cloned(before_index, module);
        log::warn!("TODO - save new empty module!"); 
    });
}
