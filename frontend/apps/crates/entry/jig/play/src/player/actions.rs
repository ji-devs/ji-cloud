use std::rc::Rc;

use dominator::clone;
use futures_signals::signal::SignalExt;
use shared::{api::{ApiEndpoint, endpoints::jig}, domain::jig::JigResponse, error::EmptyError};
use utils::{iframe::{IframeAction, JigToModuleMessage, ModuleToJigMessage}, prelude::{SETTINGS, api_with_auth}, routes::Route, unwrap::UnwrapJiExt};
use wasm_bindgen_futures::spawn_local;
use super::{timer::Timer, state::State};


pub fn load_jig(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        let path = jig::Get::PATH.replace("{id}", &state.jig_id.0.to_string());

        match api_with_auth::<JigResponse, EmptyError, ()>(&path, jig::Get::METHOD, None).await {
            Ok(resp) => {
                // state.active_module.set(Some(resp.jig.modules[0].clone()));
                state.jig.set(Some(resp.jig));
                state.active_module.set(1);
            },
            Err(_) => {},
        }
    }));
}

pub fn start_timer(state: Rc<State>, time: u32) {
    let timer = Timer::new(time);

    spawn_local(timer.time.signal().for_each(clone!(state => move|time| {
        if time == 0 {
            state.timer.set(None);
            sent_iframe_message(Rc::clone(&state), JigToModuleMessage::TimerDone);
        }
        async {}
    })));

    state.timer.set(Some(timer));
}



pub fn sent_iframe_message(state: Rc<State>, data: JigToModuleMessage) {
    let iframe_origin: String = Route::Home.into();
    let iframe_origin = unsafe {
        SETTINGS.get_unchecked()
            .remote_target
            .spa_iframe(&iframe_origin)
    };

    match &*state.iframe.borrow() {
        None => todo!(),
        Some(iframe) => {
            let m = IframeAction::new(data);
            let _ = iframe.content_window().unwrap_ji().post_message(&m.into(), &iframe_origin);
        },
    };
}

pub fn on_iframe_message(state: Rc<State>, message: ModuleToJigMessage) {
    match message {
        ModuleToJigMessage::AddPoints(amount) => {
            let mut points = state.points.lock_mut();
            *points += amount;
        },
        ModuleToJigMessage::StartTimer(time) => {
            start_timer(Rc::clone(&state), time);
        },
    };
}
