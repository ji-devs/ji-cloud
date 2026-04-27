use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::asset::Asset;
use shared::domain::jig::codes::JigCode;
use shared::domain::jig::TextDirection;
use utils::asset::ResourceContentExt;

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
    // signed share URL
    pub share_url: Mutable<String>,
    pub student_share_url: Mutable<String>,
}

impl ShareAsset {
    pub fn new(asset: Asset) -> Rc<Self> {
        let (direction, scoring, share_url, student_share_url) = match &asset {
            Asset::Jig(jig) => (
                jig.jig_data.default_player_settings.direction,
                jig.jig_data.default_player_settings.scoring,
                jig.share_url.clone(),
                jig.student_share_url.clone(),
            ),
            Asset::Playlist(playlist) => (
                Default::default(),
                Default::default(),
                playlist.share_url.clone(),
                playlist.share_url.clone(),
            ),
            Asset::Course(course) => (
                Default::default(),
                Default::default(),
                course.share_url.clone(),
                course.share_url.clone(),
            ),
            Asset::Resource(resource) => (
                Default::default(),
                Default::default(),
                resource
                    .resource_data
                    .additional_resources
                    .get(0)
                    .map(|resource| resource.resource_content.get_link())
                    .unwrap_or_default(),
                resource
                    .resource_data
                    .additional_resources
                    .get(0)
                    .map(|resource| resource.resource_content.get_link())
                    .unwrap_or_default(),
            ),
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
            share_url: Mutable::new(share_url),
            student_share_url: Mutable::new(student_share_url),
        })
    }

    pub fn embed_code(&self) -> String {
        let link = self.student_share_url.get_cloned();
        self.embed_code_with_url(&link)
    }

    pub fn embed_code_with_url(&self, link: &str) -> String {
        format!(
            r#"<iframe src="{}" width="960" height="540" allow="autoplay; fullscreen" frameborder="0"></iframe>"#,
            link
        )
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
