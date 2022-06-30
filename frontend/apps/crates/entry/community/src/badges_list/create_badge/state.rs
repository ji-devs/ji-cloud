use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal},
};
use web_sys::File;

use crate::badges_list::BadgesList;

pub struct CreateBadge {
    pub loader: AsyncLoader,
    pub name: Mutable<Option<String>>,
    pub description: Mutable<Option<String>>,
    pub image: Mutable<Option<File>>,
    pub badge_list_state: Rc<BadgesList>,
}

impl CreateBadge {
    pub fn new(badge_list_state: Rc<BadgesList>) -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            name: Mutable::default(),
            description: Mutable::default(),
            image: Mutable::default(),
            badge_list_state,
        })
    }

    pub fn can_save_signal(self: &Rc<Self>) -> impl Signal<Item = bool> {
        map_ref! {
            let image = self.image.signal_cloned(),
            let name = self.name.signal_cloned(),
            let is_loading = self.loader.is_loading() => move {
                !is_loading && name.is_some() && image.is_some()
            }
        }
    }
}
