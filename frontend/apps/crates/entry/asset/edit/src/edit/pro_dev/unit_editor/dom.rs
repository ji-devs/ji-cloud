use std::rc::Rc;

use dominator::{html, DomBuilder};
use utils::component::Component;
use web_sys::ShadowRoot;

use super::state::UnitEditor;

impl Component<UnitEditor> for Rc<UnitEditor> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        dom.child(html!("div", {
            .text("Unit: ")
            .apply(|dom| {
                match state.unit_id {
                    Some(unit_id) => dom.text(&unit_id.0.to_string()),
                    None => dom.text("new"),
                }
            })
        }))
    }
}
