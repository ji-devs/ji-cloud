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
    pub fn render(state: Rc<LoaderState>) -> Dom {
        html!("empty-fragment", {
            .child_signal(state.inner.signal_cloned().map(|state| {
                match state {
                    Some(state) => Some(LocaleOuterDom::render(state)),
                    None => {
                        Some(html!("window-loader-block", {
                            .property("visible", true)
                        }))
                    }
                }
            }))
        })
    }
}
