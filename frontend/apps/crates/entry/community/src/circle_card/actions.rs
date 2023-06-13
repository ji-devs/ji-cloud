use shared::{
    api::endpoints,
    domain::circle::{CircleId, JoinCirclePath, LeaveCirclePath},
};
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};
use wasm_bindgen_futures::spawn_local;

pub fn join_circle(member_id: CircleId) {
    spawn_local(async move {
        endpoints::circle::JoinCircle::api_with_auth_empty(JoinCirclePath(member_id), None)
            .await
            .unwrap_ji();
    });
}

pub fn leave_circle(member_id: CircleId) {
    spawn_local(async move {
        endpoints::circle::LeaveCircle::api_with_auth_empty(LeaveCirclePath(member_id), None)
            .await
            .unwrap_ji();
    })
}
