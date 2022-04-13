use serde::{Deserialize, Serialize};
use utils::unwrap::UnwrapJiExt;
use wasm_bindgen::prelude::*;
use web_sys::File;

#[derive(Serialize, Deserialize)]
struct RecorderConfig {
    #[serde(rename = "wasmURL")]
    pub wasm_url: String,
}

#[wasm_bindgen(module = "vmsg")]
extern "C" {
    #[wasm_bindgen(js_name = "Recorder")]
    type Recorder;

    #[wasm_bindgen(constructor)]
    fn new(config: &JsValue) -> Recorder;

    #[wasm_bindgen(method)]
    async fn initAudio(this: &Recorder);

    #[wasm_bindgen(method)]
    async fn initWorker(this: &Recorder);

    #[wasm_bindgen(method)]
    fn startRecording(this: &Recorder);

    #[wasm_bindgen(method)]
    async fn stopRecording(this: &Recorder) -> JsValue;
}

pub struct AudioRecorder {
    vmsg: Recorder,
}

impl Default for AudioRecorder {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioRecorder {
    pub fn new() -> Self {
        let config = RecorderConfig {
            wasm_url: "https://unpkg.com/vmsg@0.3.0/vmsg.wasm".to_string(),
        };
        let config = serde_wasm_bindgen::to_value(&config).unwrap_ji();
        Self {
            vmsg: Recorder::new(&config),
        }
    }

    pub async fn start(&self) {
        self.vmsg.initAudio().await;
        self.vmsg.initWorker().await;
        self.vmsg.startRecording();
    }

    pub async fn stop(&self) -> File {
        self.vmsg.stopRecording().await.into()
    }
}
