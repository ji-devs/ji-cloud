use super::dom;
use super::state::State;
use dominator::clone;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::SignalExt;
use std::cell::RefCell;
use std::rc::Rc;
use utils::routes::Route;

pub struct Router {
    loader: AsyncLoader,
    app: RefCell<Option<Rc<State>>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            loader: AsyncLoader::new(),
            app: RefCell::new(None),
        }
    }
}

pub fn render(state: Rc<Router>) {
    state.clone().loader.load(
        dominator::routing::url()
            .signal_ref(|url| Route::from_url(&url))
            .for_each(clone!(state => move |route| {
                match route {
                    Route::Home => {
                        let app_state = Rc::new(State::new());
                        *state.app.borrow_mut() = Some(app_state.clone());
                        let body = dominator::body();
                        body.set_inner_html("");
                        dominator::append_dom(&body, dom::render(app_state.clone()));
                    },
                    _ => {}
                };
                async {}
            })),
    );
}
