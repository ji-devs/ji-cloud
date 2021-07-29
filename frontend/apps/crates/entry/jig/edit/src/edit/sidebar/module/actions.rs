// use shared::{api::endpoints::{ApiEndpoint, self}, domain::{CreateResponse, jig::{self::*, module::body::cover::ModuleData}, jig::module::*}, error::{EmptyError, MetadataNotFound}};
use shared::{
    api::endpoints::{ApiEndpoint, self},
    error::{EmptyError, MetadataNotFound},
    domain::{
        CreateResponse,
        jig::*,
        jig::module::{*, body::cover::ModuleData}
    },
};
use std::rc::Rc;
use super::state::State;
use utils::{prelude::*, drag::Drag};
use crate::edit::sidebar::dragging::state::State as DragState;
use dominator::clone;
use std::convert::TryInto;

pub async fn update_module(jig_id: &JigId, module_id: &ModuleId, req: ModuleUpdateRequest) -> Result<(), EmptyError> {
    let path = endpoints::jig::module::Update::PATH
        .replace("{id}", &jig_id.0.to_string())
        .replace("{module_id}", &module_id.0.to_string());
    api_with_auth_empty::<EmptyError, _>(&path, endpoints::jig::module::Update::METHOD, Some(req)).await
}

pub fn mouse_down(state: Rc<State>, x: i32, y:i32) {
    state.sidebar.drag.set(Some(Rc::new(DragState::new(state.clone(), x, y))));
}

pub fn edit(state: Rc<State>) {
    if let Some(module) = &*state.module {
        let module_id = module.id;
        state.sidebar.route.set_neq(JigEditRoute::Module(module_id));
        state.sidebar.collapsed.set(true);

        let jig_id = state.sidebar.jig.id;
        let url:String = Route::Jig(JigRoute::Edit(jig_id, JigEditRoute::Module(module_id))).into();
        log::info!("{}", url);

        /* this will cause a full refresh - but preserves history
        * see the .future in EditPage too
        dominator::routing::go_to_url(&url);
        */
    }
}

pub fn delete(state:Rc<State>) {
    let index = state.index;

    state.sidebar.loader.load(clone!(state => async move {
        if let Some(module) = &*state.module {
            let path = endpoints::jig::module::Delete::PATH
                .replace("{id}",&state.sidebar.jig.id.0.to_string())
                .replace("{module_id}",&module.id.0.to_string());
            match api_with_auth_empty::<EmptyError, ()>(&path, endpoints::jig::module::Delete::METHOD, None).await {
                Ok(_) => {
                    state.sidebar.modules.lock_mut().remove(index);
                },
                Err(_) => {}
            }
        }
    }));
}
pub fn add_empty_module_after(state:Rc<State>) {
    state.sidebar.modules.lock_mut().insert_cloned(state.index + 1, Rc::new(None));
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
                    index: Some(index.try_into().unwrap_ji()),
                    ..Default::default()
                };

                match update_module(&state.sidebar.jig.id, &id, req).await {
                    Ok(_) => {
                        state.sidebar.route.set(JigEditRoute::Module(id));
                        state.sidebar.collapsed.set(true);
                    },
                    Err(_) => {},
                }
            },
            Err(_) => {},
        }
    }));
}

pub enum MoveTarget {
    Up,
    Down,
    Any(usize)
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
                    MoveTarget::Any(target) => Some(target),
                    _ => None
                }
            } {
                state.sidebar.modules.lock_mut().move_from_to(state.index, target);
                let req = ModuleUpdateRequest {
                    index: Some(target.try_into().unwrap_ji()),
                    ..Default::default()
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
