use std::rc::Rc;

use dominator::{clone, html, Dom};
use utils::routes::AdminUsersRoute;

use crate::users::table::state::UsersTable;

use super::Users;

impl Users {
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
                    AdminUsersRoute::Table => {
                        UsersTable::new(
                            Rc::clone(&state)
                        ).render()
                    },
                    AdminUsersRoute::User(_) => todo!()
                })
            })))
        })
    }
}
