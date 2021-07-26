use js_sys::Promise;
use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;
use utils::prelude::*;
use wasm_bindgen_futures::JsFuture;
use awsm_web::loaders::helpers::AbortController;
use std::future::Future;

// If an AbortController is provided, then dropping it will cause the JS promise to reject and this
// future will resolve with false
// In all cases, when upload is completed, the Future will resolve with true
pub fn wait_for_upload_ready(media_id:&str, abort_controller: Option<AbortController>) -> impl Future<Output = bool> {
    init();

    let promise = match abort_controller.as_ref() {
        Some(abort_controller) => waitForUploadReady(media_id, Some(&*abort_controller)),
        None => waitForUploadReady(media_id, None) 
    };

    async move {
        match JsFuture::from(promise).await {
            Ok(_) => true,
            Err(_) => false
        }
    }
}


//each public firebase interface should call init() first
//it can be called multiple times, will immediately return if already called
cfg_if::cfg_if! {
    if #[cfg(feature = "release")] {
        fn init() {
            _init("release");
        }
    } else {
        fn init() {
            _init("sandbox");
        }
    }
}

#[wasm_bindgen(module = "/js/firebase.js")]
extern "C" {
    fn _init(remote_target: &str);
    fn waitForUploadReady(media_id: &str, abort_controller: Option<&web_sys::AbortController>) -> Promise;
}
