use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal::{from_future, SignalExt};
use utils::routes::AdminJigCurationRoute;

use crate::jig_curation::{details::state::JigDetails, table::state::JigTable};

use super::JigCuration;

impl JigCuration {
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
                    AdminJigCurationRoute::Table => {
                        JigTable::new(
                            Rc::clone(&state)
                        ).render()
                    },
                    AdminJigCurationRoute::Jig(jig_id) => {
                        html!("empty-fragment", {
                            .child_signal(from_future(state.clone().get_jig(*jig_id)).map(clone!(state => move|jig| {
                                jig.map(|jig| {
                                    JigDetails::new(
                                        Rc::clone(&state),
                                        jig.id,
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
