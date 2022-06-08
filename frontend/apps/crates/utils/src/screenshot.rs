use serde::Deserialize;

use super::init::settings::SETTINGS;
use crate::unwrap::UnwrapJiExt;
use awsm_web::loaders::fetch::fetch_url;
use shared::domain::{
    asset::{AssetId, DraftOrLive},
    module::{ModuleId, ModuleKind},
};

pub const SCREENSHOT_PARAM: &str = "screenshot";

pub fn is_screenshot_url() -> bool {
    let url: String = dominator::routing::url().get_cloned();
    let url: web_sys::Url = web_sys::Url::new(&url).unwrap_ji();
    let params = url.search_params();

    match params.get(SCREENSHOT_PARAM) {
        None => false,
        Some(value) => value == "true",
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct ScreenshotResponse {
    jpg: String,
    #[serde(rename = "taskName")]
    task_name: String,
    #[serde(rename = "taskUrl")]
    task_url: String,
}

pub async fn call_screenshot_service(
    asset_id: AssetId,
    module_id: ModuleId,
    kind: ModuleKind,
    draft_of_live: DraftOrLive,
) {
    let screenshot_url = SETTINGS.get().unwrap_ji().remote_target.screenshot_url();

    let url = format!(
        "{}?jig={}&module={}&kind={}&draft_of_live{}",
        screenshot_url,
        asset_id.uuid(),
        module_id.0,
        kind.as_str(),
        draft_of_live.as_str(),
    );

    match fetch_url(&url).await {
        Ok(resp) => match resp.json_from_str::<ScreenshotResponse>().await {
            Ok(_) => {}
            Err(_) => {
                log::error!("Couldn't deserialize screenshot response!");
            }
        },
        Err(_) => {
            log::error!("Couldn't save screenshot!");
        }
    }
}
