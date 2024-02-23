use std::rc::Rc;

use dominator::{html, DomBuilder};
use utils::{component::Component, routes::ClassroomCodesRoute};
use web_sys::ShadowRoot;

use super::{jig_code_sessions::CodeSessions, jig_codes::JigCodes, jigs::Jigs, Codes};

impl Component<Codes> for Rc<Codes> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        dom.child(html!("div", {
            .child(html!("h1", {
                .text("My classes")
            }))
            .class("width-holder")
            .child(match self.route {
                ClassroomCodesRoute::Jigs => Jigs::new().render(),
                ClassroomCodesRoute::JigCodes(jig_id) => JigCodes::new(jig_id).render(),
                ClassroomCodesRoute::JigCodeSession(jig_id, code) => CodeSessions::new(jig_id, code).render(),
            })
        }))
    }
}
