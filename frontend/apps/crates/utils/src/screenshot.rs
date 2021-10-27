use serde::Deserialize;

use super::init::settings::SETTINGS;
use crate::unwrap::UnwrapJiExt;
use awsm_web::loaders::fetch::fetch_url;
use shared::domain::jig::{module::ModuleId, JigId, ModuleKind};

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

#[derive(Deserialize)]
struct ScreenshotResponse {
    jpg: String,
    #[serde(rename = "taskName")]
    task_name: String,
    #[serde(rename = "taskUrl")]
    task_url: String,
}

pub async fn call_screenshot_service(jig_id: JigId, module_id: ModuleId, kind: ModuleKind) {
    let screenshot_url = SETTINGS.get().unwrap_ji().remote_target.screenshot_url();

    let url = format!(
        "{}?jig={}&module={}&kind={}",
        screenshot_url,
        jig_id.0.to_string(),
        module_id.0.to_string(),
        kind.as_str()
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
