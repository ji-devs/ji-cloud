use super::state::*;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use shared::{
    api::endpoints::{self, ApiEndpoint},
    domain::jig::{module::ModuleId, Jig, JigId, JigPlayerSettings, JigResponse, JigUpdateRequest},
    error::EmptyError,
};
use std::cell::RefCell;
use std::rc::Rc;
use utils::prelude::*;

pub async fn load_jig(jig_id: JigId, jig_cell: Rc<RefCell<Option<Jig>>>) {
    let path = endpoints::jig::Get::PATH.replace("{id}", &jig_id.0.to_string());

    match api_with_auth::<JigResponse, EmptyError, ()>(&path, endpoints::jig::Get::METHOD, None)
        .await
    {
        Ok(resp) => {
            *jig_cell.borrow_mut() = Some(resp.jig);
        }
        Err(_) => {}
    }
}

pub fn navigate_to_publish(state: Rc<State>) {
    state.route.set_neq(JigEditRoute::Publish);
    state.collapsed.set(true);

    let jig_id = state.jig.id;
    let url: String = Route::Jig(JigRoute::Edit(jig_id, JigEditRoute::Publish)).into();
    log::info!("{}", url);

    /* this will cause a full refresh - but preserves history
     * see the .future in EditPage too
        dominator::routing::go_to_url(&url);
     */
}

pub async fn update_jig(jig_id: &JigId, req: JigUpdateRequest) -> Result<(), EmptyError> {
    let path = endpoints::jig::Update::PATH.replace("{id}", &jig_id.0.to_string());
    api_with_auth_empty::<EmptyError, _>(&path, endpoints::jig::Update::METHOD, Some(req)).await
}

pub fn update_display_name(state: Rc<State>, value: String) {
    state.loader.load(clone!(state => async move {
        state.name.set(value.clone());

        let req = JigUpdateRequest {
            display_name: Some(value),
            ..JigUpdateRequest::default()
        };

        match update_jig(&state.jig.id, req).await {
            Ok(_) => {},
            Err(_) => {},
        }
    }));
}

pub fn duplicate_module(state: Rc<State>, module_id: &ModuleId) {
    state.loader.load(clone!(state, module_id => async move {
        let module = super::module_cloner::clone_module(&state.jig.id, &module_id, &state.jig.id).await.unwrap_ji();
        state.modules.lock_mut().push_cloned(Rc::new(Some(module)));
    }));
}

pub fn player_settings_change_signal(state: Rc<State>) -> impl Signal<Item = JigPlayerSettings> {
    let sig = map_ref! {
        let direction = state.settings.direction.signal_cloned(),
        let display_score = state.settings.display_score.signal(),
        let track_assessments = state.settings.track_assessments.signal(),
        let drag_assist = state.settings.drag_assist.signal()
        => ( direction.clone(), display_score.clone(), track_assessments.clone(), drag_assist.clone())
    };

    sig.map(
        |(direction, display_score, track_assessments, drag_assist)| JigPlayerSettings {
            direction: direction.clone(),
            display_score: display_score.clone(),
            track_assessments: track_assessments.clone(),
            drag_assist: drag_assist.clone(),
        },
    )
}
