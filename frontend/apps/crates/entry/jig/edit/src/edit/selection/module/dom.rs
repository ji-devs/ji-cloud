use dominator::{html, clone, Dom};
use shared::domain::jig::ModuleKind;
use super::state::*;
use utils::events;
use std::rc::Rc;

pub struct ModuleDom {
}

impl ModuleDom {

    pub fn render(kind: ModuleKind) -> Dom {
        let state = Rc::new(State::new());

        html!("jig-edit-module-card", {
            .property("slot", "modules")
            .property("module", kind.as_str())
            .property_signal("drag", state.is_dragging.signal())
            .event(clone!(state => move |evt:events::DragStart| {
                if let Some(data_transfer) = evt.data_transfer() {
                    data_transfer.set_data("module_kind", &kind.as_str());
                    data_transfer.set_drop_effect("all");
                    state.is_dragging.set(true);
                } else {
                    log::error!("no data transfer - use a real computer!!!");
                }
            }))
            .event(clone!(state => move |evt:events::DragEnd| {
                state.is_dragging.set_neq(false);
            }))
            
        })
    }
}
