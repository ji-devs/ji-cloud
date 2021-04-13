use dominator::{html, clone, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use super::{
    sidebar::dom::SidebarDom,
    selection::dom::SelectionDom,
    iframe::dom::IframeDom,
};
use shared::domain::jig::{JigId, module::ModuleId};
use utils::prelude::*;
use wasm_bindgen::prelude::*;

pub struct EditPage {
}

impl EditPage {
    pub fn render(jig_id: JigId, module_id: Option<ModuleId>) -> Dom {
        let module_id = Mutable::new(module_id);

        html!("jig-edit-page", {
            /*
             * this changes the url but does not preserve history
             * commented out since it's misleading
             * see the edit action for more comments
             * we can navigate properly but then have full refresh
             * might be worth popping/listening to location history
             * but meh
            .future(module_id.signal().for_each(clone!(jig_id => move |module_id| {
                let url:String = Route::Jig(JigRoute::Edit(jig_id, module_id)).into();

                web_sys::window()
                    .unwrap_ji()
                    .history()
                    .unwrap_ji()
                    .push_state_with_url(&JsValue::NULL, "", Some(&url))
                    .unwrap_ji();
                async {}
            })))
            */
            .child(SidebarDom::render(jig_id.clone(), module_id.clone()))
            .child_signal(module_id.signal_cloned().map(clone!(jig_id => move |module_id| {
                match module_id {
                    None => {
                        Some(SelectionDom::render())
                    },

                    Some(module_id) => {
                        Some(IframeDom::render(jig_id.clone(), module_id.clone()))
                    }
                }
            })))

        })
    }
}
