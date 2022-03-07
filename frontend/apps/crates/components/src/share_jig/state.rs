use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::jig::JigId;
use utils::jig::JigPlayerOptions;
use utils::routes::{JigRoute, Route};

use utils::prelude::*;

pub struct ShareJig {
    pub active_popup: Mutable<Option<ActivePopup>>,
    pub student_code: Mutable<Option<String>>,
    pub loader: AsyncLoader,
    pub jig_id: JigId,
    pub copied_embed: Mutable<bool>,
    pub link_copied: Mutable<bool>,
}

impl ShareJig {
    pub fn new(jig_id: JigId) -> Rc<Self> {
        Rc::new(Self {
            jig_id,
            student_code: Mutable::new(None),
            loader: AsyncLoader::new(),
            active_popup: Mutable::new(None),
            copied_embed: Mutable::new(false),
            link_copied: Mutable::new(false),
        })
    }

    pub fn embed_code(&self) -> String {
        let link = self.jig_link();
        format!(r#"<iframe src="{}" width="960" height="540"></iframe>"#, link)
    }

    pub fn jig_link(&self) -> String {
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

        format!("{}{}", origin, url)
    }
}

#[derive(Clone)]
pub enum ActivePopup {
    ShareMain,
    ShareStudents,
    ShareEmbed,
}
