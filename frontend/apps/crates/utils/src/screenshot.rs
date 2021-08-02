use serde::{Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;
use crate::unwrap::UnwrapJiExt;
use super::settings::SETTINGS;
use shared::domain::jig::{JigId, ModuleKind, module::ModuleId};
use awsm_web::loaders::fetch::fetch_url;

pub const SCREENSHOT_PARAM:&'static str = "screenshot";

pub fn is_screenshot_url() -> bool { 
    let url:String = dominator::routing::url().get_cloned();
    let url:web_sys::Url = web_sys::Url::new(&url).unwrap_ji();
    let params = url.search_params();

    match params.get(SCREENSHOT_PARAM) {
        None => false,
        Some(value) => {
            if value == "true" {
                true
            } else {
                false
            }
        }
    }
}

#[derive(Deserialize)]
struct ScreenshotResponse {
    saved: bool
}

pub async fn call_screenshot_service(jig_id: JigId, module_id: ModuleId, kind: ModuleKind) {
    let cloud_functions_url = SETTINGS.get().unwrap_ji().remote_target.cloud_functions_url();

    //TIP: swap endpoint with "saveScreenshotSandbox" to debug url in browser and see the image
    let endpoint = "saveScreenshotSandbox"; 

    let url = format!("{}/{}?jig={}&module={}&kind={}", cloud_functions_url, endpoint, jig_id.0.to_string(), module_id.0.to_string(), kind.as_str());


    match fetch_url(&url).await {
        Ok(resp) => {
            match resp.json_from_str::<ScreenshotResponse>().await {
                Ok(_) => {
                },
                Err(_) => {
                    log::error!("Couldn't deserialize screenshot response!");
                }
            }
        },
        Err(_) => {
            log::error!("Couldn't save screenshot!");
        }
    }
}
