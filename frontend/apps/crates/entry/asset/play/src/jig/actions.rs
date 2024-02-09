use std::{collections::HashMap, rc::Rc};

use super::{
    restrictions::{self, Restricted},
    state::{can_load_liked_status, JigPlayer, PlayModuleAssist},
    timer::Timer,
};
use components::audio::mixer::{AudioHandle, AUDIO_MIXER};
use dominator::clone;
use futures_signals::signal::SignalExt;
use shared::domain::{
    category::{Category, CategoryId, CategoryTreeScope, GetCategoryRequest},
    jig::GetJigPlaylistsPath,
};
use shared::{
    api::endpoints::{self, jig},
    domain::{
        asset::DraftOrLive,
        category::GetCategoryPath,
        jig::{
            codes::instance::{
                PlayerSessionInstanceCompletePath, PlayerSessionInstanceCompleteRequest,
            },
            player::{ModuleConfig, PlayerNavigationHandler, Seconds},
            AudioBackground, JigGetDraftPath, JigGetLivePath, JigLikedPath, JigPlayPath,
            TextDirection,
        },
        meta::GetMetadataPath,
        module::{
            body::{ModuleAssist, ModuleAssistType},
            ModuleId,
        },
    },
};
use utils::{
    bail_on_err,
    iframe::{
        AssetPlayerToPlayerPopup, IframeAction, IframeMessageExt, JigToModulePlayerMessage,
        ModuleToJigPlayerMessage,
    },
    keyboard::{Key, KeyEvent},
    paywall,
    prelude::{ApiEndpointExt, SETTINGS},
    routes::{HomeRoute, Route},
    unwrap::UnwrapJiExt,
};
use wasm_bindgen_futures::spawn_local;

const TRACK_MODULE_COUNT: usize = 3;

impl JigPlayer {
    pub fn toggle_background_audio(self: &Rc<Self>) {
        let bg_audio_handle = self.bg_audio_handle.borrow();

        match &*bg_audio_handle {
            Some(bg_audio_handle) => {
                if self.bg_audio_playing.get() {
                    self.pause_background_audio(bg_audio_handle);
                } else {
                    self.play_background_audio(bg_audio_handle);
                };
            }
            None => {}
        };
    }

    pub fn play_background_audio(self: &Rc<Self>, audio_handle: &AudioHandle) {
        audio_handle.play();
        self.bg_audio_playing.set(true);
    }

    pub fn pause_background_audio(self: &Rc<Self>, audio_handle: &AudioHandle) {
        audio_handle.pause();
        self.bg_audio_playing.set(false);
    }

    pub fn navigate_forward_or_handle(self: &Rc<Self>) {
        match self.navigation_handler.get_cloned() {
            Some(PlayerNavigationHandler::Module) => {
                self.send_iframe_message(JigToModulePlayerMessage::Next);
            }
            _ => self.navigate_forward(),
        }
    }

    pub fn navigate_forward(self: &Rc<Self>) {
        let state = self;
        if let Some(active_module) = state.active_module.get() {
            let module_count = (&*state.jig.lock_ref())
                .as_ref()
                .map(|jig| jig.jig_data.modules.len());

            if let Some(module_count) = module_count {
                let is_done = active_module == module_count - 1;
                // Track that the JIG has been played if
                // - The JIG is not a draft;
                // - The play hasn't been tracked yet;
                // - Either TRACK_MODULE_COUNT count of modules have been played or the JIG is done.
                let should_track = !state.player_options.draft_or_live.is_draft()
                    && !*state.play_tracked.borrow()
                    && (*state.played_modules.borrow() + 1 == TRACK_MODULE_COUNT || is_done);

                if should_track {
                    state.loader.load(clone!(state => async move {
                        // We don't need to handle an Ok Result; We can ignore Err, nothing is dependent on the
                        // success of this call. The failure should be noted in the server logs.
                        let _ = jig::Play::api_no_auth(
                            JigPlayPath(state.jig_id),
                            None,
                        ).await;

                        // Set the flag to indicate that the play has been tracked for this JIG.
                        *state.play_tracked.borrow_mut() = true;
                    }))
                }

                if !is_done {
                    // Only increment the played count when navigating to the _next_ module.
                    let mut played_modules = state.played_modules.borrow_mut();
                    *played_modules += 1;

                    state.navigate_to_index(active_module + 1);
                } else {
                    state.finish();
                }
            }
        }
    }

    pub fn finish(self: &Rc<Self>) {
        let state = self;
        state.done.set(true);
        if let Some(token) = state.player_options.play_token.clone() {
            spawn_local(clone!(state => async move {
                let req = PlayerSessionInstanceCompleteRequest {
                    token,
                    session: state.session_info.borrow().clone(),
                };
                let res = endpoints::jig::codes::instance::Complete::api_with_auth(PlayerSessionInstanceCompletePath(), Some(req)).await;
                let _ = bail_on_err!(res);
            }));
        }
    }

