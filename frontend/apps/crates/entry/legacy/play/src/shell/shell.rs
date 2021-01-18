use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, with_node, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, futures::AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::player::Player;
use crate::sidebar::Sidebar;
use legacy::{Manifest, Module};
use crate::debug::DebugSettings;
use crate::path::{manifest_url, module_url};
use awsm_web::loaders::fetch::fetch_url;
use utils::resize::*;
use utils::events::ModuleResizeEvent;

pub struct Shell {
    pub jig_id: String,
    pub selected_index: Mutable<Option<usize>>,
    pub manifest: Mutable<Option<Manifest>>,
    pub debug: DebugSettings,
    pub manifest_loader: AsyncLoader,
    pub module_loader: AsyncLoader,
    pub module: RefCell<Option<Module>>,
    //this is used to signal of the module is loaded
    //so that we don't need to clone it in order to get a signal
    pub loaded_index: Mutable<Option<usize>>,
}

impl Shell {
    pub fn render(jig_id: String, init_module_id: Option<String>) -> Dom {
        let _self = Rc::new(Self {
            manifest_loader: AsyncLoader::new(),
            module_loader: AsyncLoader::new(),
            selected_index: Mutable::new(None),
            manifest: Mutable::new(None),
            loaded_index: Mutable::new(None),
            module: RefCell::new(None),
            jig_id,
            debug: crate::debug::settings()
        });


        _self.manifest_loader.load(clone!(_self => async move {
            match fetch_url(&manifest_url(&_self.jig_id)).await {
                Ok(resp) => {
                    let manifest:Manifest = resp.json_from_obj().await.unwrap();
                    let init_index = 
                        init_module_id
                            .and_then(|module_id| {
                                manifest
                                    .modules
                                    .iter()
                                    .position(|id| {
                                        id == &module_id
                                    })
                            })
                            .unwrap_or_default();

                    _self.manifest.set(Some(manifest));
                    _self.selected_index.set(Some(init_index));
                },
                Err(err) => {
                    panic!("unable to load manifest!"); 
                }
            }
        }));

        html!("module-page-grid-resize", {
            .event(|event:ModuleResizeEvent| {
                //in utils / global static
                set_resize_info(event.data());
            })
            .future(clone!(_self => {
                _self.selected_index.signal().for_each(clone!(_self => move |index| {
                    _self.loaded_index.set(None);
                    if let Some(index) = index {
                        _self.module_loader.load(clone!(_self => async move {
                            match fetch_url(&module_url(&_self.jig_id, index)).await {
                                Ok(resp) => {
                                    let module:Module = resp.json_from_obj().await.unwrap();
                                    *_self.module.borrow_mut() = Some(module);
                                    _self.loaded_index.set(Some(index));
                                },
                                Err(err) => {
                                    panic!("could not load module!");
                                }
                            }
                        }));
                    }

                    async {}
                }))
            }))

            .property("legacy", true)
            .children(vec![
                Sidebar::render(_self.clone()),
                html!("div", {
                    .property("slot", "main")
                    .property("width", "100%")
                    .property("height", "100%")
                    .child_signal(_self.loaded_index.signal().map(clone!(_self => move |index| {
                        index
                            .and_then(|_| {
                                _self.module.borrow_mut().take()
                                    .map(|module| {
                                        Player::render(_self.clone(), module)
                                    })
                            })
                    })))
                })
            ])
        })
    }

    pub fn modules_len(&self) -> impl Signal<Item = u32> {
        self.manifest.signal_ref(|manifest| {
            match manifest {
                None => 0,
                Some(manifest) => manifest.modules.len() as u32
            }
        })
    }

    pub fn module_id(&self) -> impl Signal<Item = Option<String>> {
        map_ref! {
            let manifest = self.manifest.signal_cloned(),
            let selected_index = self.selected_index.signal()
            => {
                match (manifest, selected_index) {
                    (Some(manifest), Some(selected_index)) => {
                        Some(manifest.modules[*selected_index].clone())
                    },
                    _ => None
                }
            }
        }
    }
}
/*
      <module-page-grid-resize legacy>
        <legacy-sidebar slot="sidebar" nModules="${nModules}">
        ${mapToString(
            arrayIndex(nModules), 
            index => {
                return `
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

                </legacy-sidebar-module>`
            }
        )}
        </legacy-sidebar>
        <legacy-main slot="main"></legacy-main>
      </module-page-grid-resize>
      */
