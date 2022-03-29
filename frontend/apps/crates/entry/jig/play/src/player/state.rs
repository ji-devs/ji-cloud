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
    /// Loaded after [`State`] is initialized necessitating an Option
    pub jig_liked: Mutable<Option<bool>>,
    pub loader: AsyncLoader,
    pub active_module: Mutable<usize>,
    /// Count of modules which have been played
    pub played_modules: RefCell<usize>,
    pub play_tracked: RefCell<bool>,
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
            jig_liked: Mutable::new(None),
            loader: AsyncLoader::new(),
            active_module: Mutable::new(0),
            played_modules: RefCell::new(0),
            play_tracked: RefCell::new(false),
            module_id: Mutable::new(None),
            timer: Mutable::new(None),
            points: Mutable::new(0),
            iframe: Rc::new(RefCell::new(None)),
            paused: Mutable::new(false),
            done: Mutable::new(false),
            player_options,
            bg_audio_handle: Rc::new(RefCell::new(None)),
            bg_audio_playing: Mutable::new(true),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlayerOptions {
    settings: JigPlayerSettings,
    is_student: bool,
}

/// Returns whether the liked status should be loaded for a JIG
///
/// Returns true only if there is a logged-in user who is **not** the author of the JIG, and the
/// JIG is published.
pub fn can_load_liked_status(jig: &JigResponse) -> bool {
    match utils::init::user::get_user() {
        Some(user) if jig.jig_data.draft_or_live.is_live() => match jig.author_id {
            Some(author_id) => author_id != user.id,
            None => true,
        },
        _ => false, // No logged-in user
    }
}
