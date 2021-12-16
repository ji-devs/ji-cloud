use super::state::*;
use dominator::{clone, html, Dom};
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Signal, SignalExt};
use futures_signals::signal_vec::SignalVecExt;
use shared::{
    api::{endpoints::user::Profile, ApiEndpoint},
    domain::{jig::JigResponse, user::UserProfile},
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
            let (result, status) = api_with_auth_status::<UserProfile, EmptyError, ()>("https://api.sandbox.jigzi.org/v1/jig", "GET", None).await;

            // match api_no_auth::<JigSearchResponse, EmptyError, JigSearchQuery>(
            //     jig::Search::PATH,
            //     jig::Search::METHOD,
            //     Some(req),
            // )
            // .await
            // {
            //     Err(_) => {}
            //     Ok(res) => {
            //         state.mode.set(HomePageMode::Search(
            //             query,
            //             Rc::new(MutableVec::new_with_values(res.jigs)),
            //         ));
            //     }
            // };
            
            match status  {
                401 | 403 => {
                    state.jigs.set(Some(None));
                }
                _ => {
                    match result {
                        Err(_) => {
                            log::info!("error fetching jigs");
                        },
                        Ok(jigs) => {
                            state.jigs.set(Some(Some(jigs)));
                        }
                    }
                }
            };
            state.jigs.lock_mut().replace_cloned(jigs);
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
            .children_signal_vec(state.jigs.signal_vec_cloned().map(clone!(state => move |jig| {
                html!("single-jig", {
                    .children(&mut [
                        Self::render_jig_span("jig-name", &jig.jig_name),
                        Self::render_jig_span("author", &jig.author),
                        Self::render_jig_span("author-badge", &jig.author_badge),
                        Self::render_jig_span("date", &jig.date),
                        Self::render_jig_span("language", &jig.language),
                        Self::render_jig_span("curators", &jig.curators.join(", ")),
                    ])
                })
            })))
        })
    }
}
