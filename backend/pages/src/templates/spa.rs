use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Path},
    HttpRequest, HttpResponse,
};
use ji_core::settings::RuntimeSettings;
use shared::{
    config::RemoteTarget,
    domain::jig::{JigId, JigResponse},
};
use std::borrow::Cow;
use std::str::FromStr;
use std::time::Duration;

use askama::Template;

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
    page_title: String,
    page_description: String,
    page_keywords: String,
    page_url: String,
    page_image: Option<String>,
    firebase: bool,
    google_maps_url: Option<String>,
    local_dev: bool,
    include_hubspot: bool,
    is_release: bool,
}

const DEFAULT_PAGE_TITLE: &str = "Jigzi";
const DEFAULT_PAGE_DESCRIPTION: &str = "Jigzi is a game-creation tool and crowd-sourcing platform which currently holds thousands of educational activities that teach children about Judaism, Hebrew, Israel and their culture in an engaging and interactive way. Educators can use current games, as well as create their own to complement their curriculum at any level and any language. The creation tool includes a huge collection of educational clipart that updates constantly.";
const DEFAULT_PAGE_KEYWORDS: &str = "Jigzi, Judaism, Hebrew, educational, teaching, interactive";

#[derive(Clone, Debug)]
struct PageMeta {
    title: String,
    description: String,
    keywords: String,
    image: Option<String>,
}

impl Default for PageMeta {
    fn default() -> Self {
        Self {
            title: DEFAULT_PAGE_TITLE.to_string(),
            description: DEFAULT_PAGE_DESCRIPTION.to_string(),
            keywords: DEFAULT_PAGE_KEYWORDS.to_string(),
            image: None,
        }
    }
}

fn spa_template(
    settings: &RuntimeSettings,
    spa: SpaPage,
    req: &HttpRequest,
    page_meta: PageMeta,
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
        page_title: page_meta.title,
        page_description: page_meta.description,
        page_keywords: page_meta.keywords,
        page_url: format!(
            "{}{}",
            settings.remote_target().pages_url(),
            req.uri()
                .path_and_query()
                .map(|path| path.as_str())
                .unwrap_or("/")
        ),
        page_image: page_meta.image,
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

pub async fn home_template(
    settings: Data<RuntimeSettings>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Home, &req, PageMeta::default())
}

pub async fn user_template(
    settings: Data<RuntimeSettings>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::User, &req, PageMeta::default())
}

pub async fn community_template(
    settings: Data<RuntimeSettings>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Community, &req, PageMeta::default())
}

pub async fn kids_template(
    settings: Data<RuntimeSettings>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Kids, &req, PageMeta::default())
}

pub async fn classroom_template(
    settings: Data<RuntimeSettings>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Classroom, &req, PageMeta::default())
}

pub async fn admin_template(
    settings: Data<RuntimeSettings>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Admin, &req, PageMeta::default())
}

pub async fn asset_template(
    settings: Data<RuntimeSettings>,
    req: HttpRequest,
    path: Path<(ModuleAssetPageKind, String)>,
) -> actix_web::Result<HttpResponse> {
    let (page_kind, asset_path) = path.into_inner();
    let page_meta = match page_kind {
        ModuleAssetPageKind::Play => load_asset_page_meta(&settings, &asset_path).await,
        ModuleAssetPageKind::Edit => PageMeta::default(),
    };

    spa_template(&settings, SpaPage::Asset(page_kind), &req, page_meta)
}

pub async fn legacy_template(
    settings: Data<RuntimeSettings>,
    req: HttpRequest,
    _path: Path<String>, // (jig_id)
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::LegacyJig, &req, PageMeta::default())
}

pub async fn legacy_template_with_module(
    settings: Data<RuntimeSettings>,
    req: HttpRequest,
    _path: Path<(String, String)>, // (_jig_id, _module_id)
) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::LegacyJig, &req, PageMeta::default())
}

pub async fn module_template(
    settings: Data<RuntimeSettings>,
    req: HttpRequest,
    path: Path<(String, ModuleAssetPageKind, String)>,
) -> actix_web::Result<HttpResponse> {
    let (module_kind, page_kind, _) = path.into_inner();
    spa_template(
        &settings,
        SpaPage::Module(module_kind, page_kind),
        &req,
        PageMeta::default(),
    )
}

pub async fn dev_template(
    settings: Data<RuntimeSettings>,
    req: HttpRequest,
    path: Path<String>,
) -> actix_web::Result<HttpResponse> {
    spa_template(
        &settings,
        SpaPage::Dev(path.into_inner()),
        &req,
        PageMeta::default(),
    )
}

async fn load_asset_page_meta(settings: &RuntimeSettings, asset_path: &str) -> PageMeta {
    match parse_jig_id(asset_path) {
        Some(jig_id) => load_jig_page_meta(settings, jig_id).await,
        None => PageMeta::default(),
    }
}

fn parse_jig_id(asset_path: &str) -> Option<JigId> {
    let mut segments = asset_path.split('/');

    match segments.next() {
        Some("jig") => segments.next(),
        Some(id) => Some(id),
        None => None,
    }
    .and_then(|id| JigId::from_str(id).ok())
}

async fn load_jig_page_meta(settings: &RuntimeSettings, jig_id: JigId) -> PageMeta {
    let url = format!(
        "{}/v1/jig/{}/live",
        settings.remote_target().api_url(),
        jig_id
    );

    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
    {
        Ok(client) => client,
        Err(err) => {
            log::warn!(
                "Unable to create HTTP client for social sharing metadata: {:?}",
                err
            );
            return PageMeta::default();
        }
    };

    match client.get(url).send().await {
        Ok(response) if response.status().is_success() => {
            match response.json::<JigResponse>().await {
                Ok(jig) => jig_page_meta(settings, &jig),
                Err(err) => {
                    log::warn!("Unable to parse JIG metadata for social sharing: {:?}", err);
                    PageMeta::default()
                }
            }
        }
        Ok(response) => {
            log::warn!(
                "Unable to load JIG metadata for social sharing: HTTP {}",
                response.status()
            );
            PageMeta::default()
        }
        Err(err) => {
            log::warn!("Unable to load JIG metadata for social sharing: {:?}", err);
            PageMeta::default()
        }
    }
}

fn jig_page_meta(settings: &RuntimeSettings, jig: &JigResponse) -> PageMeta {
    let display_name = jig.jig_data.display_name.trim();
    let title = if display_name.is_empty() {
        DEFAULT_PAGE_TITLE.to_string()
    } else {
        format!("{} | Jigzi", display_name)
    };

    let description = match jig.jig_data.description.trim() {
        "" if display_name.is_empty() => DEFAULT_PAGE_DESCRIPTION.to_string(),
        "" => format!("Play {} on Jigzi.", display_name),
        description => description.to_string(),
    };

    let image = jig.jig_data.modules.first().map(|module| {
        format!(
            "{}/screenshot/{}/{}/full.jpg",
            settings.remote_target().uploads_url(),
            jig.id,
            module.id
        )
    });

    PageMeta {
        title,
        description,
        keywords: DEFAULT_PAGE_KEYWORDS.to_string(),
        image,
    }
}
