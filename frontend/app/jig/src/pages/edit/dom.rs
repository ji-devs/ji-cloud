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
use core::routes::{Route, AdminRoute};
use shared::domain::{
    user::UserProfile,
    category::Category,
    image::ImageKind,
};
use super::{data::*, module_selector::*, sidebar::*};

pub struct EditPage {
    pub right_section:Mutable<RightSection>,
    pub jig: Mutable<Option<Jig>>
}

#[derive(Clone, Copy, Debug)]
pub enum RightSection {
    ModuleSelect
}



impl EditPage {
    pub fn new(id:String) -> Rc<Self> {
        let _self = Rc::new(Self { 
            right_section: Mutable::new(RightSection::ModuleSelect),
            jig: Mutable::new(None)
        });

        let _self_clone = _self.clone();

        spawn_local(async move {
            let jig = Jig::load(id).await;
            _self.jig.set(Some(jig));
        });

        _self_clone
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {

        elem!(templates::edit_page(), {
            .with_data_id!("sidebar", {
                .child_signal(Sidebar::render(Sidebar::new(_self.jig.clone())))
            })
            .with_data_id!("right-area", {
                .child_signal(_self.right_section.signal_ref(|section| {
                    Some(
                        match section {
                            RightSection::ModuleSelect => {
                                ModuleSelect::render(ModuleSelect::new())
                            }
                        }
                    )
                }))
            })
        })
    }
}


