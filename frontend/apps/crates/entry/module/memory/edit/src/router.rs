use utils::routes::{Route, ModuleRoute};
use shared::domain::jig::ModuleKind;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use dominator::{Dom, html};
use utils::components::module_page::*;
use crate::pages::index::IndexPage;

pub struct Router {
}

impl Router {
    pub fn new() -> Self {
        Self { }
    }

    fn signal() -> impl Signal<Item = Route> {
        dominator::routing::url()
            .signal_ref(|url| Route::from_url(&url))
    }
    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
        Self::signal()
            .map(|route| {
                match route {
                    Route::Module(route) => {
                        match route {
                            ModuleRoute::Edit(kind, jig_id, module_id) => {
                                match kind {
                                    ModuleKind::MemoryGame => Some(
                                        ModulePage::render(ModulePage::new(
                                            IndexPage::new(jig_id, module_id)
                                        ))
                                    ),
                                    _ => None
                                }
                            }
                            _ => None
                        }
                    },
                    _ => None
                }
            })
    }
    
    pub fn render(&self) -> Dom {
        html!("div", { .child_signal(Self::dom_signal()) } )
    }
}
