use super::state::Subscribe2;
use dominator::DomBuilder;
use std::rc::Rc;
use utils::component::Component;
use web_sys::ShadowRoot;

impl Component<Subscribe2> for Rc<Subscribe2> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        self.subscribe();
        dom
    }
}