    pub fn navigate_back_or_handle(self: &Rc<Self>) {
        match self.navigation_handler.get_cloned() {
            Some(PlayerNavigationHandler::Module) => {
                self.send_iframe_message(JigToModulePlayerMessage::Previous);
            }
            _ => self.navigate_back(),
        }
    }

    pub fn navigate_back(self: &Rc<Self>) {
        if let Some(active_module) = self.active_module.get() {
            if active_module != 0 {
                self.navigate_to_index(active_module - 1);
            }
        }
    }

    pub fn navigate_from_keyboard_event(self: &Rc<Self>, key_event: KeyEvent) {
        if let Some(jig) = self.jig.get_cloned() {
            let direction = jig.jig_data.default_player_settings.direction;
            match direction {
                TextDirection::LeftToRight => match key_event.key {
                    Key::ArrowLeft => self.navigate_back_or_handle(),
                    Key::ArrowRight => self.navigate_forward_or_handle(),
                    _ => {}
                },
                TextDirection::RightToLeft => match key_event.key {
                    Key::ArrowRight => self.navigate_back_or_handle(),
                    Key::ArrowLeft => self.navigate_forward_or_handle(),
                    _ => {}
                },
            }
        }
    }

    pub fn navigate_to_index(self: &Rc<Self>, index: usize) {
        self.active_module.set(Some(index));
        self.timer.set(None);
        self.done.set(false);
        self.set_paused(false);
    }

    pub fn navigate_to_module(self: &Rc<Self>, module_id: &ModuleId) {
        if let Some(jig) = &*self.jig.lock_ref() {
            let index = jig
                .jig_data
                .modules
                .iter()
                .position(|module| &module.id == module_id);

            if let Some(index) = index {
                self.navigate_to_index(index);
            }
        }
    }

    pub fn set_module_assist(
        self: &Rc<Self>,
        module_assist: Option<(ModuleAssist, ModuleAssistType)>,
    ) {
        let state = self;
        // Only set the module assist field if the module assist has content. Otherwise, leave it at None.
        let module_assist = match module_assist {
            Some((module_assist, module_assist_type)) if module_assist.has_content() => {
                Some((module_assist, module_assist_type))
            }
            _ => None,
        };

        state.module_assist_visible.set_neq(false);
        *state.module_assist_audio_handle.borrow_mut() = None;

        state
            .module_assist
            .set(module_assist.map(|(module_assist, module_assist_type)| {
                PlayModuleAssist::from_module_assist(module_assist, module_assist_type)
            }));
    }

    pub fn show_assist(self: &Rc<Self>, visible: bool) {
        if let Some(module_assist) = self.module_assist.get_cloned() {
            self.module_assist_visible.set_neq(visible);
            self.set_timer_paused(visible);

            if visible {
                self.play_assist_audio();
            } else {
                // Always drop the audio handle whenever the popup is hidden
                *self.module_assist_audio_handle.borrow_mut() = None;

                if module_assist.module_assist_type.is_feedback() {
                    // Clear the assist to prevent any audio possibly playing again.
                    self.set_module_assist(None);
                }
                self.module_assist_done(module_assist);
            }
        }
    }

    pub fn play_assist_audio(self: &Rc<Self>) {
        let state = self;
        if let Some(module_assist) = state.module_assist.get_cloned() {
            if let Some(audio) = &module_assist.audio {
                *state.module_assist_audio_handle.borrow_mut() = Some(AUDIO_MIXER.with(clone!(state, module_assist => move |mixer| mixer
                    .play_on_ended(audio.into(), false, clone!(state => move || {
                        if module_assist.module_assist_type.is_feedback() && module_assist.text.is_none() {
                            // Clear the assist to prevent any audio possibly playing again. But only if this is Feedbaack and
                            // there is no text
                            state.set_module_assist(None);
                        }

                        // For the `Instructions` variant, we display a default text in the popup when a _timer_ is set. In that case
                        // we don't want to fire the done event when audio completes.
                        if !((state.timer.get_cloned().is_some() || module_assist.always_show) && module_assist.module_assist_type.is_instructions()) && module_assist.text.is_none() {
                            // If there is no text, then we can notify the activity that the assist audio has completed.
                            state.module_assist_done(module_assist.clone());
                        }
                    }))
                )));
            }
        }
    }

