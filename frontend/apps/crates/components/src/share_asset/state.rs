use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::asset::Asset;
use shared::domain::jig::codes::JigCode;
use shared::domain::jig::TextDirection;
use utils::asset::{
    CoursePlayerOptions, JigPlayerOptions, PlaylistPlayerOptions, ResourceContentExt,
};
use utils::routes::{AssetRoute, Route};

use utils::prelude::*;

use crate::qr_dialog::QrDialog;

pub struct ShareAsset {
    pub active_popup: Mutable<Option<ActivePopup>>,
    pub student_code: Mutable<Option<JigCode>>,
    pub loader: AsyncLoader,
    pub asset: Asset,
    pub copied_embed: Mutable<bool>,
    pub link_copied: Mutable<bool>,
    pub copied_student_url: Mutable<bool>,
    pub copied_student_code: Mutable<bool>,
    pub qr_dialog: Mutable<Option<Rc<QrDialog>>>,
    // play settings
    pub code_name: Mutable<Option<String>>,
    pub direction: Mutable<TextDirection>,
    pub scoring: Mutable<bool>,
}

impl ShareAsset {
    pub fn new(asset: Asset) -> Rc<Self> {
        let direction = match &asset {
            Asset::Jig(jig) => jig.jig_data.default_player_settings.direction,
            _ => Default::default(),
        };
        let scoring = match &asset {
            Asset::Jig(jig) => jig.jig_data.default_player_settings.scoring,
            _ => Default::default(),
        };
        Rc::new(Self {
            asset,
            student_code: Mutable::new(None),
            loader: AsyncLoader::new(),
            active_popup: Mutable::new(None),
            copied_embed: Mutable::new(false),
            link_copied: Mutable::new(false),
            copied_student_url: Mutable::new(false),
            copied_student_code: Mutable::new(false),
            qr_dialog: Mutable::new(None),
            code_name: Mutable::new(None),
            direction: Mutable::new(direction),
            scoring: Mutable::new(scoring),
        })
    }

    pub fn embed_code(&self) -> String {
        let link = self.asset_link(true, false);
        format!(
            r#"<iframe src="{}" width="960" height="540" allow="autoplay; fullscreen" frameborder="0"></iframe>"#,
            link
        )
    }

    pub(super) fn asset_link(&self, is_student: bool, quota: bool) -> String {
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
                        quota,
                        direction: Some(self.direction.get()),
                        scoring: Some(self.scoring.get()),
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
            Asset::Playlist(playlist) => {
                let path = Route::Asset(AssetRoute::Play(AssetPlayRoute::Playlist(
                    playlist.id,
                    PlaylistPlayerOptions {
                        is_student,
                        ..Default::default()
                    },
                )))
                .to_string();
                format!("{}{}", origin, path)
            }
            Asset::Course(course) => {
                let path = Route::Asset(AssetRoute::Play(AssetPlayRoute::Course(
                    course.id,
                    None,
                    CoursePlayerOptions {
                        is_student,
                        ..Default::default()
                    },
                )))
                .to_string();
                format!("{}{}", origin, path)
            }
        };
        url
    }

    pub(super) fn asset_type_name(&self) -> &'static str {
        match self.asset {
            Asset::Jig(_) => "JIG",
            Asset::Playlist(_) => "playlist",
            Asset::Resource(_) => "resource",
            Asset::Course(_) => "course",
        }
    }
}

#[derive(Clone)]
pub enum ActivePopup {
    ShareMain,
    ShareCode,
    ShareEmbed,
}
