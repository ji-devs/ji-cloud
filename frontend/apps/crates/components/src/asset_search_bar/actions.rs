use std::rc::Rc;

use dominator::clone;

use super::AssetSearchBar;

impl AssetSearchBar {
    // TODO: use utils metadata instead
    pub fn load_data(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            state.search_options.populate_options().await;
        }));
    }
}
