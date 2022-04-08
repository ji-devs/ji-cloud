use awsm_web::loaders::helpers::AbortController;
use js_sys::Promise;
use shared::{
    domain::jig::{module::ModuleId, JigId},
    media::MediaLibrary,
};
use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
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
            Some(*abort_controller),
        ),
        None => waitForUploadReady(&media_id.to_string(), library.to_str(), None),
    };

    async move { JsFuture::from(promise).await.is_ok() }
}

static GLOBAL_SCREENSHOT_LISTENER_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct ScreenshotListener {
    pub(super) closure: Closure<dyn FnMut()>,
    id: usize,
}

impl ScreenshotListener {
    pub fn new(on_update: impl FnMut() + 'static) -> Self {
        let id = GLOBAL_SCREENSHOT_LISTENER_COUNT.fetch_add(1, Ordering::SeqCst);
        let closure = Closure::wrap(Box::new(on_update) as Box<dyn FnMut()>);

        Self { closure, id }
    }
}

impl Drop for ScreenshotListener {
    fn drop(&mut self) {
        clearScreenshotListener(self.id);
        log::info!("listener dropped!");
    }
}

pub fn listen_for_screenshot_updates(
    jig_id: &JigId,
    module_id: &ModuleId,
    on_update: impl FnMut() + 'static,
) -> ScreenshotListener {
    init();

    let listener = ScreenshotListener::new(on_update);

    listenForScreenshotUpdates(
        &jig_id.0.to_string(),
        &module_id.0.to_string(),
        listener.id,
        &listener.closure,
    );

    listener
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

    fn listenForScreenshotUpdates(
        jig_id: &str,
        module_id: &str,
        listener_id: usize,
        on_updated: &Closure<dyn FnMut()>,
    );

    fn clearScreenshotListener(id: usize);
}
