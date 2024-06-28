use std::rc::Rc;

use dominator::clone;
use futures::join;
use wasm_bindgen_futures::spawn_local;

use super::Likes;

impl Likes {
    pub(super) fn load_data(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            join!(
                state.jigs.load_items(),
                state.resources.load_items(),
                state.playlists.load_items(),
            );
        }));
    }
}
