use std::rc::Rc;

use dominator::{html, DomBuilder};
use utils::{
    component::Component,
    link,
    routes::{ClassroomCodesRoute, ClassroomRoute, Route},
};
use web_sys::ShadowRoot;

use super::{jig_code_sessions::CodeSessions, jig_codes::JigCodes, jigs::Jigs, Codes};

impl Component<Codes> for Rc<Codes> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        dom.child(html!("h1", {
            .child(link!(Route::Classroom(ClassroomRoute::Codes(ClassroomCodesRoute::Jigs)), {
                .text("My classes")
            }))
        }))
        .child(html!("div", {
            .class("width-holder")
            .child(match self.route {
                ClassroomCodesRoute::Jigs => Jigs::new().render(),
                ClassroomCodesRoute::JigCodes(jig_id) => JigCodes::new(jig_id).render(),
                ClassroomCodesRoute::JigCodeSession(jig_id, code) => CodeSessions::new(jig_id, code).render(),
            })
        }))
    }
}
