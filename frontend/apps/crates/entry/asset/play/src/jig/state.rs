use std::{cell::RefCell, collections::HashMap, rc::Rc};

use awsm_web::loaders::helpers::AsyncLoader;
use components::audio::mixer::AudioHandle;
use futures_signals::signal::Mutable;
use shared::domain::{
    asset::DraftOrLive,
    category::{Category, CategoryId},
    jig::{
        codes::JigPlaySession, player::PlayerNavigationHandler, JigId, JigResponse, TextDirection,
    },
    meta::ResourceType,
    module::{
        body::{Audio, ModuleAssist, ModuleAssistType},
        ModuleId,
    },
    playlist::PlaylistResponse,
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
    pub bg_audio_handle: Rc<RefCell<Option<AudioHandle>>>,
    pub bg_audio_playing: Mutable<bool>,
    pub module_assist_audio_handle: Rc<RefCell<Option<AudioHandle>>>,
    pub resource_types: Mutable<Vec<ResourceType>>,
    pub categories: Mutable<Vec<Category>>,
    pub category_label_lookup: Mutable<HashMap<CategoryId, String>>,
    pub playlists: Mutable<Vec<PlaylistResponse>>,
    pub module_assist: Mutable<Option<PlayModuleAssist>>,
    pub module_assist_visible: Mutable<bool>,
    pub is_full_screen: Mutable<bool>,
    pub session_info: RefCell<JigPlaySession>,
    pub play_login_popup_shown: Mutable<bool>,
    pub draft_or_live: DraftOrLive,
    pub play_token: Option<String>,
    pub players_name: Option<String>,
    pub is_student: bool,
    pub quota: bool,
    pub direction: Mutable<TextDirection>,
    pub scoring: Mutable<bool>,
    pub drag_assist: Mutable<bool>,
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
            bg_audio_handle: Rc::new(RefCell::new(None)),
            bg_audio_playing: Mutable::new(true),
            module_assist_audio_handle: Rc::new(RefCell::new(None)),
            resource_types: Default::default(),
            categories: Default::default(),
            category_label_lookup: Default::default(),
            playlists: Default::default(),
            module_assist: Mutable::new(None),
            module_assist_visible: Mutable::new(false),
            is_full_screen: Mutable::new(false),
            session_info: Default::default(),
            play_login_popup_shown: Mutable::new(false),
            draft_or_live: player_options.draft_or_live,
            play_token: player_options.play_token,
            players_name: player_options.players_name,
            is_student: player_options.is_student,
            quota: player_options.quota,
            direction: Mutable::new(player_options.direction.unwrap_or_default()),
            scoring: Mutable::new(player_options.scoring.unwrap_or_default()),
            drag_assist: Mutable::new(player_options.drag_assist.unwrap_or_default()),
        })
    }
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
pub struct PlayModuleAssist {
    pub text: Option<String>,
    pub audio: Option<Audio>,
    pub always_show: bool,
    pub module_assist_type: ModuleAssistType,
}

impl PlayModuleAssist {
    pub fn from_module_assist(
        module_assist: ModuleAssist,
        module_assist_type: ModuleAssistType,
    ) -> Self {
        Self {
            text: module_assist.text,
            audio: module_assist.audio,
            always_show: module_assist.always_show,
            module_assist_type,
        }
    }
}
