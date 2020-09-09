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
    routes::{Route, UserRoute},
    path::upload_image_url
};
use shared::error::image::UpdateError;
use shared::domain::{
    user::UserProfile,
    category::Category,
};
use super::actions::{self, Init, Id};
use std::collections::HashSet;

pub struct ImageEdit {
    id: Mutable<String>,
    error_message: Mutable<Option<String>>,
    refs:RefCell<Option<ImageEditRefs>>,
    styles: RefCell<HashSet<String>>,
    age_ranges: RefCell<HashSet<String>>,
    affiliations: RefCell<HashSet<String>>,
    init: Mutable<Option<Init>>,
    section: Mutable<Section>,
    save_loader: AsyncLoader
}

#[derive(Clone, Copy, Debug)]
enum Section {
    Meta,
    Categories
}


impl ImageEdit{
    pub fn new(id:String) -> Rc<Self> {
        let _self = Rc::new(Self { 
            id: Mutable::new(id.clone()),
            error_message: Mutable::new(None),
            refs: RefCell::new(None),
            styles: RefCell::new(HashSet::new()),
            age_ranges: RefCell::new(HashSet::new()),
            affiliations: RefCell::new(HashSet::new()),
            init: Mutable::new(None),
            save_loader: AsyncLoader::new(),
            section: Mutable::new(Section::Meta),
        });

        let _self_clone = _self.clone();

        spawn_local(async move {
            match actions::Init::load(&id).await {
                Ok(init) => { _self_clone.init.set(Some(init)); } 
                Err(_) => { log::error!("GOT ERROR!!"); }
            }
        });
        _self
    }


    fn save(_self: Rc<Self>) {
        let _self_clone = _self.clone();
        if let Some(refs) = _self.refs.borrow().as_ref() {
            
            let id = _self.id.get_cloned();
            let is_premium = refs.is_premium();
            let name = refs.name();
            let description = refs.description();

            let styles:Vec<String> = 
                _self.styles
                    .borrow()
                    .iter()
                    .map(|s| s.to_string())
                    .collect();

            let age_ranges:Vec<String> =
                _self.age_ranges
                    .borrow()
                    .iter()
                    .map(|s| s.to_string())
                    .collect();

            let affiliations:Vec<String> =
                _self.affiliations
                    .borrow()
                    .iter()
                    .map(|s| s.to_string())
                    .collect();

            _self.save_loader.load(async move {
                _self_clone.error_message.set(None);

                if let Err(err) = actions::save(id, is_premium, name, description, styles, age_ranges, affiliations).await {
                    let msg = match err {
                        UpdateError::MissingMetadata{id, kind} => {
                            format!("missing metadata!")
                        },
                        UpdateError::MissingCategory(cat) => {
                            format!("missing category!")
                        },
                        _ => {
                            format!("internal error!")
                        }
                    };

                    _self_clone.error_message.set(Some(msg));
                }
            });

        }
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        html!("div", {
            //todo - refactor so that all inputs are mapped to mutable fields (live save)
            .child_signal(_self.init.signal_cloned().map(clone!(_self => move |init| {
                init.map(|init:Init| {
                    elem!(templates::image_edit(), { 
                        .with_data_id!("img", {
                            .property_signal("src", _self.id.signal_cloned().map_future(|id| {
                                async move {
                                    let url = actions::get_image_url(&id).await.unwrap_throw();
                                    url
                                }
                            }))
                        })
                        .with_data_id!("error", {
                            .class_signal("hidden", _self.error_message.signal_ref(|err| err.is_none()))
                            .with_data_id!("message", {
                                .text_signal(_self.error_message.signal_ref(|msg| {
                                    match msg {
                                        None => "".to_string(),
                                        Some(msg) => msg.to_string()
                                    }
                                }))
                            })
                        })
                        .with_data_id!("next", {
                            .event(clone!(_self => move |_evt:events::Click| {
                                let section = { *_self.section.lock_ref() };
                                match section {
                                    Section::Meta => {
                                        _self.section.set(Section::Categories);
                                    },
                                    _ => {
                                        log::info!("GO NEXT");
                                    }
                                }
                            }))
                        })
                        .with_data_id!("premium", {
                            .event(clone!(_self => move |_evt:events::Change| {
                                Self::save(_self.clone());
                            }))
                        })
                        .with_data_id!("name", {
                            .event(clone!(_self => move |_evt:events::Input| {
                                Self::save(_self.clone());
                            }))
                        })
                        .with_data_id!("description", {
                            .event(clone!(_self => move |_evt:events::Input| {
                                Self::save(_self.clone());
                            }))
                        })
                        .after_inserted(clone!(_self => move |elem| {
                            *_self.refs.borrow_mut() = Some(ImageEditRefs::new(elem));
                        }))
                        .with_data_id!("right-area", {
                            .child_signal(_self.section.signal_ref(clone!(_self => move |section| {
                                Some(match section {
                                    Section::Meta => Self::render_section_meta(_self.clone(), &init),
                                    Section::Categories => Self::render_section_categories(_self.clone(), &init),
                                })
                            })))
                        })

                    })
                })
            })))
        })
    }
    
