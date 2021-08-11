use js_sys::Promise;
use wasm_bindgen::prelude::*;

use awsm_web::loaders::helpers::AbortController;
use shared::media::MediaLibrary;
use std::future::Future;
use uuid::Uuid;
use wasm_bindgen_futures::JsFuture;

// If an AbortController is provided, then dropping it will cause the JS promise to reject and this
// future will resolve with false
// In all cases, when upload is completed, the Future will resolve with true
pub fn wait_for_upload_ready(
    media_id: &Uuid,
    library: MediaLibrary,
    abort_controller: Option<&AbortController>,
) -> impl Future<Output = bool> {
    init();

    let promise = match abort_controller.as_ref() {
        Some(abort_controller) => waitForUploadReady(
            &media_id.to_string(),
            library.to_str(),
            Some(&*abort_controller),
        ),
        None => waitForUploadReady(&media_id.to_string(), library.to_str(), None),
    };

    async move {
        match JsFuture::from(promise).await {
            Ok(_) => true,
            Err(_) => false,
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
    fn waitForUploadReady(
        media_id: &str,
        lib_id: &str,
        abort_controller: Option<&web_sys::AbortController>,
    ) -> Promise;
}
