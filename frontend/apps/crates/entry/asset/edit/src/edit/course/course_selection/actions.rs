use std::rc::Rc;

use dominator::clone;
use itertools::Itertools;
use shared::{
    api::endpoints,
    domain::{
        asset::Asset,
        jig::{JigResponse, JigSearchPath},
    },
};
use utils::{drag::Drag, prelude::ApiEndpointExt, unwrap::UnwrapJiExt};
use web_sys::HtmlElement;

use super::state::CourseSelection;

impl CourseSelection {
    pub fn search(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            let req = state.search_bar.search_selected.to_jig_search_request();

            match endpoints::jig::Search::api_no_auth(JigSearchPath(), Some(req)).await {
                Err(_) => todo!(),
                Ok(res) => {
                    let jigs = res
                        .jigs
                        .into_iter()
                        .map(|jig| Rc::new(jig))
                        .collect_vec();
                    state.search_results.lock_mut().replace_cloned(jigs);
                }
            };

        }));
    }

    pub fn on_pointer_down(
        self: &Rc<Self>,
        elem: &HtmlElement,
        x: i32,
        y: i32,
        jig: &Rc<JigResponse>,
    ) {
        let drag = Drag::new_anchor_element_resize(x, y, elem, true, (**jig).clone().into());
        self.drag.set(Some(Rc::new(drag)));
    }

    pub fn on_pointer_move(self: &Rc<Self>, drag: &Rc<Drag<Asset>>, x: i32, y: i32) {
        drag.update(x, y);
    }

    pub fn on_pointer_up(self: &Rc<Self>, drag: &Rc<Drag<Asset>>, x: i32, y: i32) {
        let data = serde_json::to_string(&drag.data).unwrap_ji();
        drag.trigger_drop_event(x, y, &data);
        self.stop_drag();
    }

    pub fn stop_drag(self: &Rc<Self>) {
        self.drag.set(None);
    }
}
