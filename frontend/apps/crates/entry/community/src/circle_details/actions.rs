use std::rc::Rc;

use components::confirm::Confirm;
use dominator::clone;
use futures::join;
use shared::{
    api::endpoints,
    domain::{
        circle::{
            Circle, CircleDeletePath, CircleGetPath, CircleUpdateRequest, JoinCirclePath,
            LeaveCirclePath, UpdateCirclePath,
        },
        user::public_user::{PublicUserBrowsePath, UserBrowseQuery},
    },
};
use utils::{
    prelude::ApiEndpointExt,
    routes::{CommunityCirclesRoute, CommunityRoute, Route},
    unwrap::UnwrapJiExt,
};

use super::CircleDetails;

const STR_CONFIRM_DELETE_TITLE: &str = "Confirm";
const STR_CONFIRM_DELETE_MESSAGE: &str =
    "There are members in this circle! Are you sure you want to delete it?";

impl CircleDetails {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            join!(
                state.load_circle(),
                state.load_circle_members(),
            );
        }));
    }

    async fn load_circle(self: &Rc<Self>) {
        let state = self;

        match endpoints::circle::Get::api_no_auth(CircleGetPath(state.circle_id), None).await {
            Ok(circle) => {
                state.circle.set(Some(circle));
            }
            Err(_) => todo!(),
        }
    }

    async fn load_circle_members(self: &Rc<Self>) {
        let state = self;

        let req = UserBrowseQuery {
            circles: vec![state.circle_id],
            ..Default::default()
        };

        match endpoints::user::BrowsePublicUser::api_no_auth(PublicUserBrowsePath(), Some(req))
            .await
        {
            Ok(res) => {
                state.members.lock_mut().extend(res.users);
            }
            Err(_) => todo!(),
        }
    }

    pub fn join_circle(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            match endpoints::circle::JoinCircle::api_with_auth_empty(JoinCirclePath(state.circle_id), None).await
            {
                Ok(_) => {
                    let mut user = state.community_state.user.get_cloned().unwrap_ji();
                    user.circles.push(state.circle_id);
                    state.community_state.user.set(Some(user));
                }
                Err(_) => todo!(),
            }
        }));
    }

    pub fn leave_circle(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            match endpoints::circle::LeaveCircle::api_with_auth_empty(LeaveCirclePath(state.circle_id), None).await
            {
                Ok(_) => {
                    let mut user = state.community_state.user.get_cloned().unwrap_ji();
                    let index = user.circles.iter().position(|circle| *circle == state.circle_id).unwrap();
                    user.circles.remove(index);
                    state.community_state.user.set(Some(user));
                }
                Err(_) => todo!(),
            }
        }));
    }

    pub fn save_circle_changes(self: &Rc<Self>, circle: Circle) {
        let state = self;
        state.active_popup.set(None);
        state.loader.load(clone!(state => async move {
            let req = CircleUpdateRequest {
                display_name: Some(circle.display_name.clone()),
                description: Some(circle.description.clone()),
                image: Some(circle.image),
            };

            let res = endpoints::circle::Update::api_with_auth_empty(UpdateCirclePath(state.circle_id), Some(req)).await;
            if let Err(_err) = res {
                todo!()
            }
            state.circle.set(Some(circle))
        }));
    }

    pub fn delete_circle(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            match &*state.circle.lock_ref() {
                None => {},
                Some(circle) => {
                    if circle.member_count > 0 {
                        if !Confirm::new(STR_CONFIRM_DELETE_TITLE.to_string(), STR_CONFIRM_DELETE_MESSAGE.to_string()).confirm().await {
                            return;
                        }
                    }
                },
            };

            match endpoints::circle::Delete::api_with_auth_empty(CircleDeletePath(state.circle_id), None).await
            {
                Ok(_) => {
                    let route = Route::Community(CommunityRoute::Circles(CommunityCirclesRoute::List));
                    dominator::routing::go_to_url(&route.to_string());
                }
                Err(_) => todo!(),
            }
        }));
    }
}
