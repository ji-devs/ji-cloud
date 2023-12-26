use std::rc::Rc;

use dominator::{clone, html, DomBuilder};
use futures_signals::signal_vec::SignalVecExt;
use utils::{
    component::Component,
    link,
    routes::{ClassroomCodesRoute, ClassroomRoute, Route},
};
use web_sys::ShadowRoot;

use super::JigCodes;

impl Component<JigCodes> for Rc<JigCodes> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        state.load_data();

        dom.child(html!("div", {
            .text(&self.jig_id.to_string())
            .children_signal_vec(state.codes.signal_vec_cloned().map(clone!(state => move |code| {
                link!(Route::Classroom(ClassroomRoute::Codes(ClassroomCodesRoute::JigCodeSession(state.jig_id, code.index))), {
                    .class("code-section")
                    .text(&code.index.to_string())
                })
            })))
        }))
    }
}
