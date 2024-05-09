use awsm_web::loaders::fetch::fetch_with_headers_and_data;
use serde::Serialize;
use shared::domain::{jig::JigId, user::UserId};
use wasm_bindgen_futures::spawn_local;

use crate::{prelude::get_user_id, storage::get_session_storage, unwrap::UnwrapJiExt};

#[cfg(feature = "release")]
const JIG_INDEX: &str = "release_jig";
#[cfg(not(feature = "release"))]
const JIG_INDEX: &str = "sandbox_jig";

const ALGOLIA_EVENTS_URL: &str = "https://insights.algolia.io/1/events";

const UNAUTHENTICATED: &str = "unauthenticated-token";

fn send_event(data: impl Serialize + 'static) {
    spawn_local(async move {
        fetch_with_headers_and_data(
            ALGOLIA_EVENTS_URL,
            "POST",
            false,
            &[
                ("X-Algolia-Api-Key", "367b0901cead031192bf06c10f03d5c4"),
                ("X-Algolia-Application-Id", "RCIA2LNKTS"),
            ],
            Some(data),
        )
        .await
        .unwrap_ji();
    });
}

enum UserToken {
    Authenticated(UserId),
    Unauthenticated(String),
}
impl UserToken {
    fn get() -> Self {
        match get_user_id() {
            Some(user_id) => UserToken::Authenticated(user_id),
            None => UserToken::Unauthenticated(Self::get_unauthenticated_id()),
        }
    }

    fn get_unauthenticated_id() -> String {
        let storage = get_session_storage().unwrap_ji();
        storage.get(UNAUTHENTICATED).unwrap_ji().unwrap_or_else(|| {
            let token = js_sys::Math::random().to_string().replace(".", "");
            storage.set(UNAUTHENTICATED, &token).unwrap_ji();
            token
        })
    }
}

pub fn viewed_jig(jig_id: JigId) {
    let mut data = serde_json::json!({
        "eventType": "click",
        "eventName": "JIG opened",
        "index": JIG_INDEX,
        "objectIDs": [jig_id.to_string()],
    });
    if let Some(user_id) = get_user_id() {
        data.as_object_mut()
            .unwrap()
            .insert("userToken".into(), user_id.to_string().into());
    }
    match UserToken::get() {
        UserToken::Authenticated(user_id) => {
            data.as_object_mut()
                .unwrap()
                .insert("authenticatedUserToken".into(), user_id.to_string().into());
        }
        UserToken::Unauthenticated(user_id) => {
            data.as_object_mut()
                .unwrap()
                .insert("userToken".into(), user_id.to_string().into());
        }
    }
    let data = serde_json::json!({
        "events": [data],
    });
    send_event(data);
}

pub fn finished_jig(jig_id: JigId) {
    let mut data = serde_json::json!({
        "eventType": "conversion",
        "eventName": "JIG finished",
        "index": JIG_INDEX,
        "objectIDs": [jig_id.to_string()],
    });
    match UserToken::get() {
        UserToken::Authenticated(user_id) => {
            data.as_object_mut()
                .unwrap()
                .insert("authenticatedUserToken".into(), user_id.to_string().into());
        }
        UserToken::Unauthenticated(user_id) => {
            data.as_object_mut()
                .unwrap()
                .insert("userToken".into(), user_id.to_string().into());
        }
    }
    let data = serde_json::json!({
        "events": [data],
    });
    send_event(data);
}
