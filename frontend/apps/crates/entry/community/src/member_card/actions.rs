use shared::{
    api::endpoints,
    domain::user::{
        public_user::{PublicUserFollowPath, PublicUserUnfollowPath},
        UserId,
    },
};
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};
use wasm_bindgen_futures::spawn_local;

pub fn follow_user(member_id: UserId) {
    spawn_local(async move {
        endpoints::user::Follow::api_with_auth(PublicUserFollowPath(member_id), None)
            .await
            .unwrap_ji();
    });
}

pub fn unfollow_user(member_id: UserId) {
    spawn_local(async move {
        endpoints::user::Unfollow::api_with_auth(PublicUserUnfollowPath(member_id), None)
            .await
            .unwrap_ji();
    })
}
