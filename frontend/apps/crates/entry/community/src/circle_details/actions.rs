use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::circle::Circle,
    error::EmptyError,
};
use utils::{
    prelude::{api_no_auth, api_with_auth_empty},
    unwrap::UnwrapJiExt,
};

use super::CircleDetails;

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

        let path = endpoints::circle::Get::PATH.replace("{id}", &state.circle_id.0.to_string());
        match api_no_auth::<Circle, EmptyError, ()>(&path, endpoints::circle::Get::METHOD, None)
            .await
        {
            Ok(circle) => {
                state.circle.set(Some(circle));
            }
            Err(_) => todo!(),
        }
    }

    async fn load_circle_members(self: &Rc<Self>) {
        // let state = self;

        // let path = endpoints::circle::BrowseMembers::PATH.replace("{id}", &state.circle_id.0.to_string());
        // match api_no_auth::<BrowseMembersResponse, EmptyError, ()>(
        //     &path,
        //     endpoints::circle::BrowseMembers::METHOD,
        //     None
        // ).await {
        //     Ok(res) => {
        //         state.members.lock_mut().extend(res.members);
        //     },
        //     Err(_) => todo!(),
        // }
    }

    pub fn join_circle(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            let path = endpoints::circle::JoinCircle::PATH.replace("{id}", &state.circle_id.0.to_string());
            match api_with_auth_empty::<EmptyError, ()>(&path, endpoints::circle::JoinCircle::METHOD, None).await
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
            let path = endpoints::circle::LeaveCircle::PATH.replace("{id}", &state.circle_id.0.to_string());
            match api_with_auth_empty::<EmptyError, ()>(&path, endpoints::circle::LeaveCircle::METHOD, None).await
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
}
