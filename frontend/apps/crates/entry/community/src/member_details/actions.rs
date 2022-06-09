use std::rc::Rc;

use dominator::clone;
use futures::future::join;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{asset::UserOrMe, jig::JigBrowseQuery, user::public_user::PublicUser},
    error::EmptyError,
};
use utils::prelude::{api_no_auth, ApiEndpointExt};

use super::{Creations, MemberDetails};

impl MemberDetails {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            join(
                state.load_member(),
                state.load_members_jigs()
            ).await;
        }));
    }

    async fn load_member(self: &Rc<Self>) {
        let state = self;

        let path =
            endpoints::user::GetPublicUser::PATH.replace("{user_id}", &state.member_id.to_string());
        match api_no_auth::<PublicUser, EmptyError, ()>(
            &path,
            endpoints::user::GetPublicUser::METHOD,
            None,
        )
        .await
        {
            Ok(member) => {
                state.member.set(Some(member));
            }
            Err(_) => todo!(),
        }
    }

    async fn load_members_jigs(self: &Rc<Self>) {
        let state = self;

        let req = JigBrowseQuery {
            author_id: Some(UserOrMe::User(state.member_id)),
            ..Default::default()
        };

        match endpoints::jig::Browse::api_no_auth(Some(req)).await {
            Ok(res) => state.creations.set(Creations::Jigs(Some(res.jigs))),
            Err(_) => todo!(),
        }
    }
}
