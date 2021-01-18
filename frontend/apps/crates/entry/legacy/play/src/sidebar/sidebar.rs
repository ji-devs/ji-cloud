use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, with_node, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, futures::AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use legacy::Manifest;
use crate::player::Player;
use awsm_web::loaders::fetch::fetch_url;
use crate::shell::Shell;
use super::module::Module;

pub struct Sidebar {
    closed: Mutable<bool>,
}

impl Sidebar {
    pub fn render(shell: Rc<Shell>) -> Dom {
        let _self = Rc::new(Self { 
            closed: Mutable::new(true),
        });


        html!("legacy-sidebar", {
            .event(clone!(_self => move |evt:utils::events::Close| {
                _self.closed.set(true);
            }))
            .event(clone!(_self => move |evt:utils::events::Open| {
                _self.closed.set(false);
            }))
            .property("slot", "sidebar")
            .property_signal("nModules", shell.modules_len())
            .property_signal("closed", _self.closed.signal())
            .children_signal_vec(Self::children(_self.clone(), shell.clone()))
        })
    }

    fn children(_self: Rc<Self>, shell:Rc<Shell>) -> impl SignalVec<Item = Dom> {
        shell.manifest.signal_ref(clone!(shell, _self => move |manifest| {
            manifest
                .as_ref()
                .map(|manifest| {
                    let elements:Vec<Dom> = 
                        manifest
                            .modules
                            .iter()
                            .enumerate()
                            .map(|(index, id)| {
                                Module::render(shell.clone(), _self.clone(), id.to_string(), index)
                            })
                            .collect();
                    elements
                })
                .unwrap_or(Vec::new())
        }))
        .to_signal_vec()
    }

}
