use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::asset::AssetId;
use utils::asset::{CoursePlayerOptions, JigPlayerOptions};
use utils::routes::{AssetRoute, Route};

use utils::prelude::*;

pub struct ShareAsset {
    pub active_popup: Mutable<Option<ActivePopup>>,
    pub student_code: Mutable<Option<String>>,
    pub loader: AsyncLoader,
    pub asset_id: AssetId,
    pub copied_embed: Mutable<bool>,
    pub link_copied: Mutable<bool>,
    pub copied_student_url: Mutable<bool>,
    pub copied_student_code: Mutable<bool>,
}

impl ShareAsset {
    pub fn new(asset_id: AssetId) -> Rc<Self> {
        Rc::new(Self {
            asset_id,
            student_code: Mutable::new(None),
            loader: AsyncLoader::new(),
            active_popup: Mutable::new(None),
            copied_embed: Mutable::new(false),
            link_copied: Mutable::new(false),
            copied_student_url: Mutable::new(false),
            copied_student_code: Mutable::new(false),
        })
    }

    pub fn embed_code(&self) -> String {
        let link = self.asset_link(true);
        format!(
            r#"<iframe src="{}" width="960" height="540"></iframe>"#,
            link
        )
    }

    pub(super) fn asset_link(&self, is_student: bool) -> String {
        let url = match self.asset_id {
            AssetId::JigId(jig_id) => Route::Asset(AssetRoute::Play(AssetPlayRoute::Jig(
                jig_id,
                None,
                JigPlayerOptions {
                    is_student,
                    ..Default::default()
                },
            ))),
            AssetId::ResourceId(_) => {
                todo!()
            }
            AssetId::CourseId(course_id) => Route::Asset(AssetRoute::Play(AssetPlayRoute::Course(
                course_id,
                CoursePlayerOptions {
                    is_student,
                    ..Default::default()
                },
            ))),
        }
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
