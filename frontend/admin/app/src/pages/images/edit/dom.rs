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
use super::actions::{self, Init, Id, EditCategory, EditCategoryMode};
use std::collections::{HashSet, HashMap};

pub struct ImageEdit {
    id: Mutable<String>,
    error_message: Mutable<Option<String>>,
    refs:RefCell<Option<ImageEditRefs>>,
    styles: RefCell<HashSet<String>>,
    age_ranges: RefCell<HashSet<String>>,
    affiliations: RefCell<HashSet<String>>,
    init: Mutable<Option<Init>>,
    section: Mutable<Section>,
    save_loader: AsyncLoader,
    category_expansions: RefCell<HashMap<Id, Mutable<bool>>>,
    selected_categories: Mutable<HashSet<Id>> 
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
            section: Mutable::new(Section::Categories),
            category_expansions: RefCell::new(HashMap::new()),
            selected_categories: Mutable::new(HashSet::new()) 
        });

        let _self_clone = _self.clone();

        spawn_local(async move {
            match actions::Init::load(&id).await {
                Ok(init) => {
                    fn set_expansions(curr:&Vec<EditCategory>, expansions: &mut HashMap<Id, Mutable<bool>>) {
                        for cat in curr.iter() {
                            expansions.insert(cat.id.clone(), Mutable::new(false));
                            if !cat.children.is_empty() {
                                set_expansions(&cat.children, expansions);
                            }
                        }
                    };
                    let mut expansions = _self_clone.category_expansions.borrow_mut();
                    set_expansions(&init.categories, &mut expansions);
                    let mut selected_categories = _self_clone.selected_categories.lock_mut();
                    for id in init.selected_categories.iter() {
                        selected_categories.insert(id.to_string());
                    }
                    _self_clone.init.set(Some(init)); 
                },
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

            .with_data_id!("affiliations", {
                .children(
                    init.affiliations
                        .iter()
                        .map(|(id, label, contains)| {
                            elem!(templates::checkbox(&id, &label), {
                                .with_data_id!(id, {
                                    .property("checked", *contains)
                                    .event(clone!(_self, id => move |evt:events::Change| {
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

        })
    }
    fn render_section_categories(_self: Rc<Self>, init:&Init) -> Dom {
        elem!(templates::image_edit_categories(), {
            .with_data_id!("select-list", {
                .children(init.categories.iter().map(clone!(_self => move |cat| {
                    Self::render_category_select(_self.clone(), cat.clone())
                })))
            })
            .with_data_id!("summary-list", {
                .children(init.categories.iter().map(clone!(_self => move |cat| {
                    Self::render_category_summary(_self.clone(), cat.clone())
                })))
            })
        })
    }

    fn render_category_summary(_self: Rc<Self>, cat: EditCategory) -> Dom {
        let id = cat.id.clone();

        let elem = match cat.mode {
            EditCategoryMode::Parent => templates::image_edit_category_summary_parent(&cat.name),
            EditCategoryMode::Child => templates::image_edit_category_summary_child(&cat.name)
        };

        elem!(elem, {
            .with_data_id!("children", {
                .children(cat.children.iter().map(clone!(_self => move |cat| {
                    Self::render_category_summary(_self.clone(), cat.clone())
                })))
            })
            .class_signal("hidden", _self.selected_categories.signal_ref(move |selected| {
                !cat.contains_leaf_set(selected)
            }))
        })
    }

    //TODO - make more DRY!... it's not so much parent vs child, more like end vs. not end
    fn render_category_select(_self: Rc<Self>, cat: EditCategory) -> Dom {
        let id = cat.id.clone();

        let elem = match cat.mode {
            EditCategoryMode::Parent => {
                if cat.is_end {
                    templates::image_edit_category_parent_end(&cat.name)
                } else {
                    templates::image_edit_category_parent(&cat.name)
                }
            }
            EditCategoryMode::Child => {
                if cat.is_end {
                    templates::image_edit_category_child_end(&cat.name)
                } else {
                    templates::image_edit_category_child(&cat.name)
                }
            }
        };

        if cat.is_end {
            elem!(elem, {
                .with_data_id!("checkbox", {
                    .property("checked", cat.assigned)
                    .event(clone!(_self, id => move |evt:events::Change| {
                        if let Some(checked) = evt.checked() {
                            {
                                let mut selected = _self.selected_categories.lock_mut();
                                if checked {
                                    selected.insert(id.to_string());
                                } else {
                                    selected.remove(&id);
                                }
                            }
                            Self::save(_self.clone());
                        }
                    }))
                })
            })
        } else {
            elem!(elem, {
                .with_data_id!("children", {
                    .children_signal_vec(
                        _self.category_expansions.borrow().get(&cat.id).unwrap_throw().signal().map(clone!(_self => move |expanded| {
                                if expanded {
                                    let children:Vec<Dom> = cat.children.iter().map(clone!(_self => move |cat| {
                                        Self::render_category_select(_self.clone(), cat.clone())
                                    })).collect();
                                    children
                                } else {
                                    Vec::new()
                                }
                        }))
                        .to_signal_vec()
                    )
                })
                .with_data_id!("arrow", {
                    .event(clone!(_self, id => move |evt:events::Click| {
                        _self.category_expansions.borrow()
                            .get(&id)
                            .unwrap_throw()
                            .replace_with(|x| !*x);

                    }))
                })
            })
        }
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

