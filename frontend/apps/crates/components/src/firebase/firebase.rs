use shared::domain::{asset::AssetId, module::ModuleId};
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::prelude::*;

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
    asset_id: &AssetId,
    module_id: &ModuleId,
    on_update: impl FnMut() + 'static,
) -> ScreenshotListener {
    init();

    let listener = ScreenshotListener::new(on_update);

    listenForScreenshotUpdates(
        &asset_id.uuid().to_string(),
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

    fn listenForScreenshotUpdates(
        jig_id: &str,
        module_id: &str,
        listener_id: usize,
        on_updated: &Closure<dyn FnMut()>,
    );

    fn clearScreenshotListener(id: usize);
}
