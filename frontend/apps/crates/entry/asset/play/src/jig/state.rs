use std::{cell::RefCell, rc::Rc};

use awsm_web::loaders::helpers::AsyncLoader;
use components::audio::mixer::AudioHandle;
use futures_signals::signal::Mutable;
use serde::{Deserialize, Serialize};
use shared::domain::{
    jig::{player::PlayerNavigationHandler, JigId, JigPlayerSettings, JigResponse},
    meta::ResourceType,
    module::{
        body::{Audio, Instructions as ModuleInstructions, InstructionsType},
        ModuleId,
    },
};
use utils::asset::JigPlayerOptions;
use web_sys::HtmlIFrameElement;

use super::timer::Timer;

pub struct JigPlayer {
    pub jig_id: JigId,
    pub jig: Mutable<Option<JigResponse>>,
    /// Loaded after [`State`] is initialized necessitating an Option
    pub jig_liked: Mutable<Option<bool>>,
    pub loader: AsyncLoader,
    pub active_module: Mutable<Option<usize>>,
    /// Count of modules which have been played
    pub played_modules: RefCell<usize>,
    pub play_tracked: RefCell<bool>,
    pub start_module_id: Option<ModuleId>,
    pub navigation_handler: Mutable<Option<PlayerNavigationHandler>>,
    pub timer: Mutable<Option<Timer>>,
    pub points: Mutable<u32>,
    pub iframe: Rc<RefCell<Option<HtmlIFrameElement>>>,
    /// Whether this activity has started (via clicking Play button, or automatically).
    ///
    /// Note: *Not* related to the paused field.
    pub started: Mutable<bool>,
    pub paused: Mutable<bool>,
    pub done: Mutable<bool>,
    pub player_options: JigPlayerOptions,
    pub bg_audio_handle: Rc<RefCell<Option<AudioHandle>>>,
    pub bg_audio_playing: Mutable<bool>,
    pub instructions_audio_handle: Rc<RefCell<Option<AudioHandle>>>,
    pub resource_types: Mutable<Vec<ResourceType>>,
    pub instructions: Mutable<Option<Instructions>>,
    pub instructions_visible: Mutable<bool>,
}

impl JigPlayer {
    pub fn new(
        jig_id: JigId,
        module_id: Option<ModuleId>,
        player_options: JigPlayerOptions,
    ) -> Rc<Self> {
        let active_module = match module_id {
            // If the module_id is specified, then we need to make sure that we don't unecessarily load
            // the first module;
            Some(_) => Mutable::new(None),
            // Otherwise, if no module_id is set, then set the active module to the first module.
            None => Mutable::new(Some(0)),
        };

        Rc::new(Self {
            jig_id,
            jig: Mutable::new(None),
            jig_liked: Mutable::new(None),
            loader: AsyncLoader::new(),
            active_module,
            played_modules: RefCell::new(0),
            play_tracked: RefCell::new(false),
            start_module_id: module_id,
            timer: Mutable::new(None),
            navigation_handler: Mutable::new(None),
            points: Mutable::new(0),
            iframe: Rc::new(RefCell::new(None)),
            started: Mutable::new(false),
            paused: Mutable::new(false),
            done: Mutable::new(false),
            player_options,
            bg_audio_handle: Rc::new(RefCell::new(None)),
            bg_audio_playing: Mutable::new(true),
            instructions_audio_handle: Rc::new(RefCell::new(None)),
            resource_types: Default::default(),
            instructions: Mutable::new(None),
            instructions_visible: Mutable::new(false),
        })
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
    match utils::init::user::get_user_id() {
        Some(user_id) if jig.jig_data.draft_or_live.is_live() => match jig.author_id {
            Some(author_id) => author_id != user_id,
            None => true,
        },
        _ => false, // No logged-in user
    }
}

#[derive(Debug, Clone)]
pub struct Instructions {
    pub text: Option<String>,
    pub audio: Option<Audio>,
    pub instructions_type: InstructionsType,
}

impl Instructions {
    pub fn from_instructions(
        instructions: ModuleInstructions,
        instructions_type: InstructionsType,
    ) -> Self {
        Self {
            text: instructions.text,
            audio: instructions.audio,
            instructions_type,
        }
    }
}
