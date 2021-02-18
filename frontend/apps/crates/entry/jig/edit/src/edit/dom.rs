use dominator::{html, clone, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use super::{
    sidebar::dom::SidebarDom,
    selection::dom::SelectionDom
};
use shared::domain::jig::{JigId, ModuleId};

pub struct EditPage {
}

impl EditPage {
    pub fn render(jig_id: JigId, module_id: Option<ModuleId>) -> Dom {
        let module_id = Mutable::new(module_id);

        html!("jig-edit-page", {
            .child(SidebarDom::render(jig_id.clone(), module_id.clone()))
            .child_signal(module_id.signal_cloned().map(|module_id| {
                match module_id {
                    None => {
                        Some(SelectionDom::render())
                    },

                    Some(module_id) => {
                        //TODO - iframe?
                        None
                    }
                }
            }))

        })
    }
}
