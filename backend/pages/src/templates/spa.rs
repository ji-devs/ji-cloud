use crate::db::{get_course_metadata, get_jig_metadata, get_playlist_metadata, AssetMetadata};
use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Path},
    HttpRequest, HttpResponse,
};
use ji_core::settings::RuntimeSettings;
use shared::{
    config::RemoteTarget,
    domain::{asset::AssetId, course::CourseId, jig::JigId, playlist::PlaylistId},
};
use sqlx::PgPool;
use std::borrow::Cow;
use std::str::FromStr;

use askama::Template;

const DEFAULT_DESCRIPTION: &str = "Jigzi is a game-creation tool and crowd-sourcing platform which currently holds thousands of educational activities that teach children about Judaism, Hebrew, Israel and their culture in an engaging and interactive way. Educators can use current games, as well as create their own to complement their curriculum at any level and any language. The creation tool includes a huge collection of educational clipart that updates constantly.";
const DEFAULT_KEYWORDS: &str = "Jigzi, Judaism, Hebrew, educational, teaching, interactive";

#[derive(Debug, Clone, PartialEq, Copy, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModuleAssetPageKind {
    Edit,
    Play,
}

impl ModuleAssetPageKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Edit => "edit",
            Self::Play => "play",
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
pub enum SpaPage {
    Home,
    User,
    Community,
    Admin,
    Kids,
    Classroom,
    Asset(ModuleAssetPageKind),
    Module(String, ModuleAssetPageKind),
    Dev(String),
    LegacyJig,
}

impl SpaPage {
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Self::Home => Cow::Borrowed("home"),
            Self::User => Cow::Borrowed("user"),
            Self::Community => Cow::Borrowed("community"),
            Self::Admin => Cow::Borrowed("admin"),
            Self::Kids => Cow::Borrowed("kids"),
            Self::Classroom => Cow::Borrowed("classroom"),
            Self::Asset(page_kind) => Cow::Owned(format!("asset/{}", page_kind.as_str())),
            Self::Module(kind, page_kind) => {
                Cow::Owned(format!("module/{}/{}", kind, page_kind.as_str()))
            }
            Self::Dev(path) => Cow::Owned(format!("dev/{}", path)),
            Self::LegacyJig => Cow::Borrowed("legacy/play"),
        }
    }
}

#[derive(Template)]
#[template(path = "spa.html")]
struct SpaPageInfo {
    app_js: String,
    app_css: String,
    app_favicon: String,
    app_custom_elements_js: String,
    metadata: Option<Metadata>,
    firebase: bool,
    google_maps_url: Option<String>,
    local_dev: bool,
    include_hubspot: bool,
    is_release: bool,
}

pub struct Metadata {
    title: String,
    description: String,
    keywords: String,
    url: String,
    image_url: Option<String>,
}

fn spa_template(
    settings: &RuntimeSettings,
    spa: SpaPage,
    metadata: Option<Metadata>,
) -> actix_web::Result<HttpResponse> {
    let google_maps_url = match spa {
        // todo: `Cow::borrowed` ('static)
        SpaPage::User | SpaPage::Community => {
            Some(settings.remote_target().google_maps_url().to_owned())
        }
        _ => None,
    };

    let info = SpaPageInfo {
        app_js: settings
            .remote_target()
            .spa_url(&*spa.as_str(), "js/index.js"),
        app_css: settings.remote_target().static_url("head.css"),
        app_favicon: settings.remote_target().static_url("favicon.ico"),
        app_custom_elements_js: settings
            .remote_target()
            .spa_url(&*spa.as_str(), "elements/custom-elements.js"),
        metadata,
        firebase: matches!(spa, SpaPage::User),
        google_maps_url,
        local_dev: settings.is_local(),
        is_release: matches!(settings.remote_target(), RemoteTarget::Release),
        include_hubspot: match spa {
            SpaPage::Asset(ModuleAssetPageKind::Play) => false,
            SpaPage::Module(_, _) => false,
            SpaPage::LegacyJig => false,
            _ => true,
        },
    };

    let info = info.render().map_err(ErrorInternalServerError)?;

    Ok(actix_web::HttpResponse::Ok().body(info))
}

