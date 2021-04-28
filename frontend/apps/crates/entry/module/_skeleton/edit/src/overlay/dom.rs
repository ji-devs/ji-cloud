use dominator::{html, Dom, clone};
use crate::data::{raw, state::*};
use std::rc::Rc;
use utils::events;

pub struct OverlayDom {}
impl OverlayDom {
    pub fn render(state:Rc<State>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "overlay")
            .children_signal_vec(state.overlay.children())
        })
    }
}
