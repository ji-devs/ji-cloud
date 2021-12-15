use super::state::*;
use dominator::{clone, html, Dom};
use dominator_helpers::futures::AsyncLoader;
use std::rc::Rc;

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
            let jigs = vec![
                JigData {
                    jig_name: String::from("Hebrew Letters"),
                    author: String::from("Michael Wikes"),
                    author_badge: String::from("JI Team"),
                    date: String::from("Aug. 5, 2020"),
                    language: String::from("English (American)"),
                    curators: vec![String::from("Anat (13.7.21)")],
                },
                JigData {
                    jig_name: String::from("Hebrew Letters"),
                    author: String::from("Michael Wikes"),
                    author_badge: String::from("JI Team"),
                    date: String::from("Aug. 5, 2020"),
                    language: String::from("English (American)"),
                    curators: vec![String::from("Anat (13.7.21)")],
                },
                JigData {
                    jig_name: String::from("Hebrew Letters"),
                    author: String::from("Michael Wikes"),
                    author_badge: String::from("JI Team"),
                    date: String::from("Aug. 5, 2020"),
                    language: String::from("English (American)"),
                    curators: vec![String::from("Anat (13.7.21)")],
                },
                JigData {
                    jig_name: String::from("Hebrew Letters"),
                    author: String::from("Michael Wikes"),
                    author_badge: String::from("JI Team"),
                    date: String::from("Aug. 5, 2020"),
                    language: String::from("English (American)"),
                    curators: vec![String::from("Anat (13.7.21)")],
                },
                JigData {
                    jig_name: String::from("Hebrew Letters"),
                    author: String::from("Michael Wikes"),
                    author_badge: String::from("JI Team"),
                    date: String::from("Aug. 5, 2020"),
                    language: String::from("English (American)"),
                    curators: vec![String::from("Anat (13.7.21)")],
                },
                JigData {
                    jig_name: String::from("Hebrew Letters"),
                    author: String::from("Michael Wikes"),
                    author_badge: String::from("JI Team"),
                    date: String::from("Aug. 5, 2020"),
                    language: String::from("English (American)"),
                    curators: vec![String::from("Anat (13.7.21)")],
                },
            ];
            state.jigs.lock_mut().replace_cloned(jigs);
        }));

        let jigs = vec![
            JigData {
                jig_name: String::from("Hebrew Letters"),
                author: String::from("Michael Wikes"),
                author_badge: String::from("JI Team"),
                date: String::from("Aug. 5, 2020"),
                language: String::from("English (American)"),
                curators: vec![String::from("Anat (13.7.21)")],
            },
            JigData {
                jig_name: String::from("Hebrew Letters"),
                author: String::from("Michael Wikes"),
                author_badge: String::from("JI Team"),
                date: String::from("Aug. 5, 2020"),
                language: String::from("English (American)"),
                curators: vec![String::from("Anat (13.7.21)")],
            },
            JigData {
                jig_name: String::from("Hebrew Letters"),
                author: String::from("Michael Wikes"),
                author_badge: String::from("JI Team"),
                date: String::from("Aug. 5, 2020"),
                language: String::from("English (American)"),
                curators: vec![String::from("Anat (13.7.21)")],
            },
            JigData {
                jig_name: String::from("Hebrew Letters"),
                author: String::from("Michael Wikes"),
                author_badge: String::from("JI Team"),
                date: String::from("Aug. 5, 2020"),
                language: String::from("English (American)"),
                curators: vec![String::from("Anat (13.7.21)")],
            },
            JigData {
                jig_name: String::from("Hebrew Letters"),
                author: String::from("Michael Wikes"),
                author_badge: String::from("JI Team"),
                date: String::from("Aug. 5, 2020"),
                language: String::from("English (American)"),
                curators: vec![String::from("Anat (13.7.21)")],
            },
            JigData {
                jig_name: String::from("Hebrew Letters"),
                author: String::from("Michael Wikes"),
                author_badge: String::from("JI Team"),
                date: String::from("Aug. 5, 2020"),
                language: String::from("English (American)"),
                curators: vec![String::from("Anat (13.7.21)")],
            },
        ];
        html!("jig-label-ui", {
            .children(jigs.iter().map(|jig: &JigData| {
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
            }))
        })
    }
}