    fn render_section_meta(_self: Rc<Self>, init:&Init) -> Dom {
        elem!(templates::image_edit_meta(), {
            .with_data_id!("styles", {
                .children(
                    init.styles
                        .iter()
                        .map(|(id, label, contains)| {
                            elem!(templates::checkbox(&id, &label), {
                                .with_data_id!(id, {
                                    .property("checked", *contains)
                                    .event(clone!(_self, id => move |evt:events::Change| {
                                        if let Some(checked) = evt.checked() {
                                            {
                                                let mut styles = _self.styles.borrow_mut();
                                                if checked {
                                                    styles.insert(id.to_string());
                                                } else {
                                                    styles.remove(&id);
                                                }
                                            }
                                            Self::save(_self.clone());
                                        }
                                    }))
                                })
                            })
                        })
                )
            })

            .with_data_id!("age_ranges", {
                .children(
                    init.age_ranges
                        .iter()
                        .map(|(id, label, contains)| {
                            elem!(templates::checkbox(&id, &label), {
                                .with_data_id!(id, {
                                    .property("checked", *contains)
                                    .event(clone!(_self, id => move |evt:events::Change| {
                                        if let Some(checked) = evt.checked() {
                                            {
                                                let mut age_ranges = _self.age_ranges.borrow_mut();
                                                if checked {
                                                    age_ranges.insert(id.to_string());
                                                } else {
                                                    age_ranges.remove(&id);
                                                }
                                            }
                                            Self::save(_self.clone());
                                        }
                                    }))
                                })
                            })
                        })
                )
            })

            /*
            .with_data_id!("age_ranges", {
                .children(
                    init.age_ranges
                        .into_iter()
                        .map(|(id, label, contains)| {
                            elem!(templates::checkbox(&id, &label), {
                                .with_data_id!(id, {
                                    .property("checked", contains)
                                    .event(clone!(_self => move |evt:events::Change| {
                                        if let Some(checked) = evt.checked() {
                                            {
                                                let mut age_ranges = _self.age_ranges.borrow_mut();
                                                if checked {
                                                    age_ranges.insert(id.to_string());
                                                } else {
                                                    age_ranges.remove(&id);
                                                }
                                            }
                                            Self::save(_self.clone());
                                        }
                                    }))
                                })
                            })
                        })
                )
            })

            .with_data_id!("affiliations", {
                .children(
                    init.affiliations
                        .into_iter()
                        .map(|(id, label, contains)| {
                            elem!(templates::checkbox(&id, &label), {
                                .with_data_id!(id, {
                                    .property("checked", contains)
                                    .event(clone!(_self => move |evt:events::Change| {
                                        if let Some(checked) = evt.checked() {
                                            {
                                                let mut affiliations = _self.affiliations.borrow_mut();
                                                if checked {
                                                    affiliations.insert(id.to_string());
                                                } else {
                                                    affiliations.remove(&id);
                                                }
                                            }
                                            Self::save(_self.clone());
                                        }
                                    }))
                                })
                            })
                        })
                )
            })
            */
        })
    }
    fn render_section_categories(_self: Rc<Self>, init:&Init) -> Dom {
        elem!(templates::image_edit_categories(), {})
    }
}

fn is_checked(possible:&[(Id, String)], item_list:&[Id], id:&Id) -> bool {
    true
}
struct ImageEditRefs {
    is_premium_elem: HtmlInputElement,
    name_elem: HtmlInputElement,
    description_elem: HtmlInputElement
}

impl ImageEditRefs {
    fn new(elem:HtmlElement) -> Self {
        Self {
            is_premium_elem: elem.select(&data_id("premium")),
            name_elem: elem.select(&data_id("name")),
            description_elem: elem.select(&data_id("description")),
        }
    }

    pub fn is_premium(&self) -> bool {
        self.is_premium_elem.checked()
    }

    pub fn name(&self) -> String {
        self.name_elem.value()
    }

    pub fn description(&self) -> String {
        self.description_elem.value()
    }

    /*
    pub fn styles(&self) -> String {
        self.description_elem.value()
    }
    */
}
