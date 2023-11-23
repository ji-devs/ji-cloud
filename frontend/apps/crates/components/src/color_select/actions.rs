use rgb::RGBA8;
use shared::{
    api::endpoints,
    domain::user::{
        UserColorCreatePath, UserColorDeletePath, UserColorGetPath, UserColorValueRequest,
    },
};
use std::rc::Rc;

use utils::prelude::*;

use super::state::ColorSelector;

pub fn set_selected(state: Rc<ColorSelector>, value: Option<RGBA8>) {
    if let Some(on_select) = state.on_select.as_ref() {
        on_select(value);
    }
    state.value.set(value);
}

pub async fn get_user_colors() -> Result<Vec<RGBA8>, anyhow::Error> {
    let res = endpoints::user::GetColors::api_with_auth(UserColorGetPath(), None).await?;

    Ok(res.colors)
}

pub async fn add_user_color(state: Rc<ColorSelector>, color: RGBA8) -> Result<(), anyhow::Error> {
    let req = UserColorValueRequest { color };

    endpoints::user::CreateColor::api_with_auth(UserColorCreatePath(), Some(req)).await?;

    state.user_colors.lock_mut().push_cloned(color);
    set_selected(Rc::clone(&state), Some(color));

    Ok(())
}

pub async fn delete_user_color(state: Rc<ColorSelector>, index: usize) {
    let res =
        endpoints::user::DeleteColor::api_with_auth(UserColorDeletePath(index as i32), None).await;

    match res {
        Err(_) => {}
        Ok(_) => {
            state.user_colors.lock_mut().remove(index);
        }
    }
}
