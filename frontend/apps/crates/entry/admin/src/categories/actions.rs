use super::state::*;
use futures_signals::signal_vec::{MutableVecLockMut, MutableVecLockRef};
use shared::{
    api::endpoints::{self, ApiEndpoint},
    domain::category::{
        CategoryResponse, CategoryTreeScope, CreateCategoryRequest, GetCategoryRequest,
        NewCategoryResponse, UpdateCategoryRequest,
    },
    error::EmptyError,
};
use std::rc::Rc;
use utils::{fetch::{api_with_auth, api_with_auth_empty}, unwrap::UnwrapJiExt};

use dominator::clone;
use wasm_bindgen::prelude::*;

pub fn toggle_expand_all(cat: &Rc<Category>, flag: bool) {
    cat.expanded.set(flag);

    for child in cat.children.lock_ref().iter() {
        toggle_expand_all(child, flag);
    }
}
pub fn load_categories(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        let req = GetCategoryRequest {
            ids: Vec::new(),
            scope: Some(CategoryTreeScope::Decendants)
        };

        match api_with_auth::<CategoryResponse, EmptyError, _>(endpoints::category::Get::PATH, endpoints::category::Get::METHOD, Some(req)).await {
            Ok(resp) => {
                let categories:Vec<Rc<Category>> = resp.categories
                    .into_iter()
                    .map(Category::from)
                    .map(Rc::new)
                    .collect();

                state.categories.lock_mut().replace_cloned(categories);

            },
            Err(_) => {
                log::info!("err!")
            }
        }
    }));
}

impl ContentState {
    //siblings includes self
    fn with_siblings_mut<A, F: FnOnce(MutableVecLockMut<Rc<Category>>) -> A>(&self, f: F) -> A {
        match &self.parent {
            Some(parent) => f(parent.children.lock_mut()),
            None => f(self.state.categories.lock_mut()),
        }
    }
    fn with_siblings_ref<A, F: FnOnce(MutableVecLockRef<Rc<Category>>) -> A>(&self, f: F) -> A {
        match &self.parent {
            Some(parent) => f(parent.children.lock_ref()),
            None => f(self.state.categories.lock_ref()),
        }
    }

    pub fn close_menu(&self) {
        if let Some(menu_ref) = self.menu_ref.borrow().as_ref() {
            let _ = js_sys::Reflect::set(
                menu_ref,
                &JsValue::from_str("visible"),
                &JsValue::from_bool(false),
            );
        }
    }

    fn current_index(&self) -> usize {
        self.with_siblings_ref(|children| {
            children
                .iter()
                .position(|x| x.id == self.cat.id)
                .unwrap_ji()
        })
    }
}

pub fn add_category_root(state: Rc<State>) {
    add_category(state, None);
}
pub fn add_category_child(content_state: Rc<ContentState>) {
    content_state.close_menu();
    add_category(content_state.state.clone(), Some(content_state.cat.clone()));
}
fn add_category(state: Rc<State>, parent: Option<Rc<Category>>) {
    state.loader.load(clone!(state => async move {
        let name = crate::strings::STR_NEW_CATEGORY_NAME.to_string();

        let req = CreateCategoryRequest {
            name: name.clone(),
            parent_id: parent.as_ref().map(|cat| cat.id)
        };

        match api_with_auth::<NewCategoryResponse, EmptyError, _>(endpoints::category::Create::PATH, endpoints::category::Create::METHOD, Some(req)).await {
            Ok(resp) => {
                // Categories created here should be in the editing state already.
                let cat = Rc::new(Category::new(resp.id, name, true));

                match parent {
                    Some(parent) => {
                        parent.children.lock_mut().push_cloned(cat);
                    },
                    None => {
                        state.categories.lock_mut().push_cloned(cat);
                    }
                }
            },
            Err(_) => {
                log::info!("err!")
            }
        }
    }));
}

pub enum Direction {
    Up,
    Down,
}
pub fn move_category(content_state: Rc<ContentState>, dir: Direction) {
    content_state.close_menu();

    let current_index = content_state.current_index();

    //release the borrow of content_state before doing async
    let target_index = content_state.with_siblings_mut(|mut children| {
        let target_index = match dir {
            Direction::Up => {
                if current_index > 0 {
                    Some(current_index - 1)
                } else {
                    None
                }
            }
            Direction::Down => {
                if current_index < children.len() - 1 {
                    Some(current_index + 1)
                } else {
                    None
                }
            }
        };

        if let Some(target_index) = target_index {
            children.move_from_to(current_index, target_index);
        }

        target_index
    });

    if let Some(target_index) = target_index {
        let id = content_state.cat.id;

        let path = endpoints::category::Update::PATH.replace("{id}", &id.0.to_string());

        let req = UpdateCategoryRequest {
            name: None,
            parent_id: None,
            index: Some(target_index as u16),
            user_scopes: None,
        };
        content_state.state.loader.load(async move {
            match api_with_auth_empty::<EmptyError, _>(
                &path,
                endpoints::category::Update::METHOD,
                Some(req),
            )
            .await
            {
                Ok(_) => {}
                Err(_) => {
                    log::info!("err!")
                }
            }
        });
    }
}

pub fn delete_category(content_state: Rc<ContentState>) {
    content_state.state.loader.load(clone!(content_state => async move {
        let id = content_state.cat.id;

        let path = endpoints::category::Delete::PATH.replace(
            "{id}",
            &id.0.to_string()
        );

        match api_with_auth_empty::<EmptyError, ()>(&path, endpoints::category::Delete::METHOD, None).await {
            Ok(_) => {
                content_state
                    .with_siblings_mut(move |mut children|
                        children.retain(|x| x.id != id)
                    );
            },
            Err(_) => {
                log::info!("err!")
            }
        }
    }));
}

pub fn rename_category(cat: &Rc<Category>, state: Rc<State>, name: String) {
    cat.name.set(name.clone());
    let id = cat.id;

    state.loader.load(async move {
        let path = endpoints::category::Update::PATH.replace("{id}", &id.0.to_string());
        let req = UpdateCategoryRequest {
            name: Some(name),
            parent_id: None,
            index: None,
            user_scopes: None,
        };

        match api_with_auth_empty::<EmptyError, _>(
            &path,
            endpoints::category::Update::METHOD,
            Some(req),
        )
        .await
        {
            Ok(_) => {}
            Err(_) => {
                log::info!("err!")
            }
        }
    });
}
