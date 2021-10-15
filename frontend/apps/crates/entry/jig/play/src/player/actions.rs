use std::rc::Rc;

use super::{state::State, timer::Timer};
use awsm_web::audio::AudioClipOptions;
use components::audio::mixer::{AudioSourceExt, AUDIO_MIXER};
use dominator::clone;
use futures_signals::signal::SignalExt;
use shared::{
    api::{endpoints::jig, ApiEndpoint},
    domain::jig::{AudioBackground, JigResponse},
    error::EmptyError,
};
use utils::{
    iframe::{IframeAction, JigToModulePlayerMessage, ModuleToJigPlayerMessage},
    prelude::{api_no_auth, SETTINGS},
    routes::{HomeRoute, Route},
    unwrap::UnwrapJiExt,
};
use wasm_bindgen_futures::spawn_local;

pub fn toggle_background_audio(state: Rc<State>, background_audio: AudioBackground) {
    let mut bg_audio_handle = state.bg_audio_handle.borrow_mut();

    match &*bg_audio_handle {
        Some(bg_audio_handle) => {
            if state.bg_audio_playing.get() {
                bg_audio_handle.pause();
                state.bg_audio_playing.set(false);
            } else {
                bg_audio_handle.play();
                state.bg_audio_playing.set(true);
            };
        }
        None => {
            let handle = AUDIO_MIXER.with(|mixer| {
                mixer.add_source(
                    background_audio.as_source(),
                    AudioClipOptions {
                        auto_play: true,
                        is_loop: true,
                        on_ended: None::<fn()>,
                    },
                )
            });

            *bg_audio_handle = Some(handle);
            state.bg_audio_playing.set(true);
        }
    };
}

pub fn navigate_forward(state: Rc<State>) {
    let mut active_module = state.active_module.lock_mut();
    if let Some(jig) = &*state.jig.lock_ref() {
        if *active_module < jig.jig_data.modules.len() - 1 {
            *active_module += 1;
            state.timer.set(None);
        } else {
            state.done.set(true);
        }
    }
}

pub fn navigate_back(state: Rc<State>) {
    let mut active_module = state.active_module.lock_mut();
    if *active_module != 0 {
        *active_module -= 1;
        state.timer.set(None);
    }
}

pub fn load_jig(state: Rc<State>) {
    state.loader.load(clone!(state => async move {

        let resp = match state.player_options.draft {
            false => {
                let path = jig::GetLive::PATH.replace("{id}", &state.jig_id.0.to_string());
                api_no_auth::<JigResponse, EmptyError, ()>(&path, jig::GetLive::METHOD, None).await
            },
            true => {
                let path = jig::GetDraft::PATH.replace("{id}", &state.jig_id.0.to_string());
                api_no_auth::<JigResponse, EmptyError, ()>(&path, jig::GetDraft::METHOD, None).await
            },
        };

        match resp {
            Ok(resp) => {
                // state.active_module.set(Some(resp.jig.modules[0].clone()));
                state.jig.set(Some(resp));
            },
            Err(_) => {},
        }
    }));
}

pub fn start_timer(state: Rc<State>, time: u32) {
    let timer = Timer::new(time);

    spawn_local(timer.time.signal().for_each(clone!(state => move|time| {
        if time == 0 {
            sent_iframe_message(Rc::clone(&state), JigToModulePlayerMessage::TimerDone);
        }
        async {}
    })));

    state.timer.set(Some(timer));
}

pub fn toggle_paused(state: Rc<State>) {
    let paused = !state.paused.get();

    // set state to paused
    state.paused.set(paused);

    // pause timer if exists
    match &*state.timer.lock_ref() {
        None => {}
        Some(timer) => {
            *timer.paused.borrow_mut() = paused;
        }
    }

    // let iframe know that paused
    let iframe_message = match paused {
        false => JigToModulePlayerMessage::Play,
        true => JigToModulePlayerMessage::Pause,
    };
    sent_iframe_message(Rc::clone(&state), iframe_message);
}

pub fn sent_iframe_message(state: Rc<State>, data: JigToModulePlayerMessage) {
    let iframe_origin: String = Route::Home(HomeRoute::Home).into();
    let iframe_origin = unsafe {
        SETTINGS
            .get_unchecked()
            .remote_target
            .spa_iframe(&iframe_origin)
    };

    match &*state.iframe.borrow() {
        None => todo!(),
        Some(iframe) => {
            let m = IframeAction::new(data);
            let _ = iframe
                .content_window()
                .unwrap_ji()
                .post_message(&m.into(), &iframe_origin);
        }
    };
}

pub fn on_iframe_message(state: Rc<State>, message: ModuleToJigPlayerMessage) {
    match message {
        ModuleToJigPlayerMessage::AddPoints(amount) => {
            let mut points = state.points.lock_mut();
            *points += amount;
        }
        ModuleToJigPlayerMessage::Start(time) => {
            if let Some(time) = time {
                start_timer(Rc::clone(&state), time);
            }
        }
        ModuleToJigPlayerMessage::Next => {
            navigate_forward(Rc::clone(&state));
        }
        ModuleToJigPlayerMessage::Stop => {
            state.timer.set(None);
        }
    };
}

pub fn reload_iframe(state: Rc<State>) {
    match &*state.iframe.borrow() {
        None => {}
        Some(iframe) => {
            iframe.set_src(&iframe.src());
            state.timer.set(None);
        }
    };
}
