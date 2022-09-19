use std::rc::Rc;

use super::{super::actions::file_change, player};
use crate::audio::input::state::{AudioInput, AudioInputAddMethod, AudioInputMode};
use crate::file_input::{FileInput, FileInputConfig, MaxSize};
use dominator::{clone, html, Dom};
use futures_signals::signal::{from_stream, SignalExt};
use gloo_timers::future::IntervalStream;
use utils::component::Component;
use wasm_bindgen_futures::spawn_local;

const STR_UPLOADING_TEXT: &str = "Uploading... please wait.";

pub fn render(state: Rc<AudioInput>, mode: AudioInputMode, add_method: AudioInputAddMethod) -> Dom {
    match mode {
        AudioInputMode::Playing(audio) => player::dom::render(state, audio),
        AudioInputMode::Stopped(_) => render_input_icon("success"),
        AudioInputMode::Empty => render_start(state, add_method),
        AudioInputMode::Recording => render_recording(),
        AudioInputMode::Uploading => render_uploading(),
    }
}

fn render_start(state: Rc<AudioInput>, add_method: AudioInputAddMethod) -> Dom {
    match add_method {
        AudioInputAddMethod::Record => render_input_icon("record"),
        AudioInputAddMethod::Upload => FileInput::new(FileInputConfig {
            max_size: MaxSize::MB5,
            accept: "audio/*",
            slot: Some("main-content"),
            show_border: false,
            on_change: Box::new(clone!(state => move |file| {
                spawn_local(clone!(state => async move {
                    if let Some(file) = file {
                        file_change(state.clone(), file).await;
                    }
                }));
            })),
            ..Default::default()
        })
        .render(),
    }
}

fn render_recording() -> Dom {
    html!("audio-input-recording", {
        .property("slot", "main-content")
    })
}

fn render_input_icon(kind: &str) -> Dom {
    html!("audio-input-icon", {
        .property("kind", kind)
        .property("slot", "main-content")
    })
}

fn render_uploading() -> Dom {
    let mut progress = 0_f64;
    let mut step = 0.8_f64;

    html!("progress-bar", {
        .property("color", "blue")
        .property("progress", 50)
        .property("slot", "main-content")
        .property_signal("progress", from_stream(IntervalStream::new(50)).map(move |_| {
            progress += step;
            if progress > 95_f64 {
                step = 0.01;
            } else if progress > 80_f64 {
                step = 0.1;
            } else if progress > 60_f64 {
                step = 0.3;
            } else if progress > 40_f64 {
                step = 0.5;
            };
            progress as i32
        }))
        .child(html!("div", {
            .property("slot", "progress-label")
            .text(STR_UPLOADING_TEXT)
        }))
    })
}
