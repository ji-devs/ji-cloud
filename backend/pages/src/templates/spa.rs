use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Path},
    HttpResponse,
};
use core::settings::RuntimeSettings;
use std::borrow::Cow;

use askama::Template;

#[derive(Debug, Clone, PartialEq, Copy, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModuleJigPageKind {
    Edit,
    Play,
}

impl ModuleJigPageKind {
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
    Admin,
    Kids,
    Jig(ModuleJigPageKind),
    Module(String, ModuleJigPageKind),
    Dev(String),
    LegacyJig,
}

impl SpaPage {
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Self::Home => Cow::Borrowed("home"),
            Self::User => Cow::Borrowed("user"),
            Self::Admin => Cow::Borrowed("admin"),
            Self::Kids => Cow::Borrowed("kids"),
            Self::Jig(page_kind) => Cow::Owned(format!("jig/{}", page_kind.as_str())),
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
    app_custom_elements_js: String,
    firebase: bool,
    google_maps_url: Option<String>,
    local_dev: bool,
    include_hubspot: bool,
}

fn spa_template(settings: &RuntimeSettings, spa: SpaPage) -> actix_web::Result<HttpResponse> {
    let google_maps_url = match spa {
        // todo: `Cow::borrowed` ('static)
        SpaPage::User => Some(settings.remote_target().google_maps_url().to_owned()),
        _ => None,
    };

    let info = SpaPageInfo {
        app_js: settings
            .remote_target()
            .spa_url(&*spa.as_str(), "js/index.js"),
        app_css: settings.remote_target().css_url(true),
        app_custom_elements_js: settings
            .remote_target()
            .spa_url(&*spa.as_str(), "elements/custom-elements.js"),
        firebase: matches!(spa, SpaPage::User),
        google_maps_url,
        local_dev: settings.is_local(),
        include_hubspot: match spa {
            SpaPage::Jig(ModuleJigPageKind::Play) => false,
            SpaPage::Module(_, __) => false,
            SpaPage::LegacyJig => false,
            _ => true,
        },
    };

    let info = info.render().map_err(ErrorInternalServerError)?;

    Ok(actix_web::HttpResponse::Ok().body(info))
}

pub async fn home_template(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Home)
}

pub async fn user_template(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::User)
}

pub async fn kids_template(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Kids)
}

pub async fn admin_template(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Admin)
}

pub async fn jig_template(
    settings: Data<RuntimeSettings>,
    path: Path<(ModuleJigPageKind, String)>,
) -> actix_web::Result<HttpResponse> {
    let (page_kind, _jig_id) = path.into_inner();
    spa_template(&settings, SpaPage::Jig(page_kind))
}

pub async fn jig_template_with_module(
    settings: Data<RuntimeSettings>,
    path: Path<(ModuleJigPageKind, String, String)>,
) -> actix_web::Result<HttpResponse> {
    let (page_kind, _jig_id, _module_id) = path.into_inner();
    spa_template(&settings, SpaPage::Jig(page_kind))
}

pub async fn legacy_template(
    settings: Data<RuntimeSettings>,
    _path: Path<String>, // (jig_id)
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::LegacyJig)
}

pub async fn legacy_template_with_module(
    settings: Data<RuntimeSettings>,
    _path: Path<(String, String)>, // (_jig_id, _module_id)
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::LegacyJig)
}

pub async fn module_template(
    settings: Data<RuntimeSettings>,
    path: Path<(String, ModuleJigPageKind, String, String)>,
) -> actix_web::Result<HttpResponse> {
    let (module_kind, page_kind, _jig_id, _module_id) = path.into_inner();
    spa_template(&settings, SpaPage::Module(module_kind, page_kind))
}

pub async fn dev_template(
    settings: Data<RuntimeSettings>,
    path: Path<String>,
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Dev(path.into_inner()))
}
