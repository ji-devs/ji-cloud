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

pub fn on_module_kind_drop(state: Rc<State>, module_kind: ModuleKind) {
    if state.index == 0 && module_kind != ModuleKind::Cover {
        return;
    }
    if state.module.is_none() {
        assign_kind(state.clone(), module_kind);
    }

    // Remove module highlights whenever a new module is added to the list.
    state.sidebar.highlight_modules.set_neq(None);
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

#[allow(dead_code)] // this should be removed eventually
pub fn mouse_down(state: Rc<State>, x: i32, y: i32) {
    state
        .sidebar
        .drag
        .set(Some(Rc::new(DragState::new(state.clone(), x, y))));
}

pub fn edit(state: Rc<State>) {
    if let Some(module) = &*state.module {
        let module_id = module.id();
        state
            .sidebar
            .jig_edit_state
            .route
            .set_neq(JigEditRoute::Module(*module_id));
        state.sidebar.collapsed.set(true);

        let jig_id = state.sidebar.jig.id;
        Route::push_state(Route::Jig(JigRoute::Edit(
            jig_id,
            state.sidebar.jig.jig_focus,
            JigEditRoute::Module(*module_id),
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
                id: StableOrUniqueId::Unique(*module.id())
            };

            match api_with_auth_empty::<EmptyError, _>(&path, endpoints::jig::module::Delete::METHOD, Some(req)).await {
                Ok(_) => {
                    state.sidebar.modules.lock_mut().remove(index);
                },
                Err(_) => {}
            }
        } else {
            // The module is placeholder, it is not persisted so it can be removed from the list with
            // no extra work required.
            state.sidebar.modules.lock_mut().remove(index);
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
                    is_complete: false,
                }.into()));

                {
                    // Instead of replacing the module at the index, we remove the old module and
                    // add the new one. This is slightly less efficient because it fires signals
                    // for the entire list of modules, however, it is necessary so that the modules
                    // before and after this one can have their views updated.
                    let mut modules = state.sidebar.modules.lock_mut();
                    modules.remove(index);
                    modules.insert_cloned(index, module);

                    // Only add a new placeholder module once the above request has completed and
                    // the new module has been added to the list of modules.
                    let placeholder_exists = {
                        match modules.last() {
                            // If the list of modules is not empty and the last module is None, then it is
                            // a placeholder module.
                            Some(module) => module.is_none(),
                            // When the list is empty or the last module is not a placeholder module.
                            _ => false,
                        }
                    };

                    // if this is the empty module at the end
                    if !placeholder_exists {
                        modules.push_cloned(Rc::new(None));
                    }
                }

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
                        Route::push_state(Route::Jig(JigRoute::Edit(
                            state.sidebar.jig.id,
                            state.sidebar.jig.jig_focus,
                            JigEditRoute::Module(id)
                        )));
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
                    id: StableOrUniqueId::Unique(*module.id()),
                    index: Some(target.try_into().unwrap_ji()),
                    body: None,
                    is_complete: None,
                };

                match update_module(&state.sidebar.jig.id, module.id(), req).await {
                    Ok(_) => {
                    },
                    Err(_) => {},
                }
            }
        }
    }));
}
