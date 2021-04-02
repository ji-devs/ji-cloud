use std::rc::Rc;

use dominator::{Dom, clone, html};
use futures_signals::signal::{SignalExt, from_stream};
use gloo_timers::future::IntervalStream;
use utils::events;
use wasm_bindgen_futures::spawn_local;

use crate::audio_input::state::{State, AudioInputMode};
use super::{super::actions::file_change, play_progress};


pub fn render(state: Rc<State>) -> Dom {
    let mode = state.mode.lock_ref();
    match *mode {
        AudioInputMode::Record => render_input_icon("record"),
        AudioInputMode::Success => render_input_icon("success"),
        AudioInputMode::Playing => render_player(state.clone()),
        AudioInputMode::Uploading => render_uploading(),
        AudioInputMode::Recording => {
            html!("audio-input-recording", {
                .property("slot", "main-content")
            })
        },
        AudioInputMode::Upload => {
            html!("input-file", {
                .property("slot", "main-content")
                .child(html!("audio-input-icon", {
                    .property("kind", "upload")
                }))
                .event(clone!(state => move |evt: events::CustomFile| {
                    let state = state.clone();
                    spawn_local(async move {
                        let file = evt.file();
                        file_change(state.clone(), file).await;
                    });
                }))
            })
        },
    }
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
    })
}

fn render_player(state: Rc<State>) -> Dom {
    play_progress::render(state.clone())
}
