use std::rc::Rc;

use dominator::{Dom, html, clone};
use utils::routes::AdminCurationRoute;

use crate::curation::{table::state::CurationTable, jig::state::CurationJig};

use super::Curation;

impl Curation {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        state.load_data();

        html!("empty-fragment", {
            .child_signal(self.route.signal_ref(clone!(state => move|route| {
                Some(match route {
                    AdminCurationRoute::Table => {
                        CurationTable::new(
                            Rc::clone(&state)
                        ).render()
                    },
                    AdminCurationRoute::Jig(jig_id) => {
                        CurationJig::new(
                            Rc::clone(&state),
                            jig_id.clone()
                        ).render()
                    },
                })
            })))
        })
    }
}
