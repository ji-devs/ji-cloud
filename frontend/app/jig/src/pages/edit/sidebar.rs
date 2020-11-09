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
use web_sys::{HtmlElement, Element, HtmlImageElement, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::utils::templates;
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use core::{
    routes::{Route, JigRoute, ModuleRoute, module_kind_from_str, module_kind_to_str, module_kind_to_label},
    settings::SETTINGS,

};
use shared::domain::{
    user::UserProfile,
    category::Category,
    image::ImageKind,
    jig::ModuleKind,
};
use super::scrolling::*;
use super::dragging::*;
use super::data::*;
#[derive(Clone, Copy, Debug)]
enum HorizontalDirection {
    Right,
    Left,
}

pub struct Sidebar {
    pub id: Id,
    pub title: Mutable<String>,
    pub cover: Mutable<Option<Id>>,
    pub ending: Mutable<Option<Id>>,
    pub modules: Rc<MutableVec<Module>>,
    pub menu_index: Mutable<Option<usize>>, //which menu is currently open
    pub drag_index: Mutable<Option<usize>>, //target of current drag
    pub scrolling: Scrolling,
    pub dragging: RefCell<Dragging>,
    pub element: RefCell<Option<Element>>,
    pub scrollable_elem: RefCell<Option<HtmlElement>>,
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
            scrolling: Scrolling::new(),
            dragging: RefCell::new(Dragging::new()),
            element: RefCell::new(None),
            scrollable_elem: RefCell::new(None),
        });

        _self
    }

    pub fn render(_self: Rc<Self>) -> Dom { 
        elem!(templates::edit_sidebar_section(), {
            .future(_self.drag_index.signal().for_each(clone!(_self => move |index| {
                if index.is_none() {
                    _self.scrolling.stop();
                }
                ready(())
            })))
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
                    add_empty_module(_self.clone(), 0);
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

            .with_data_id!("hover-module", {
                .visible_signal(_self.dragging.borrow().active_signal())
                .style_signal("top", _self.dragging.borrow().top_style_signal())
                .style_signal("left", _self.dragging.borrow().left_style_signal())
                .with_data_id!("hover-module-img", {
                    .property_signal("src", {
                        _self.dragging.borrow().module_signal().map(|module| {
                            match module{
                                Some(module) => module.kind.get_thumbnail(),
                                None => "".to_string()
                            }
                        })
                    })
                })
            })

            .after_inserted(clone!(_self => move |elem| {
                *_self.element.borrow_mut() = Some(elem.unchecked_into());
            }))

            .global_event(clone!(_self => move |evt:events::MouseMove| {
                _self.dragging.borrow_mut().on_move(evt.x(), evt.y());
                if _self.dragging.borrow().active() {
                    if let Some(elem) = _self.scrollable_elem.borrow().as_ref() {
                        let rect = elem.get_bounding_client_rect();
                        _self.scrolling.start(evt.y(), rect.y(), rect.height());
                    }
                    
                }
            }))

            .global_event(clone!(_self => move |evt:events::MouseUp| {
                if let Some((src_index, dest_index)) = _self.dragging.borrow_mut().stop_drag() {
                    move_module_index(_self.clone(), src_index, dest_index);
                }

                _self.scrolling.stop();

            }))

            .with_data_id!("scrollable-area", {
                .scroll_top_signal(_self.scrolling.scroll_top_signal())
                .after_inserted(clone!(_self => move |elem| {
                    *_self.scrollable_elem.borrow_mut() = Some(elem);
                }))
            })

        })
    }

    //Converts the modules into the renderable list
    //
    //It does this
    //By removing the module at the src_index
    //Setting the space at dest_index
    //
    fn renderable_modules(_self: Rc<Self>) -> impl SignalVec<Item = Rc<ModuleDom>> {
        
        _self.modules
            .signal_vec_cloned()
            .enumerate()
            .map_signal(clone!(_self => move |(index, module)| {
                index.signal().map(clone!(_self => move |index| {
                    let direction = {
                        if let Some(index) = index {
                            if index % 2 == 0 {
                                HorizontalDirection::Left
                            } else {
                                HorizontalDirection::Right
                            }
                        } else {
                            HorizontalDirection::Right
                        }
                    };
                    Rc::new(ModuleDom { 
                            module: module.clone(), 
                            direction, 
                            index: index.unwrap_or(0),
                            sidebar: _self.clone(),
                            img_size: RefCell::new(None)
                    })
                }))
            }))
    }
}

