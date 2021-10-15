use std::{cell::RefCell, rc::Rc};

use awsm_web::{audio::AudioHandle, loaders::helpers::AsyncLoader};
use futures_signals::signal::Mutable;
use serde::{Deserialize, Serialize};
use shared::domain::jig::{module::ModuleId, JigId, JigPlayerSettings, JigResponse};
use utils::jig::JigPlayerOptions;
use web_sys::HtmlIFrameElement;

use super::timer::Timer;

pub struct State {
    pub jig_id: JigId,
    pub jig: Mutable<Option<JigResponse>>,
    pub loader: AsyncLoader,
    pub active_module: Mutable<usize>,
    pub module_id: Mutable<Option<ModuleId>>, // needed?
    pub timer: Mutable<Option<Timer>>,
    pub points: Mutable<u32>,
    pub iframe: Rc<RefCell<Option<HtmlIFrameElement>>>,
    pub paused: Mutable<bool>,
    pub done: Mutable<bool>,
    pub player_options: JigPlayerOptions,
    pub bg_audio_handle: Rc<RefCell<Option<AudioHandle>>>,
    pub bg_audio_playing: Mutable<bool>,
}

impl State {
    pub fn new(
        jig_id: JigId,
        _module_id: Option<ModuleId>,
        player_options: JigPlayerOptions,
    ) -> Self {
        Self {
            jig_id,
            jig: Mutable::new(None),
            loader: AsyncLoader::new(),
            active_module: Mutable::new(0),
            module_id: Mutable::new(None),
            timer: Mutable::new(None),
            points: Mutable::new(0),
            iframe: Rc::new(RefCell::new(None)),
            paused: Mutable::new(false),
            done: Mutable::new(false),
            player_options,
            bg_audio_handle: Rc::new(RefCell::new(None)),
            bg_audio_playing: Mutable::new(false),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlayerOptions {
    settings: JigPlayerSettings,
    is_student: bool,
}
