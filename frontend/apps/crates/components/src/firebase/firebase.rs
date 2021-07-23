use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;

#[wasm_bindgen(module = "/js/firebase.js")]
extern "C" {
    fn _init(remote_target: &str);
    fn listenForUploadImage(image_id: &str);
}

//init can be called multiple times, will immediately return if already called
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

pub fn listen_for_upload_image(image_id:&str) {
    init();

    listenForUploadImage(image_id);
}
