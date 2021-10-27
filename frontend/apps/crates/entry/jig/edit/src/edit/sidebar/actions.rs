use super::state::*;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use shared::{
    api::endpoints::{self, ApiEndpoint},
    domain::{
        jig::{
            module::{ModuleCreateRequest, ModuleGetQuery, ModuleId, ModuleResponse},
            JigId, JigPlayerSettings, JigResponse, JigUpdateDraftDataRequest, LiteModule,
            ModuleKind,
        },
        CreateResponse,
    },
    error::EmptyError,
};
use std::cell::RefCell;
use std::rc::Rc;
use utils::{iframe::ModuleToJigEditorMessage, jig::JigPlayerOptions, prelude::*};

pub async fn load_jig(jig_id: JigId, jig_cell: Rc<RefCell<Option<JigResponse>>>) {
    let path = endpoints::jig::GetDraft::PATH.replace("{id}", &jig_id.0.to_string());

    match api_with_auth::<JigResponse, EmptyError, ()>(
        &path,
        endpoints::jig::GetDraft::METHOD,
        None,
    )
    .await
    {
        Ok(resp) => {
            *jig_cell.borrow_mut() = Some(resp);
        }
        Err(_) => {}
    }
}

pub fn navigate_to_publish(state: Rc<State>) {
    state.jig_edit_state.route.set_neq(JigEditRoute::Publish);
    state.collapsed.set(true);

    let jig_id = state.jig.id;
    Route::push_state(Route::Jig(JigRoute::Edit(jig_id, JigEditRoute::Publish)));
}

pub async fn update_jig(jig_id: &JigId, req: JigUpdateDraftDataRequest) -> Result<(), EmptyError> {
    let path = endpoints::jig::UpdateDraftData::PATH.replace("{id}", &jig_id.0.to_string());
    api_with_auth_empty::<EmptyError, _>(&path, endpoints::jig::UpdateDraftData::METHOD, Some(req))
        .await
}

pub fn update_display_name(state: Rc<State>, value: String) {
    state.loader.load(clone!(state => async move {
        state.name.set(value.clone());

        let req = JigUpdateDraftDataRequest {
            display_name: Some(value),
            ..Default::default()
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

pub fn get_player_settings(state: Rc<State>) -> JigPlayerOptions {
    let direction = state.settings.direction.get_cloned();
    let display_score = state.settings.display_score.get();
    let track_assessments = state.settings.track_assessments.get();
    let drag_assist = state.settings.drag_assist.get();

    JigPlayerOptions {
        direction: direction,
        display_score: display_score,
        track_assessments: track_assessments,
        drag_assist: drag_assist,
        is_student: false,
        draft: true,
    }
}

pub fn on_iframe_message(state: Rc<State>, message: ModuleToJigEditorMessage) {
    match message {
        ModuleToJigEditorMessage::AppendModule(module) => {
            populate_added_module(Rc::clone(&state), module);
        }
        ModuleToJigEditorMessage::Next => {
            state.collapsed.set(false);
        }
    }
}

fn populate_added_module(state: Rc<State>, module: LiteModule) {
    state
        .modules
        .lock_mut()
        .push_cloned(Rc::new(Some(module.clone())));
    state
        .jig_edit_state
        .route
        .set_neq(JigEditRoute::Module(module.id));
}

pub fn use_module_as(state: Rc<State>, target_kind: ModuleKind, source_module_id: ModuleId) {
    state.loader.load(clone!(state => async move {
        let target_module_id: Result<ModuleId, EmptyError> = async {
            let req = ModuleGetQuery { q: String::from("unique") };

            let path = endpoints::jig::module::GetDraft::PATH
                .replace("{id}", &state.jig.id.0.to_string())
                .replace("{module_id}", &source_module_id.0.to_string());

            let source_module = api_with_auth::<ModuleResponse, EmptyError, _>(
                &path,
                endpoints::jig::module::GetDraft::METHOD,
                Some(req)
            ).await?.module;

            let target_body = source_module.body.convert_to_body(target_kind).unwrap_ji();

            let path = endpoints::jig::module::Create::PATH
                .replace("{id}", &state.jig.id.0.to_string());

            let req = ModuleCreateRequest { body: target_body };

            let res = api_with_auth::<CreateResponse<ModuleId>, EmptyError, ModuleCreateRequest>(
                &path,
                endpoints::jig::module::Create::METHOD,
                Some(req),
            )
            .await?;

            Ok(res.id)
        }.await;

        match target_module_id {
            Err(_) => {
                log::error!("request to create module failed!");
            },
            Ok(target_module_id) => {
                let lite_module = LiteModule {
                    id: target_module_id,
                    kind: target_kind,
                };
                populate_added_module(Rc::clone(&state), lite_module);
            },
        };
    }));
}
