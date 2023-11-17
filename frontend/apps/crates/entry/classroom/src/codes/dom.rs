use std::rc::Rc;

use dominator::{html, DomBuilder};
use futures_signals::signal_vec::SignalVecExt;
use utils::{component::Component, link, routes::ClassroomRoute};
use web_sys::ShadowRoot;

use utils::routes::Route;

use super::Codes;

impl Component<Codes> for Rc<Codes> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        state.load_data();

        dom.child(html!("div", {
            .prop("slot", "page-header")
            .children_signal_vec(state.codes.signal_vec_cloned().map(|code| {
                link!(Route::Classroom(ClassroomRoute::CodeSession(code.index)), {
                    .class("code-section")
                    .text(&code.index.to_string())
                })
            }))
        }))
    }
}
