use futures_signals::signal::Mutable;
use crate::locale::components::locale_outer::LocaleOuterDom;
use dominator_helpers::futures::AsyncLoader;
use dominator::{Dom, html, clone};
use std::{cell::RefCell, rc::Rc};
use super::state::*;
use futures_signals::signal::SignalExt;


pub struct LocalePage {
    pub state: Rc<State>
}

impl LocalePage {
    pub fn render() -> Dom {
        let state: Rc<RefCell<Option<State>>> = Rc::new(RefCell::new(None));

        let loader = Rc::new(AsyncLoader::new());
        loader.load(clone!(state => async move {
            state.replace(Some(State::new().await));
        }));

        Dom::with_state(loader, move |loader| {
            html!("empty-fragment", {
                .child_signal(loader.is_loading().map(move |loading| {
                    if loading {
                        Some(html!("window-loader-block", {
                            .property("visible", true)
                        }))
                    } else {
                        let state: Rc<State> = Rc::new(k);
                        Some(LocaleOuterDom::render(state))
                    }
                }))
            })
        })
    }
}
