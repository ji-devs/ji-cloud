use super::state::*;
use dominator::{clone, html, Dom};
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Signal, SignalExt};
use futures_signals::signal_vec::SignalVecExt;
use shared::{
    api::{
        endpoints::{jig, user::Profile},
        ApiEndpoint,
    },
    domain::jig::{JigResponse, JigSearchQuery, JigSearchResponse},
    error::EmptyError,
};
use std::rc::Rc;
use utils::{prelude::*, routes::AdminRoute::Jigs};

impl JigUI {
    fn render_jig_span(slot: &str, text: String) -> Dom {
        html!("span", {
            .attribute("slot", slot)
            .text(&text)
        })
    }
    pub fn render(state: Rc<Self>) -> Dom {
        state.loader.load(clone!(state => async move {
            match api_no_auth::<JigSearchResponse, EmptyError, JigSearchQuery>(
                jig::Search::PATH,
                jig::Search::METHOD,
                None
            )
            .await
            {
                Err(_) => todo!(),
                Ok(resp) => {
                    state.jigs.lock_mut().replace_cloned(resp.jigs);
                }
            };
        }));
        html!("jig-label-ui", {
            .children_signal_vec(state.jigs.signal_vec_cloned().map(clone!(state => move |jig: JigResponse| {
                html!("single-jig", {
                    .children(&mut [
                        Self::render_jig_span("jig-name", jig.jig_data.display_name),
                        Self::render_jig_span("author", match jig.author_name {
                            Some(name) => name,
                            None => "".to_string()
                        }),
                        Self::render_jig_span("author-badge", "AUTHOR BADGE".to_string()),
                        Self::render_jig_span("date", match jig.published_at {
                            Some(published_at) => published_at.format("%b %e, %Y").to_string(),
                            None => "".to_string()
                        }),
                        Self::render_jig_span("language", jig.jig_data.language),
                        Self::render_jig_span("curators", "CURATORS".to_string()),
                    ])
                })
            })))
        })
    }
}
