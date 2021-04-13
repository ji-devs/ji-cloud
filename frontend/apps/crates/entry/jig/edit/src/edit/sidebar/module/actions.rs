use shared::{
    api::endpoints::{ApiEndpoint, self},
    error::{EmptyError, MetadataNotFound},
    domain::{CreateResponse, jig::*, jig::module::*},
};
use std::rc::Rc;
use super::state::{State, Module};
use utils::{prelude::*, drag::Drag};
use crate::edit::sidebar::dragging::state::State as DragState;
use dominator::clone;

pub fn mouse_down(state: Rc<State>, x: i32, y:i32) {
    state.sidebar.drag.set(Some(Rc::new(DragState::new(state.clone(), x, y))));
}

pub fn edit(state: Rc<State>) {
    let module_id = state.module.id;
    state.sidebar.module_id.set_neq(Some(module_id));

    let jig_id = state.sidebar.jig_id;
    let url:String = Route::Jig(JigRoute::Edit(jig_id, Some(module_id))).into();
    log::info!("{}", url);

    /* this will cause a full refresh - but preserves history
     * see the .future in EditPage too
    dominator::routing::go_to_url(&url);
     */
}
pub fn assign_kind(state: Rc<State>, kind: ModuleKind) {
    state.sidebar.loader.load(clone!(state => async move {
        let req = Some(ModuleUpdateRequest {
            kind: Some(kind),
            ..ModuleUpdateRequest::default()
        });

        let path = endpoints::jig::module::Update::PATH.replace("{id}",&state.module.id.0.to_string());
        match api_with_auth_empty::<EmptyError, _>(&path, endpoints::jig::module::Update::METHOD, req).await {
            Ok(_) => {
                state.module.kind.set_neq(Some(kind));
            },
            Err(_) => {},
        }
    }));
}

pub fn delete(state:Rc<State>) {
    let index = state.index;

    state.sidebar.loader.load(clone!(state => async move {
        let path = endpoints::jig::module::Delete::PATH.replace("{id}",&state.module.id.0.to_string());
        match api_with_auth_empty::<EmptyError, ()>(&path, endpoints::jig::module::Delete::METHOD, None).await {
            Ok(_) => {
                state.sidebar.modules.lock_mut().remove(index);
                update_module_list(state.clone()).await;
            },
            Err(_) => {}
        }
    }));

}
pub fn add_empty_module_after(state:Rc<State>) {
    state.sidebar.loader.load(clone!(state => async move {
        let req = Some(ModuleCreateRequest {
            kind: None,
            body: None
        });

        match api_with_auth::<CreateResponse<ModuleId>, EmptyError, _>(&endpoints::jig::module::Create::PATH, endpoints::jig::module::Create::METHOD, req).await {
            Ok(resp) => {
                let id = resp.id;
                let index = state.index+1;
                state.sidebar.modules.lock_mut().insert_cloned(index, Rc::new(Module::new(id)));
                update_module_list(state.clone()).await;
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
            let index = state.index;
            state.sidebar.modules.lock_mut().move_from_to(state.index, target);
            update_module_list(state.clone()).await;
        }
    }));
}

async fn update_module_list(state: Rc<State>) {
    let modules:Vec<ModuleId> = state.sidebar.modules
            .lock_ref()
            .iter()
            .map(|module| module.id.clone())
            .collect();

    let req = Some(JigUpdateRequest {
        display_name: None,
        modules: Some(modules),
        content_types: None,
        author_id: None,
        publish_at: None
    });

    let path = endpoints::jig::Update::PATH.replace("{id}",&state.sidebar.jig_id.0.to_string());
    match api_with_auth_empty::<MetadataNotFound, _>(&path, endpoints::jig::Update::METHOD, req).await {
        Ok(_) => {},
        Err(_) => {},
    }
}
