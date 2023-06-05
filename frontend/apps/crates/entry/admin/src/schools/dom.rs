use std::rc::Rc;

use crate::schools::details::state::SchoolDetails;
use crate::schools::table::state::SchoolTable;
use dominator::{clone, html, Dom};
use utils::routes::AdminSchoolsRoute;

use super::Schools;

impl Schools {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        html!("empty-fragment", {
            .child_signal(self.route.signal_ref(clone!(state => move|route| {
                Some(match route {
                    AdminSchoolsRoute::Table => {
                        SchoolTable::new(
                            Rc::clone(&state)
                        ).render()
                    },
                    AdminSchoolsRoute::School(school_id) => {
                        SchoolDetails::new(
                            Rc::clone(&state),
                            *school_id,
                        ).render()
                    },
                })
            })))
        })
    }
}
