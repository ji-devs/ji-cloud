use super::raw;
use std::{
    rc::Rc,
    cell::RefCell
};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal, self},
    signal_vec::{MutableVec, SignalVecExt, SignalVec, self},
};
use rand::prelude::*;
use shared::{domain::{image::ImageId, jig::{JigId, module::{ModuleId, body::Instructions}}}, media::MediaLibrary};
use wasm_bindgen::UnwrapThrowExt;
use std::future::Future;
use futures::future::join_all;
use gloo_timers::future::TimeoutFuture;
use utils::prelude::*;
use components::instructions::player::InstructionsPlayer;
use web_sys::AudioContext;

pub struct State {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub theme_id: ThemeId,
    pub instructions: InstructionsPlayer,
    pub audio_ctx: AudioContext
}

impl State {
    pub fn new(jig_id: JigId, module_id: ModuleId, raw_data:raw::ModuleData) -> Self {

        let audio_ctx = AudioContext::new().unwrap_ji();

        Self {
            jig_id,
            module_id,
            theme_id: raw_data.theme_id,
            instructions: InstructionsPlayer::new(raw_data.instructions), 
            audio_ctx,
        }
    }

}
