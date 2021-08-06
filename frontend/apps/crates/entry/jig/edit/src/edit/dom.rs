use super::{
    iframe::dom::IframeDom, publish::dom::render as render_publish, selection::dom::SelectionDom,
    sidebar::dom::SidebarDom,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use shared::domain::jig::JigId;
use utils::prelude::*;

pub struct EditPage {}

impl EditPage {
    pub fn render(jig_id: JigId, route: JigEditRoute) -> Dom {
        let route = Mutable::new(route);

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
            .child(SidebarDom::render(jig_id.clone(), route.clone()))
            .child_signal(route.signal_cloned().map(clone!(jig_id => move |route| {
                match route {
                    JigEditRoute::Landing => {
                        Some(SelectionDom::render())
                    },
                    JigEditRoute::Module(module_id) => {
                        Some(IframeDom::render(jig_id.clone(), module_id.clone()))
                    },
                    JigEditRoute::Publish => {
                        Some(render_publish(jig_id.clone()))
                    }
                }
            })))

        })
    }
}