pub async fn home_template(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Home, None)
}

pub async fn user_template(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::User, None)
}

pub async fn community_template(
    settings: Data<RuntimeSettings>,
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Community, None)
}

pub async fn kids_template(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Kids, None)
}

pub async fn classroom_template(
    settings: Data<RuntimeSettings>,
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Classroom, None)
}

pub async fn admin_template(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Admin, None)
}

pub async fn asset_template(
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    req: HttpRequest,
    path: Path<(ModuleAssetPageKind, String)>,
) -> actix_web::Result<HttpResponse> {
    let (page_kind, asset_path) = path.into_inner();
    let metadata = load_asset_spa_metadata(&settings, &db, &req, page_kind, &asset_path).await;

    spa_template(&settings, SpaPage::Asset(page_kind), metadata)
}

pub async fn legacy_template(
    settings: Data<RuntimeSettings>,
    _path: Path<String>, // (jig_id)
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::LegacyJig, None)
}

pub async fn legacy_template_with_module(
    settings: Data<RuntimeSettings>,
    _path: Path<(String, String)>, // (_jig_id, _module_id)
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::LegacyJig, None)
}

pub async fn module_template(
    settings: Data<RuntimeSettings>,
    path: Path<(String, ModuleAssetPageKind, String)>,
) -> actix_web::Result<HttpResponse> {
    let (module_kind, page_kind, _) = path.into_inner();
    spa_template(&settings, SpaPage::Module(module_kind, page_kind), None)
}

pub async fn dev_template(
    settings: Data<RuntimeSettings>,
    path: Path<String>,
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Dev(path.into_inner()), None)
}

async fn load_asset_spa_metadata(
    settings: &RuntimeSettings,
    db: &PgPool,
    req: &HttpRequest,
    page_kind: ModuleAssetPageKind,
    asset_path: &str,
) -> Option<Metadata> {
    if page_kind != ModuleAssetPageKind::Play {
        return None;
    }

    let asset_id = asset_id_from_asset_path(asset_path)?;
    let metadata = match load_asset_metadata(db, asset_id).await {
        Ok(Some(metadata)) => metadata,
        Ok(None) => return None,
        Err(err) => {
            log::warn!("failed to load asset metadata for {:?}: {}", asset_id, err);
            return None;
        }
    };

    Some(Metadata {
        title: metadata.display_name,
        description: match metadata.description.trim() {
            "" => DEFAULT_DESCRIPTION.to_owned(),
            description => description.to_owned(),
        },
        keywords: match metadata.other_keywords.trim() {
            "" => DEFAULT_KEYWORDS.to_owned(),
            keywords => format!("{DEFAULT_KEYWORDS}, {keywords}"),
        },
        url: format!("{}{}", settings.remote_target().pages_url(), req.uri()),
        image_url: metadata.cover_module_id.map(|module_id| {
            format!(
                "{}/screenshot/{}/{}/full.jpg",
                settings.remote_target().uploads_url(),
                asset_id.uuid(),
                module_id,
            )
        }),
    })
}

fn asset_id_from_asset_path(asset_path: &str) -> Option<AssetId> {
    let mut parts = asset_path.split('/');

    match (parts.next(), parts.next()) {
        (Some("jig"), Some(jig_id)) => JigId::from_str(jig_id).ok().map(AssetId::JigId),
        (Some("playlist"), Some(playlist_id)) => PlaylistId::from_str(playlist_id)
            .ok()
            .map(AssetId::PlaylistId),
        (Some("course"), Some(course_id)) => {
            CourseId::from_str(course_id).ok().map(AssetId::CourseId)
        }
        _ => None,
    }
}

async fn load_asset_metadata(
    db: &PgPool,
    asset_id: AssetId,
) -> sqlx::Result<Option<AssetMetadata>> {
    match asset_id {
        AssetId::JigId(jig_id) => get_jig_metadata(db, jig_id).await,
        AssetId::PlaylistId(playlist_id) => get_playlist_metadata(db, playlist_id).await,
        AssetId::CourseId(course_id) => get_course_metadata(db, course_id).await,
        AssetId::ResourceId(_) => Ok(None),
    }
}