    fn module_assist_done(self: &Rc<Self>, play_module_assist: PlayModuleAssist) {
        let module_assist_type = play_module_assist.module_assist_type;
        self.send_iframe_message(JigToModulePlayerMessage::ModuleAssistDone(
            module_assist_type,
        ));
    }

    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        if state.player_options.quota {
            if let Some(restricted) = restrictions::play_restricted() {
                match restricted {
                    Restricted::FreeAccountLimit => {
                        paywall::dialog_play(restrictions::FREE_ACCOUNT_LIMIT_MESSAGE);
                    }
                    Restricted::NoAccountLimit => {
                        self.play_login_popup_shown.set(true);
                    }
                }
            } else {
                restrictions::increase_played_count();
            }
        }

        state.loader.load(clone!(state => async move {
            state.load_categories().await;
            state.load_resource_types().await;
            state.load_jig_playlists().await;
            state.load_jig().await;
        }));
    }

    async fn load_jig(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let (jig, jig_liked) = match state.player_options.draft_or_live {
                DraftOrLive::Live => {
                    let jig = {
                        jig::GetLive::api_no_auth(JigGetLivePath(state.jig_id), None).await
                    };

                    // Fetch whether the current user has liked this JIG.
                    // TODO: now included in JigResponse
                    let jig_liked = {
                        match &jig {
                            // Only fetch liked status if the jig request didn't error, the user is
                            // logged in and the user is not the author of the JIG.
                            Ok(jig) if can_load_liked_status(jig) => {
                                jig::Liked::api_with_auth(JigLikedPath(state.jig_id), None)
                                    .await
                                    .map_or(false, |r| r.is_liked)
                            },
                            _ => false
                        }
                    };

                    (jig, jig_liked)
                },
                DraftOrLive::Draft => {
                    let jig = {
                        jig::GetDraft::api_no_auth(JigGetDraftPath(state.jig_id), None).await
                    };

                    (jig, false)
                },
            };

            match jig {
                Ok(jig) => {
                    if !state.player_options.is_student && !paywall::can_play_jig(jig.admin_data.premium) {
                        paywall::dialog_play("
                            Looking to access our premium content?
                            Upgrade now for UNLIMITED JIGs and resources.
                        ");
                        return;
                    }

                    // state.active_module.set(Some(resp.jig.modules[0].clone()));
                    if let Some(start_module_id) = state.start_module_id {
                        if let Some((index, _)) = jig.jig_data.modules.iter().enumerate().find(|module| {
                            module.1.id == start_module_id
                        }) {
                            state.active_module.set_neq(Some(index));
                        };
                    }
                    state.jig.set(Some(jig));
                    state.jig_liked.set(Some(jig_liked));
                },
                Err(_) => {
                    todo!();
                },
            }
        }));
    }

    async fn load_resource_types(self: &Rc<Self>) {
        match endpoints::meta::Get::api_with_auth(GetMetadataPath(), None).await {
            Err(_) => todo!(),
            Ok(meta) => {
                self.resource_types.set(meta.resource_types);
            }
        };
    }

    async fn load_categories(self: &Rc<Self>) {
        log::warn!("load categories");

        let req = GetCategoryRequest {
            ids: Vec::new(),
            scope: Some(CategoryTreeScope::Descendants),
        };

        match endpoints::category::Get::api_with_auth(GetCategoryPath(), Some(req)).await {
            Err(_) => todo!(),
            Ok(categories) => {
                self.categories.set(categories.categories.to_owned());
                let mut category_label_lookup = HashMap::new();
                get_categories_labels(
                    &categories.categories.to_owned(),
                    &mut category_label_lookup,
                    "",
                );
                self.category_label_lookup.set(category_label_lookup)
            }
        };
    }

    pub(crate) async fn load_jig_playlists(self: &Rc<Self>) {
        match endpoints::jig::GetJigPlaylists::api_no_auth(GetJigPlaylistsPath(self.jig_id), None)
            .await
        {
            Err(_) => todo!(),
            Ok(resp) => {
                log::warn!("inside call");

                self.playlists.set(resp.playlists);
            }
        };
    }

    fn init_audio(self: &Rc<Self>, background_audio: AudioBackground) {
        let handle = AUDIO_MIXER.with(move |mixer| mixer.play(background_audio.into(), true));

        let mut bg_audio_handle = self.bg_audio_handle.borrow_mut();
        *bg_audio_handle = Some(handle);
        self.bg_audio_playing.set(true);
    }

    pub fn start_timer(self: &Rc<Self>, time: Seconds) {
        let state = self;
        let timer = Timer::new(*time);

        spawn_local(timer.time.signal().for_each(clone!(state => move|time| {
            if time == 0 {
                state.send_iframe_message(JigToModulePlayerMessage::TimerDone);
            }
            async {}
        })));

        self.timer.set(Some(timer));
    }

    pub fn toggle_paused(self: &Rc<Self>) {
        let paused = !self.paused.get();
        self.set_paused(paused);
    }

    pub fn set_paused(self: &Rc<Self>, paused: bool) {
        self.paused.set(paused);

        self.set_timer_paused(paused);

        let bg_audio_handle = self.bg_audio_handle.borrow();
        let expectations = match &*bg_audio_handle {
            Some(bg_handle) => vec![bg_handle],
            None => vec![],
        };

        AUDIO_MIXER.with(|mixer| {
            match paused {
                true => mixer.pause_all_except(&expectations[..]),
                false => mixer.play_all_except(&expectations[..]),
            };
        });
    }

    fn set_timer_paused(self: &Rc<Self>, paused: bool) {
        match &*self.timer.lock_ref() {
            None => {}
            Some(timer) => {
                *timer.paused.borrow_mut() = paused;
            }
        }
    }

    pub fn send_iframe_message(self: &Rc<Self>, data: JigToModulePlayerMessage) {
        let iframe_origin: String = Route::Home(HomeRoute::Home).into();
        let iframe_origin = SETTINGS
            .get()
            .unwrap_ji()
            .remote_target
            .spa_iframe(&iframe_origin);

        match &*self.iframe.borrow() {
            None => {
                // Do nothing - we cannot send a message to an iframe which does not exist yet.
            }
            Some(iframe) => {
                let m = IframeAction::new(data);
                let _ = iframe
                    .content_window()
                    .unwrap_ji()
                    .post_message(&m.into(), &iframe_origin);
            }
        };
    }

    pub fn on_iframe_message(self: &Rc<Self>, message: ModuleToJigPlayerMessage) {
        match message {
            ModuleToJigPlayerMessage::AddPoints(amount) => {
                let mut points = self.points.lock_mut();
                *points += amount;
            }
            ModuleToJigPlayerMessage::Start(config) => {
                self.start_player(config);
            }
            ModuleToJigPlayerMessage::ResetTimer(time) => {
                self.start_timer(time);
            }
            ModuleToJigPlayerMessage::PauseTimer => {
                self.set_timer_paused(true);
            }
            ModuleToJigPlayerMessage::UnpauseTimer => {
                self.set_timer_paused(false);
            }
            ModuleToJigPlayerMessage::Previous => {
                self.navigation_handler.set(None);
                self.navigate_back();
            }
            ModuleToJigPlayerMessage::AddCodeSessionInfo(info) => {
                self.session_info.borrow_mut().modules.push(info);
            }
            ModuleToJigPlayerMessage::Next => {
                self.navigation_handler.set(None);
                self.navigate_forward();
            }
            ModuleToJigPlayerMessage::JumpToIndex(index) => {
                self.navigate_to_index(index);
            }
            ModuleToJigPlayerMessage::JumpToId(module_id) => {
                self.navigate_to_module(&module_id);
            }
            ModuleToJigPlayerMessage::ModuleAssist(module_assist) => {
                self.set_module_assist(module_assist);
            }
            ModuleToJigPlayerMessage::KeyEvent(key_event) => {
                self.navigate_from_keyboard_event(key_event)
            }
        };
    }

    fn start_player(self: &Rc<Self>, config: ModuleConfig) {
        // If bg audio is not yet set (i.e. first module to be ready) initialize the audio once the jig is started
        if self.bg_audio_handle.borrow().is_none() {
            if let Some(jig) = self.jig.get_cloned() {
                match jig.jig_data.audio_background {
                    Some(audio_background) => {
                        self.init_audio(audio_background);
                    }
                    None => {
                        AUDIO_MIXER.with(move |mixer| {
                            mixer.init_silently();
                        });
                    }
                }
            }
        }

        // If the background audio is set to play, then start the audio
        if self.bg_audio_playing.get() {
            if let Some(bg_audio_handle) = &*self.bg_audio_handle.borrow() {
                self.play_background_audio(bg_audio_handle);
            }
        }

        if let Some(time) = config.timer {
            self.start_timer(time);
        }

        self.navigation_handler.set(Some(config.navigation_handler));

        self.started.set_neq(true);
    }

    pub fn reload_iframe(self: &Rc<Self>) {
        match &*self.iframe.borrow() {
            None => {}
            Some(iframe) => {
                iframe.set_src(&iframe.src());
                self.timer.set(None);
            }
        };
    }

    pub fn close_player(&self) {
        let _ = IframeAction::new(AssetPlayerToPlayerPopup::Close).try_post_message_to_parent();
    }
}

fn get_categories_labels(
    categories: &Vec<Category>,
    lookup: &mut HashMap<CategoryId, String>,
    base_name: &str,
) {
    for category in categories {
        let name = format!("{}{}", base_name, category.name);
        lookup.insert(category.id, name.clone());
        let base_name = name + "/";
        get_categories_labels(&category.children, lookup, &base_name);
    }
}
