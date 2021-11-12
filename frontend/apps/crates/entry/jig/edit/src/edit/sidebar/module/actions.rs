use super::state::State;
use crate::edit::sidebar::dragging::state::State as DragState;
use dominator::clone;
use shared::{
    api::endpoints::{self, ApiEndpoint},
    domain::{jig::module::*, jig::*, CreateResponse},
    error::EmptyError,
};
use std::convert::TryInto;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub fn on_module_kind_drop(state: Rc<State>, module_kind: ModuleKind) {
    if state.index == 0 {
        if module_kind == ModuleKind::Cover {
            fist_cover_dragged(Rc::clone(&state));
        } else {
            todo!("invalid module type");
        }
    } else {
        if state.module.is_none() {
            assign_kind(state.clone(), module_kind);

            // if this is the empty module at the end
            if state.sidebar.jig.jig_data.modules.len() == state.index {
                state.sidebar.modules.lock_mut().push_cloned(Rc::new(None));
            }
        }
    }
}

pub fn fist_cover_dragged(state: Rc<State>) {
    spawn_local(clone!(state => async move {
        let path = endpoints::jig::Cover::PATH.replace("{id}", &state.sidebar.jig.id.0.to_string());

        let res = api_with_auth_empty::<EmptyError, ()>(
            &path,
            endpoints::jig::Cover::METHOD,
            None
        ).await;
        match res {
            Err(_) => todo!(),
            Ok(_) => {
                let cover_id = match &*state.module {
                    None => unreachable!(),
                    Some(module) => {
                        module.id
                    },
                };

                state.sidebar.first_cover_assigned.set(true);
                state.sidebar.collapsed.set(true);
                state.sidebar.jig_edit_state.route.set(JigEditRoute::Module(cover_id.clone()));
                Route::push_state(Route::Jig(JigRoute::Edit(state.sidebar.jig.id, JigEditRoute::Module(cover_id))));
            },
        }
    }));
}

pub async fn update_module(
    jig_id: &JigId,
    module_id: &ModuleId,
    req: ModuleUpdateRequest,
) -> Result<(), EmptyError> {
    let path = endpoints::jig::module::Update::PATH
        .replace("{id}", &jig_id.0.to_string())
        .replace("{module_id}", &module_id.0.to_string());
    api_with_auth_empty::<EmptyError, _>(&path, endpoints::jig::module::Update::METHOD, Some(req))
        .await
}

#[allow(dead_code)] // this should be remove eventually
pub fn mouse_down(state: Rc<State>, x: i32, y: i32) {
    state
        .sidebar
        .drag
        .set(Some(Rc::new(DragState::new(state.clone(), x, y))));
}

pub fn edit(state: Rc<State>) {
    if let Some(module) = &*state.module {
        let module_id = module.id;
        state
            .sidebar
            .jig_edit_state
            .route
            .set_neq(JigEditRoute::Module(module_id));
        state.sidebar.collapsed.set(true);

        let jig_id = state.sidebar.jig.id;
        Route::push_state(Route::Jig(JigRoute::Edit(
            jig_id,
            JigEditRoute::Module(module_id),
        )));
    }
}

pub fn delete(state: Rc<State>) {
    let index = state.index;

    state.sidebar.loader.load(clone!(state => async move {
        if let Some(module) = &*state.module {
            let path = endpoints::jig::module::Delete::PATH
                .replace("{id}", &state.sidebar.jig.id.0.to_string());

            let req = ModuleDeleteRequest {
                id: StableOrUniqueId::Unique(module.id)
            };

            match api_with_auth_empty::<EmptyError, _>(&path, endpoints::jig::module::Delete::METHOD, Some(req)).await {
                Ok(_) => {
                    state.sidebar.modules.lock_mut().remove(index);
                },
                Err(_) => {}
            }
        }
    }));
}
pub fn add_empty_module_after(state: Rc<State>) {
    state
        .sidebar
        .modules
        .lock_mut()
        .insert_cloned(state.index + 1, Rc::new(None));
    state
        .sidebar
        .jig_edit_state
        .route
        .set_neq(JigEditRoute::Landing);
}
pub fn assign_kind(state: Rc<State>, kind: ModuleKind) {
    state.sidebar.loader.load(clone!(state => async move {
        let req = Some(ModuleCreateRequest {
            body: ModuleBody::new(kind),
        });
        let path = endpoints::jig::module::Create::PATH.replace("{id}",&state.sidebar.jig.id.0.to_string());

        match api_with_auth::<CreateResponse<ModuleId>, EmptyError, _>(&path, endpoints::jig::module::Create::METHOD, req).await {
            Ok(resp) => {
                let id = resp.id;
                let index = state.index;
                let module = Rc::new(Some(LiteModule {
                    id,
                    kind,
                }));
                state.sidebar.modules.lock_mut().set_cloned(index, module);
                let req = ModuleUpdateRequest {
                    id: StableOrUniqueId::Unique(id.clone()),
                    index: Some(index.try_into().unwrap_ji()),
                    body: None,
                    is_complete: None,
                };

                match update_module(&state.sidebar.jig.id, &id, req).await {
                    Ok(_) => {
                        state.sidebar.collapsed.set(true);
                        state.sidebar.jig_edit_state.route.set(JigEditRoute::Module(id.clone()));
                        Route::push_state(Route::Jig(JigRoute::Edit(state.sidebar.jig.id, JigEditRoute::Module(id))));
                    },
                    Err(e) => {
                        log::error!("Error: {:?}", e);
                    },
                }
            },
            Err(e) => {
                log::error!("Error: {:?}", e);
            },
        }
    }));
}

pub enum MoveTarget {
    Up,
    Down,
}
pub fn move_index(state: Rc<State>, move_target: MoveTarget) {
    state.sidebar.loader.load(clone!(state => async move {
        if let Some(module) = &*state.module {
            if let Some(target) = {
                match move_target {
                    MoveTarget::Up if state.index > 1 => {
                        Some(state.index-1)
                    },
                    MoveTarget::Down if state.index < state.total_len-1 => {
                        Some(state.index+1)
                    },
                    _ => None
                }
            } {
                state.sidebar.modules.lock_mut().move_from_to(state.index, target);
                let req = ModuleUpdateRequest {
                    id: StableOrUniqueId::Unique(module.id.clone()),
                    index: Some(target.try_into().unwrap_ji()),
                    body: None,
                    is_complete: None,
                };

                match update_module(&state.sidebar.jig.id, &module.id, req).await {
                    Ok(_) => {
                    },
                    Err(_) => {},
                }
            }
        }
    }));
}
