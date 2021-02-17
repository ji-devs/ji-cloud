use futures_signals::signal::Mutable;
use crate::locale::components::locale_outer::LocaleOuterDom;
use dominator_helpers::futures::AsyncLoader;
use dominator::{Dom, html, clone};
use std::rc::Rc;
use super::state::*;
use futures_signals::signal::SignalExt;


pub struct LocalePage {
    pub state: Rc<State>
}

impl LocalePage {
    pub fn render() -> Dom {
        // pretty bad, ha?
        super::temp_utils::add_styles(&std::include_str!("./temp_styles.css"));


        let state: Mutable<Option<Rc<State>>> = Mutable::new(None);

        let loader = AsyncLoader::new();
        loader.load(clone!(state => async move {
            state.set(Some(Rc::new(State::new().await)));
        }));

        Dom::with_state(loader, move |loader| {
            html!("empty-fragment", {
                .child_signal(loader.is_loading().map(move |loading| {
                    if loading {
                        Some(html!("window-loader-block", {
                            .property("visible", true)
                        }))
                    } else {
                        let thingy: Rc<State> = state.lock_ref().clone().unwrap();
                        Some(LocaleOuterDom::render(thingy))
                    }
                }))
            })
        })
    }
}
