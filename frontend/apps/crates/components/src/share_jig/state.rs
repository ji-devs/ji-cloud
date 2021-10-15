use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::jig::JigId;
use utils::jig::JigPlayerOptions;
use utils::routes::{JigRoute, Route};

use utils::prelude::*;

pub struct State {
    pub active_popup: Mutable<Option<ActivePopup>>,
    pub student_code: Mutable<Option<String>>,
    pub loader: AsyncLoader,
    pub jig_id: JigId,
    pub copied_embed: Mutable<bool>,
}

impl State {
    pub fn new(jig_id: JigId) -> Self {
        Self {
            jig_id,
            student_code: Mutable::new(None),
            loader: AsyncLoader::new(),
            active_popup: Mutable::new(None),
            copied_embed: Mutable::new(false),
        }
    }

    pub fn embed_code(&self) -> String {
        let url = Route::Jig(JigRoute::Play(
            self.jig_id,
            None,
            JigPlayerOptions::default(),
        ))
        .to_string();
        let origin = web_sys::window()
            .unwrap_ji()
            .location()
            .origin()
            .unwrap_ji();
        format!(r#"<iframe src="{}{}"></iframe>"#, origin, url)
    }
}

#[derive(Clone)]
pub enum ActivePopup {
    ShareMain,
    ShareStudents,
    ShareEmbed,
}
