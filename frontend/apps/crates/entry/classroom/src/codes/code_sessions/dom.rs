use std::rc::Rc;

use dominator::DomBuilder;
use utils::component::Component;
use web_sys::ShadowRoot;

use super::CodeSessions;

impl Component<CodeSessions> for Rc<CodeSessions> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        dom.text("code sessions")
    }
}