struct ModuleDom {
    pub module: Module,
    pub direction: HorizontalDirection,
    pub index: usize,
    pub sidebar: Rc<Sidebar>,
    pub img_size: RefCell<Option<(f64, f64)>>,
}

impl ModuleDom {

    fn render(_self:Rc<Self>) -> Dom {
        let dragging = &mut *_self.sidebar.dragging.borrow_mut();
        let elem = {
            match _self.direction {
                HorizontalDirection::Right => templates::edit_module_right(),
                HorizontalDirection::Left => templates::edit_module_left(),
            }
        };
        elem!(elem, {
            .with_node!(div => {
                .with_data_id!("title", {
                    .text(&format!("{}", _self.index+1))
                })
                .with_data_id!("subtitle", {
                    .text(match _self.module.kind {
                        None => "",
                        Some(kind) => module_kind_to_label(kind)
                    })
                })
                .with_data_id!("add-btn", {
                    .event(clone!(_self => move |evt:events::Click| {
                        add_empty_module(_self.sidebar.clone(), _self.index+1);
                    }))
                })
                .with_data_id!("menu", {
                    .child_signal(_self.sidebar.menu_index.signal_ref(clone!(_self => move |menu_index| {
                        menu_index.and_then(|menu_index| {
                            if menu_index == _self.index {
                                Some(MenuDom::render(Rc::new(MenuDom {
                                    module_id: _self.module.id.clone(),
                                    module_kind: _self.module.kind.clone(),
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
                .with_data_id!("img" => HtmlImageElement, {
                    .with_node!(elem => {
                        .property_signal("src", clone!(_self => 
                            dragging
                                .kind_at(_self.index)
                                .map(clone!(_self => move |kind| {
                                    match kind {
                                        Some(kind) => kind.get_thumbnail(),
                                        None => _self.module.kind.get_thumbnail()
                                    }
                                }))
                        ))

                        //TODO - change to Load Event
                        .after_inserted(clone!(_self => move |_| {
                            *_self.img_size.borrow_mut() = Some((101.0, 101.0));
                        }))
                    })
                })
                .event(clone!(_self => move |evt:events::MouseDown| {
                    match (_self.img_size.borrow().as_ref(), _self.sidebar.element.borrow().as_ref()) {
                        (Some(img_size), Some(parent_elem)) => {
                            let modules:Vec<Element> = parent_elem.select_vec(&data_id("module-container"));
                            let module_kinds:Vec<Module> = _self.sidebar.modules.lock_ref().to_vec();
                            let module_kinds:Vec<Option<ModuleKind>> = module_kinds.into_iter().map(|m| m.kind).collect(); 

                            _self.sidebar.dragging.borrow().start_drag(
                                _self.index, 
                                evt.x(), evt.y(), 
                                img_size.0, img_size.1,
                                _self.module.clone(), 
                                modules,
                                module_kinds,
                            );
                        }
                        _ => {}
                    }
                }))
                .event_preventable(clone!(_self => move |evt:events::DragOver| {
                    if let Some(data_transfer) = evt.data_transfer() {
                        if data_transfer.types().index_of(&JsValue::from_str("module_kind"), 0) != -1 {
                            if _self.module.kind.is_none() {
                                evt.prevent_default();
                            } 
                        }                     }

                }))


                .event(clone!(_self => move |evt:events::Drop| {
                    _self.sidebar.drag_index.set(None);

                    if let Some(data_transfer) = evt.data_transfer() {
                        if let Some(module_kind) = data_transfer.get_data("module_kind").ok() { 
                            let kind = module_kind_from_str(&module_kind).unwrap_throw();
                            assign_module_kind(_self.sidebar.clone(), _self.index, _self.module.id.clone(), kind);
                            
                        }
                    }
                }))
            })
        })
    }
}



struct MenuDom {
    pub module_id: Id,
    pub module_kind: Option<ModuleKind>,
    pub index: usize,
    pub sidebar: Rc<Sidebar>
}

impl MenuDom {
    fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::edit_menu_section(), {
            .with_data_id!("edit", {
                .apply_if(_self.module_kind.is_none(), |dom| dom.class("hidden"))
                .event(clone!(_self => move |evt:events::Click| {
                    let module_id = _self.module_id.clone();
                    let module_kind = _self.module_kind.clone().unwrap_throw();
                    let route:String = Route::Module(ModuleRoute::Edit(module_kind, module_id)).into();
                    dominator::routing::go_to_url(&route);
                }))
            })
            .with_data_id!("duplicate", {
                .apply_if(_self.module_kind.is_none(), |dom| dom.class("hidden"))
                .event(clone!(_self => move |evt:events::Click| {
                    duplicate_module(_self.sidebar.clone(), _self.index, _self.module_id.clone());
                    _self.sidebar.menu_index.set(None);
                }))
            })
            .with_data_id!("copy", {
                .apply_if(_self.module_kind.is_none(), |dom| dom.class("hidden"))
                .event(clone!(_self => move |evt:events::Click| {
                    log::warn!("TODO - copy jig and module ID to clipboard for later pasting");
                    _self.sidebar.menu_index.set(None);
                }))
            })
            .with_data_id!("delete", {
                .event(clone!(_self => move |evt:events::Click| {
                    delete_module(_self.sidebar.clone(), _self.index, _self.module_id.clone());
                    _self.sidebar.menu_index.set(None);
                }))
            })
            .with_data_id!("move-up", {
                .apply_if(_self.index == 0, |dom| dom.class("hidden"))
                .event(clone!(_self => move |evt:events::Click| {
                    move_module_index(_self.sidebar.clone(), _self.index, _self.index-1);
                    _self.sidebar.menu_index.set(None);
                }))
            })
            .with_data_id!("move-down", {
                .apply_if(_self.index == _self.sidebar.modules.lock_ref().len()-1, |dom| dom.class("hidden"))
                .event(clone!(_self => move |evt:events::Click| {
                    move_module_index(_self.sidebar.clone(), _self.index, _self.index+1);
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
fn add_empty_module(sidebar:Rc<Sidebar>, before_index:usize) {
    spawn_local(async move {
        log::warn!("TODO - BACKEND - add new empty module!"); 
        let module = Module {
            id: "blah_blah".to_string(),
            kind: None, 
        };
        sidebar.modules.lock_mut().insert_cloned(before_index, module);
    });
}
fn assign_module_kind(sidebar:Rc<Sidebar>, index:usize, id:Id, kind:ModuleKind) {
    spawn_local(async move {

        log::warn!("TODO - BACKEND - assign module kind!"); 
        let mut modules = sidebar.modules.lock_mut();
        modules.set_cloned(
            index, 
            Module {
                id: id.clone(),
                kind: Some(kind)
            }
        );
    });
}

fn duplicate_module(sidebar:Rc<Sidebar>, index:usize, id:Id) {
    spawn_local(async move {

        log::warn!("TODO - BACKEND - duplicate module!"); 
        let module = Module {
            id: "blah_blah".to_string(),
            kind: Some(ModuleKind::Poster), 
        };
        sidebar.modules.lock_mut().insert_cloned(index+1, module);
    });
}
fn delete_module(sidebar:Rc<Sidebar>, index:usize, id:Id) {

    log::warn!("TODO - BACKEND - delete module!"); 
    sidebar.modules.lock_mut().remove(index);
    sidebar.menu_index.set(None);
}

fn move_module_index(sidebar:Rc<Sidebar>, src_index: usize, dest_index:usize) {

    log::warn!("TODO - BACKEND - swap module index!"); 
    let mut modules = sidebar.modules.lock_mut();
    modules.move_from_to(src_index, dest_index);
}
