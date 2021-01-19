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
use crate::path::manifest_url;
use crate::shell::Shell;
use super::Sidebar;

pub struct Module {
}

impl Module {
    pub fn render(shell: Rc<Shell>, sidebar: Rc<Sidebar>, module_id: String, index: usize) -> Dom {
        html!("legacy-sidebar-module", {
            .event(clone!(shell, sidebar, module_id => move |evt:events::Click| {
                shell.selected_index.set(Some(index));
            }))
            .property("slot", format!("module-{}", index))
            .property("index", index as u32)
            .property("jigId", &shell.jig_id)
            .property("moduleId", &module_id)
            .property_signal("selected", selected_signal(shell.clone(), index))
            .child(html!("img-legacy", {
                .property("slot", "img") 
                .property("jigId", &shell.jig_id)
                .property("moduleId", &module_id)
                .property("path", "photoThumb1.jpg")
                .apply_if(shell.debug.is_mock, |dom| {
                    dom.property("mock", true)
                })
            }))
        })
    }
}

fn selected_signal(shell: Rc<Shell>, index:usize) -> impl Signal<Item = bool> {
    shell 
        .selected_index
        .signal_ref(clone!(index => move |sel_index| {
            match sel_index {
                None => false,
                Some(sel_index) => *sel_index == index
            }
        }))
}
    /*
                <legacy-sidebar-module 
                    slot="module-${index}" 
                    index="${index}"
                    jigId="web-stress-test"
                    moduleId="21f7c750-0623-11ea-beec-3f00dc406aac">

                    <img-legacy
                        slot="img" 
                        jigId="web-stress-test"
                        moduleId="21f7c750-0623-11ea-beec-3f00dc406aac"
                        path="photoThumb1.jpg"
                        mock>
                    </img-legacy>
                    */
