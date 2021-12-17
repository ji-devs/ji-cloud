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
    fn render_jig_span(slot: &str, text: &str) -> Dom {
        html!("span", {
            .attribute("slot", slot)
            .text(text)
        })
    }
    pub fn render(state: Rc<Self>) -> Dom {
        let init_loader = AsyncLoader::new();
        init_loader.load(clone!(state => async move {
            match api_no_auth::<JigSearchResponse, EmptyError, JigSearchQuery>(
                jig::Search::PATH,
                jig::Search::METHOD,
                None
            )
            .await
            {
                Err(_) => {}
                Ok(jigSearchResponse) => {
                    state.jigs.lock_mut().replace_cloned(jigSearchResponse.jigs);
                }
            };
        }));

        // let jigs = vec![
        //     JigData {
        //         jig_name: String::from("Hebrew Letters"),
        //         author: String::from("Michael Wikes"),
        //         author_badge: String::from("JI Team"),
        //         date: String::from("Aug. 5, 2020"),
        //         language: String::from("English (American)"),
        //         curators: vec![String::from("Anat (13.7.21)")],
        //     },
        //     JigData {
        //         jig_name: String::from("Hebrew Letters"),
        //         author: String::from("Michael Wikes"),
        //         author_badge: String::from("JI Team"),
        //         date: String::from("Aug. 5, 2020"),
        //         language: String::from("English (American)"),
        //         curators: vec![String::from("Anat (13.7.21)")],
        //     },
        //     JigData {
        //         jig_name: String::from("Hebrew Letters"),
        //         author: String::from("Michael Wikes"),
        //         author_badge: String::from("JI Team"),
        //         date: String::from("Aug. 5, 2020"),
        //         language: String::from("English (American)"),
        //         curators: vec![String::from("Anat (13.7.21)")],
        //     },
        //     JigData {
        //         jig_name: String::from("Hebrew Letters"),
        //         author: String::from("Michael Wikes"),
        //         author_badge: String::from("JI Team"),
        //         date: String::from("Aug. 5, 2020"),
        //         language: String::from("English (American)"),
        //         curators: vec![String::from("Anat (13.7.21)")],
        //     },
        //     JigData {
        //         jig_name: String::from("Hebrew Letters"),
        //         author: String::from("Michael Wikes"),
        //         author_badge: String::from("JI Team"),
        //         date: String::from("Aug. 5, 2020"),
        //         language: String::from("English (American)"),
        //         curators: vec![String::from("Anat (13.7.21)")],
        //     },
        //     JigData {
        //         jig_name: String::from("Hebrew Letters"),
        //         author: String::from("Michael Wikes"),
        //         author_badge: String::from("JI Team"),
        //         date: String::from("Aug. 5, 2020"),
        //         language: String::from("English (American)"),
        //         curators: vec![String::from("Anat (13.7.21)")],
        //     },
        // ];
        html!("jig-label-ui", {
            // .children(jigs.iter().map(|jig: &JigData| {
            //     html!("single-jig", {
            //         .children(&mut [
            //             Self::render_jig_span("jig-name", &jig.jig_name),
            //             Self::render_jig_span("author", &jig.author),
            //             Self::render_jig_span("author-badge", &jig.author_badge),
            //             Self::render_jig_span("date", &jig.date),
            //             Self::render_jig_span("language", &jig.language),
            //             Self::render_jig_span("curators", &jig.curators.join(", ")),
            //         ])
            //     })
            // }))
            .children_signal_vec(state.jigs.signal_vec_cloned().map(clone!(state => move |jig: JigResponse| {
                html!("single-jig", {
                    .children(&mut [
                        Self::render_jig_span("jig-name", &jig.jig_data.display_name),
                        Self::render_jig_span("author", match &jig.author_name {
                            Some(name) => name,
                            None => ""
                        }),
                        Self::render_jig_span("author-badge", "AUTHOR BADGE"),
                        Self::render_jig_span("date", match &jig.published_at {
                            Some(published_at) => "", // &published_at.date().naive_utc().format("%Y-%m-%d").to_string(),
                            None => ""
                        }),
                        Self::render_jig_span("language", &jig.jig_data.language),
                        Self::render_jig_span("curators", "CURATORS"),
                    ])
                })
            })))
        })
    }
}
