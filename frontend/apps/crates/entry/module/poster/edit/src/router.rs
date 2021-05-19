use utils::routes::{Route, ModuleRoute};
use shared::domain::jig::ModuleKind;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use dominator::{Dom, html, clone};
use dominator_helpers::futures::AsyncLoader;
use std::cell::RefCell;
use components::module::edit::dom::render_page_body;
use super::state::{AppState, create_state};

pub struct Router {
    loader: AsyncLoader,
    state: RefCell<Option<Rc<AppState>>>
}


pub fn render() {
    let _self = Rc::new(Router {
        loader: AsyncLoader::new(),
        state: RefCell::new(None)
    });

    _self.clone().loader.load(
        dominator::routing::url()
            .signal_ref(|url| Route::from_url(&url))
            .for_each(clone!(_self => move |route| {
                match route {
                    Route::Module(route) => {
                        match route {
                            ModuleRoute::Edit(kind, jig_id, module_id) => {
                                match kind {
                                    ModuleKind::Poster => {
                                        let state = create_state(jig_id, module_id);
                                        render_page_body(state.clone());
                                        *_self.state.borrow_mut() = Some(state);
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    },
                    _ => {}
                };
                async {}
            }))
    );
}
