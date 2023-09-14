use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use futures_signals::{map_ref, signal::Signal};
use std::rc::Rc;

use crate::users::{FetchMode, Users};

pub struct UsersTable {
    pub loader: AsyncLoader,
    pub search_query: Mutable<String>,
    pub users_state: Rc<Users>,
}

impl UsersTable {
    pub fn new(users_state: Rc<Users>) -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            search_query: Mutable::new("".into()),
            users_state,
        })
    }

    pub fn search_count(self: &Rc<Self>) -> impl Signal<Item = Option<u64>> {
        map_ref! {
            let count = self.users_state.total_user_count.signal(),
            let fetch_mode = self.users_state.fetch_mode.signal_cloned()
                => {
                log::info!("match statement");
                match fetch_mode {
                    FetchMode::Browse => None,
                    FetchMode::Search(_) => count.to_owned()
                }
            }
        }
    }
}
