use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal::{from_future, SignalExt};
use utils::routes::AdminCurationRoute;

use crate::curation::{jig::state::CurationJig, table::state::CurationTable};

use super::Curation;

impl Curation {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        state.load_data();

        html!("empty-fragment", {
            .child(html!("window-loader-block", {
                .property("slot", "loader")
                .property_signal("visible", state.loader.is_loading())
            }))
            .child_signal(self.route.signal_ref(clone!(state => move|route| {
                Some(match route {
                    AdminCurationRoute::Table => {
                        CurationTable::new(
                            Rc::clone(&state)
                        ).render()
                    },
                    AdminCurationRoute::Jig(jig_id) => {
                        html!("empty-fragment", {
                            .child_signal(from_future(state.clone().get_jig(jig_id.clone())).map(clone!(state => move|jig| {
                                jig.map(|jig| {
                                    CurationJig::new(
                                        Rc::clone(&state),
                                        jig.id.clone(),
                                        jig
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
