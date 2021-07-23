use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;
use serde::{Deserialize};
use utils::prelude::*;

pub struct FirebaseListener {
    pub id: usize,
    pub closure: Closure<dyn FnMut(JsValue)>
}
impl Drop for FirebaseListener {
    fn drop(&mut self) {
        removeUploadListener(self.id);
        log::info!("firebase listener #{} has dropped!", self.id);
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UploadStatus {
    pub ready: bool,
    pub processing: bool
}

// It is up to the caller to keep the listener valid 
pub fn add_upload_listener(media_id:&str, mut on_status: impl FnMut(UploadStatus) + 'static) -> FirebaseListener {
    init();

    let closure = Closure::new(move |status| {
        on_status(serde_wasm_bindgen::from_value(status).unwrap_ji());
    });

    let id = addUploadListener(media_id, &closure);

    FirebaseListener { id, closure }
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
    fn addUploadListener(media_id: &str, on_ready: &Closure<dyn FnMut(JsValue)>) -> usize;
    fn removeUploadListener(listener_id: usize);
}
