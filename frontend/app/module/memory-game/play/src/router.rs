use core::routes::{Route, ModuleRoute};
use shared::domain::jig::ModuleKind;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use dominator::{Dom, html};
use crate::pages::PlayerPage;

pub struct Router {
}

impl Router {
    pub fn new() -> Self {
        Self { }
    }

    fn signal() -> impl Signal<Item = (Route, Url)> {
        dominator::routing::url()
            .signal_ref(|url| (
                Route::from_url(&url),
                Url::new(&url).unwrap_throw()
            ))
    }

    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
        Self::signal()
            .map(|(route, url)| {
                match route {
                    Route::Module(route) => {
                        match route {
                            ModuleRoute::Play(kind, jig_id, module_id) => {
                                match kind {
                                    ModuleKind::MemoryGame => Some(PlayerPage::render(PlayerPage::new(url, jig_id, module_id))),
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
        html!("main", { .child_signal(Self::dom_signal()) } )
    }
}
