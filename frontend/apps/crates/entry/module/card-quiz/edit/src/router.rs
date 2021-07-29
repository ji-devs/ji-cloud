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
use components::module::_common::edit::entry::dom::render_page_body;
use super::state::{AppState, create_state};

pub struct Router {
    loader: AsyncLoader,
    app: RefCell<Option<Rc<AppState>>>
}

impl Router {
    pub fn new() -> Self {
        Self {
            loader: AsyncLoader::new(),
            app: RefCell::new(None)
        }
    }
}


pub fn render(state: Rc<Router>) {

    state.clone().loader.load(
        dominator::routing::url()
            .signal_ref(|url| Route::from_url(&url))
            .for_each(clone!(state => move |route| {
                match route {
                    Route::Module(route) => {
                        match route {
                            ModuleRoute::Edit(kind, jig_id, module_id) => {
                                match kind {
                                    ModuleKind::CardQuiz => {
                                        let app = create_state(jig_id, module_id);
                                        render_page_body(app.clone());
                                        *state.app.borrow_mut() = Some(app);
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
