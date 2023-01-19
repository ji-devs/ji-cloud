use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::asset::Asset;
use utils::asset::{CoursePlayerOptions, JigPlayerOptions, ResourceContentExt};
use utils::routes::{AssetRoute, Route};

use utils::prelude::*;

pub struct ShareAsset {
    pub active_popup: Mutable<Option<ActivePopup>>,
    pub student_code: Mutable<Option<String>>,
    pub loader: AsyncLoader,
    pub asset: Asset,
    pub copied_embed: Mutable<bool>,
    pub link_copied: Mutable<bool>,
    pub copied_student_url: Mutable<bool>,
    pub copied_student_code: Mutable<bool>,
}

impl ShareAsset {
    pub fn new(asset: Asset) -> Rc<Self> {
        Rc::new(Self {
            asset,
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
        let origin = web_sys::window()
            .unwrap_ji()
            .location()
            .origin()
            .unwrap_ji();
        let url = match &self.asset {
            Asset::Jig(jig) => {
                let path = Route::Asset(AssetRoute::Play(AssetPlayRoute::Jig(
                    jig.id,
                    None,
                    JigPlayerOptions {
                        is_student,
                        ..Default::default()
                    },
                )))
                .to_string();
                format!("{}{}", origin, path)
            }
            Asset::Resource(resource) => {
                match resource.resource_data.additional_resources.get(0) {
                    Some(resource) => resource.resource_content.get_link(),
                    None => {
                        // Should't really get here
                        String::new()
                    }
                }
            }
            Asset::Course(course) => {
                let path = Route::Asset(AssetRoute::Play(AssetPlayRoute::Course(
                    course.id,
                    CoursePlayerOptions {
                        is_student,
                        ..Default::default()
                    },
                )))
                .to_string();
                format!("{}{}", origin, path)
            }
            Asset::ProDev(_) => todo!(),
        };
        url
    }

    pub(super) fn asset_type_name(&self) -> &'static str {
        match self.asset {
            Asset::Jig(_) => "JIG",
            Asset::Course(_) => "course",
            Asset::Resource(_) => "resource",
            Asset::ProDev(_) => todo!(),
        }
    }
}

#[derive(Clone)]
pub enum ActivePopup {
    ShareMain,
    ShareStudents,
    ShareEmbed,
}
