use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal::{from_future, SignalExt};
use utils::routes::AdminResourceCurationRoute;

use crate::resource_curation::{details::state::ResourceDetails, table::state::ResourceTable};

use super::ResourceCuration;

impl ResourceCuration {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        state.load_data();

        html!("empty-fragment", {
            .child(html!("window-loader-block", {
                .prop("slot", "loader")
                .prop_signal("visible", state.loader.is_loading())
            }))
            .child_signal(self.route.signal_ref(clone!(state => move|route| {
                Some(match route {
                    AdminResourceCurationRoute::Table => {
                        ResourceTable::new(
                            Rc::clone(&state)
                        ).render()
                    },
                    AdminResourceCurationRoute::Resource(resource_id) => {
                        html!("empty-fragment", {
                            .child_signal(from_future(state.clone().get_resource(*resource_id)).map(clone!(state => move|resource| {
                                resource.map(|resource| {
                                    ResourceDetails::new(
                                        Rc::clone(&state),
                                        resource
                                    ).render()
                                })
                            })))
                        })
                    },
                })
            })))
        })
    }
}
