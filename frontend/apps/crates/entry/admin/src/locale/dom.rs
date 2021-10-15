use crate::locale::components::locale_outer::LocaleOuterDom;

use super::state::*;
use dominator::{html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

pub struct LocalePage {
    pub state: Rc<State>,
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
