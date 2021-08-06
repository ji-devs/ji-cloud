use rgb::RGBA8;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::user::{UserColorResponse, UserColorValueRequest},
    error::EmptyError,
};
use std::rc::Rc;

use utils::prelude::*;

use super::state::State;

pub fn set_selected(state: Rc<State>, value: Option<RGBA8>) {
    if let Some(on_select) = state.on_select.as_ref() {
        on_select(value.clone());
    }
    state.value.set(value);
}

pub async fn get_user_colors() -> Result<Vec<RGBA8>, EmptyError> {
    let res = api_with_auth::<UserColorResponse, EmptyError, Option<()>>(
        &endpoints::user::GetColors::PATH,
        endpoints::user::GetColors::METHOD,
        None,
    )
    .await?;

    Ok(res.colors)
}

pub async fn add_user_color(state: Rc<State>, color: RGBA8) -> Result<(), EmptyError> {
    let req = UserColorValueRequest { color };

    api_with_auth::<UserColorResponse, EmptyError, UserColorValueRequest>(
        &endpoints::user::CreateColor::PATH,
        endpoints::user::CreateColor::METHOD,
        Some(req),
    )
    .await?;

    state.user_colors.lock_mut().push_cloned(color.clone());
    set_selected(Rc::clone(&state), Some(color));

    Ok(())
}

pub async fn delete_user_color(state: Rc<State>, index: usize) {
    let res = api_with_auth_empty::<EmptyError, ()>(
        &endpoints::user::DeleteColor::PATH.replace("{index}", &index.to_string()),
        endpoints::user::DeleteColor::METHOD,
        None,
    )
    .await;

    match res {
        Err(_) => {}
        Ok(_) => {
            state.user_colors.lock_mut().remove(index);
        }
    }
}
