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
use shared::domain::{
    user::UserProfile,
    category::Category
};
use super::actions::{self, MetaOptions};
use std::collections::HashSet;

pub struct ImageEdit {
    id: Mutable<String>,
    error_message: Mutable<Option<String>>,
    refs:RefCell<Option<ImageEditRefs>>,
    meta_options: Mutable<Option<MetaOptions>>,
    styles: RefCell<HashSet<String>>,
    age_ranges: RefCell<HashSet<String>>,
    affiliations: RefCell<HashSet<String>>,
}

impl ImageEdit{
    pub fn new(id:String) -> Rc<Self> {
        let _self = Rc::new(Self { 
            id: Mutable::new(id),
            error_message: Mutable::new(None),
            refs: RefCell::new(None),
            meta_options: Mutable::new(None),
            styles: RefCell::new(HashSet::new()),
            age_ranges: RefCell::new(HashSet::new()),
            affiliations: RefCell::new(HashSet::new()),
        });

        let _self_clone = _self.clone();

        spawn_local(async move {
            match actions::MetaOptions::load().await {
                Ok(meta_options) => { _self_clone.meta_options.set(Some(meta_options)); } 
                Err(_) => { log::error!("GOT ERROR!!"); }
            }
        });
        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        html!("div", {
            .child_signal(_self.meta_options.signal_cloned().map(clone!(_self => move |meta_options| {
                meta_options.map(|meta_options| {
                    elem!(templates::image_edit(), { 
                        .with_data_id!("img", {
                            .property_signal("src", _self.id.signal_cloned().map(|id| {
                                upload_image_url(&id)
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
                                if let Some(refs) = _self.refs.borrow().as_ref() {
                                   
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

                                    log::info!("premium: {}, name: {}, description: {}, styles: {:?}, age_ranges: {:?}, affiliations: {:?}", 
                                        refs.is_premium(), 
                                        refs.name(), 
                                        refs.description(),
                                        styles,
                                        age_ranges,
                                        affiliations
                                    );
                                }
                            }))
                        })
                        .with_data_id!("styles", {
                            .children(meta_options.styles.into_iter().map(clone!(_self => move |(id, label)| {
                                elem!(templates::checkbox(&id, &label), {
                                    .event(clone!(_self => move |evt:events::Click| {
                                        if let Some(target) = evt.target() {
                                            let target:HtmlInputElement = target.unchecked_into();
                                            let mut styles = _self.styles.borrow_mut();
                                            if target.checked() {
                                                styles.insert(id.to_string());
                                            } else {
                                                styles.remove(&id);
                                            }
                                        }
                                    }))
                                })
                            })))
                        })
                        .with_data_id!("age_ranges", {
                            .children(meta_options.age_ranges.into_iter().map(clone!(_self => move |(id, label)| {
                                elem!(templates::checkbox(&id, &label), {
                                    .event(clone!(_self => move |evt:events::Click| {
                                        if let Some(target) = evt.target() {
                                            let target:HtmlInputElement = target.unchecked_into();
                                            let mut age_ranges = _self.age_ranges.borrow_mut();
                                            if target.checked() {
                                                age_ranges.insert(id.to_string());
                                            } else {
                                                age_ranges.remove(&id);
                                            }
                                        }
                                    }))
                                })
                            })))
                        })
                        .with_data_id!("affiliations", {
                            .children(meta_options.affiliations.into_iter().map(clone!(_self => move |(id, label)| {
                                elem!(templates::checkbox(&id, &label), {
                                    .event(clone!(_self => move |evt:events::Click| {
                                        if let Some(target) = evt.target() {
                                            let target:HtmlInputElement = target.unchecked_into();
                                            let mut affiliations = _self.affiliations.borrow_mut();
                                            if target.checked() {
                                                affiliations.insert(id.to_string());
                                            } else {
                                                affiliations.remove(&id);
                                            }
                                        }
                                    }))
                                })
                            })))
                        })

                        .after_inserted(clone!(_self => move |elem| {
                            *_self.refs.borrow_mut() = Some(ImageEditRefs::new(elem));
                        }))
                    })
                })
            })))
        })
    }
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
