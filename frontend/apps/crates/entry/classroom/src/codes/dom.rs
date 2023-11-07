use std::rc::Rc;

use dominator::DomBuilder;
use utils::component::Component;
use web_sys::ShadowRoot;

use super::Codes;

impl Component<Codes> for Rc<Codes> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        dom.text("codes")
    }
}
