use dominator::{html, Dom};

pub struct JigData {
    pub jig_name: String,
    pub author: String,
    pub author_badge: String,
    pub date: String,
    pub language: String,
    pub curators: Vec<String>,
}

pub struct JigUI {}

impl JigUI {
    fn render_jig_span(slot: &str, text: &str) -> Dom {
        html!("span", {
            .attribute("slot", slot)
            .text(text)
        })
    }
    pub fn render() -> Dom {
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
