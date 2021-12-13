use std::rc::Rc;

use dominator::{clone, html, Dom};

use shared::{
    api::{endpoints::jig, ApiEndpoint},
    error::EmptyError, domain::jig::JigResponse,
};

use utils::{
    prelude::api_with_auth_empty,
    events
};

use super::super::state::State;

pub fn render(state: Rc<State>, jig: &JigResponse) -> Dom {
    html!("jig-play-sidebar-action", {
        .property("slot", "actions")
        .property("kind", "like")
        // TODO Render active or not active
        .event(clone!(state, jig => move |_: events::Click| {
            state.loader.load(async move {
                let path = jig::Like::PATH.replace("{id}", &jig.id.0.to_string());
                let response = api_with_auth_empty::<EmptyError, ()>(
                    &path,
                    jig::Like::METHOD,
                    None
                )
                .await;

                if response.is_ok() {
                    // TODO Toggle active state
                    // TODO Update like count or refetch jig data
                }
            })
        }))
    })
}

