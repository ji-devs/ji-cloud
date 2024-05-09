use awsm_web::loaders::fetch::fetch_with_headers_and_data;
use serde::Serialize;
use shared::domain::jig::JigId;
use wasm_bindgen_futures::spawn_local;

use crate::{prelude::get_user_id, unwrap::UnwrapJiExt};

#[cfg(feature = "release")]
const JIG_INDEX: &str = "release_jig";
#[cfg(not(feature = "release"))]
const JIG_INDEX: &str = "sandbox_jig";

const ALGOLIA_EVENTS_URL: &str = "https://insights.algolia.io/1/events";

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
    if let Some(user_id) = get_user_id() {
        data.as_object_mut()
            .unwrap()
            .insert("userToken".into(), user_id.to_string().into());
    }
    let data = serde_json::json!({
        "events": [data],
    });
    send_event(data);
}
