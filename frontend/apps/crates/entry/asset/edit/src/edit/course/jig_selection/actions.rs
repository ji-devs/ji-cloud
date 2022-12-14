use std::rc::Rc;

use dominator::clone;
use itertools::Itertools;
use shared::{
    api::endpoints,
    domain::{
        course::{CourseGetDraftPath, CourseUpdateDraftDataPath, CourseUpdateDraftDataRequest},
        jig::{JigGetLivePath, JigId, JigResponse, JigSearchPath, JigSearchQuery},
    },
};
use utils::{drag::Drag, prelude::ApiEndpointExt, unwrap::UnwrapJiExt};
use web_sys::HtmlElement;

use crate::edit::sidebar::{CourseSpot, SidebarSpot, SidebarSpotItem};

use super::state::JigSelection;

impl JigSelection {
    pub fn load_course(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            let res = endpoints::course::GetDraft::api_with_auth(
                CourseGetDraftPath(state.course_id),
                None,
            )
            .await;

            match res {
                Ok(course) => {
                    let mut items = Vec::with_capacity(course.course_data.items.len());
                    for jig_id in course.course_data.items {
                        let jig = state.get_jig(&jig_id).await;
                        items.push(SidebarSpot::new_course_item(jig));
                    }
                    state.asset_edit_state.sidebar_spots.lock_mut().replace_cloned(items);
                },
                Err(_) => todo!(),
            }
        }));
    }

    pub fn save_course(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            let items = state
                .asset_edit_state
                .sidebar_spots
                .lock_ref()
                .iter()
                .filter_map(|spot| {
                    // filter out cover and empty spots
                    match &spot.item {
                        SidebarSpotItem::Jig(_) => unreachable!(),
                        SidebarSpotItem::Course(spot) => {
                            match spot {
                                None => None,
                                Some(spot) => {
                                    match &**spot {
                                        CourseSpot::Cover(_) => None,
                                        CourseSpot::Item(jig) => Some(jig.id),
                                    }
                                },
                            }
                        },
                    }
                })
                .collect_vec();
            let req = CourseUpdateDraftDataRequest {
                items: Some(items),
                ..Default::default()
            };

            let _ = endpoints::course::UpdateDraftData::api_with_auth_empty(
                CourseUpdateDraftDataPath(state.course_id.clone()),
                Some(req),
            )
            .await;
        }));
    }

    pub fn add_jig(self: &Rc<Self>, jig: Rc<JigResponse>) {
        let item = SidebarSpot::new_course_item((*jig).clone());
        self.asset_edit_state
            .sidebar_spots
            .lock_mut()
            .push_cloned(item);
        self.save_course();
    }

    // pub fn remove_jig(self: &Rc<Self>, to_remove: &JigId) {
    //     self.asset_edit_state.sidebar_spots.lock_mut().retain(|jig| &jig.id != to_remove);
    //     self.save_course();
    // }

    // pub fn move_up_jig(self: &Rc<Self>, jig_id: &JigId) {
    //     let mut items = self.asset_edit_state.sidebar_spots.lock_mut();
    //     let pos = items.iter().position(|jig| &jig.id == jig_id).unwrap();
    //     items.move_from_to(pos, pos - 1);
    //     self.save_course();
    // }

    // pub fn move_down_jig(self: &Rc<Self>, jig_id: &JigId) {
    //     let mut items = self.asset_edit_state.sidebar_spots.lock_mut();
    //     let pos = items.iter().position(|jig| &jig.id == jig_id).unwrap();
    //     items.move_from_to(pos, pos + 1);
    //     self.save_course();
    // }

    async fn get_jig(self: &Rc<Self>, jig_id: &JigId) -> JigResponse {
        endpoints::jig::GetLive::api_with_auth(JigGetLivePath(jig_id.clone()), None)
            .await
            .unwrap_ji()
    }

    pub fn search(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            let req = JigSearchQuery {
                q: String::from(state.input.borrow().clone()),
                ..Default::default()
            };

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

    pub fn on_pointer_down(self: &Rc<Self>, elem: &HtmlElement, x: i32, y: i32) {
        let drag = Drag::new_anchor_element_resize(x, y, elem, true, ());
        self.drag.set(Some(Rc::new(drag)));
    }

    pub fn on_pointer_move(self: &Rc<Self>, drag: &Rc<Drag<()>>, x: i32, y: i32) {
        drag.update(x, y);
    }

    pub fn on_pointer_up(self: &Rc<Self>, _drag: &Rc<Drag<()>>, _x: i32, _y: i32) {
        // let asset_id = AssetId::Jig();
        // drag.trigger_drop_event(x, y, self.kind.as_str());
        self.stop_drag();
    }

    pub fn stop_drag(self: &Rc<Self>) {
        self.drag.set(None);
    }
}
