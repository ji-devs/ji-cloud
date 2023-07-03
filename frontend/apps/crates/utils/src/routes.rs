use crate::asset::{CoursePlayerOptions, JigPlayerOptions, PlaylistPlayerOptions};
use gloo::utils::window;
use serde::{Deserialize, Serialize};
use shared::domain::billing::{PlanType, SchoolId};
use shared::domain::{
    asset::{AssetId, AssetType, DraftOrLive},
    category::CategoryId,
    circle::CircleId,
    course::{unit::CourseUnitId, CourseId},
    image::{ImageId, ImageSearchQuery},
    jig::JigId,
    meta::{AffiliationId, AgeRangeId},
    module::{ModuleId, ModuleKind},
    playlist::PlaylistId,
    resource::ResourceId,
    session::OAuthUserProfile,
    user::{UserId, UserScope},
};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use web_sys::Url;

use super::unwrap::*;

pub type StringId = String;

#[derive(Debug, Clone)]
pub enum Route {
    NotFound,
    Community(CommunityRoute),
    User(UserRoute),
    Kids(KidsRoute),
    Admin(AdminRoute),
    Home(HomeRoute),
    Asset(AssetRoute),
    Module(ModuleRoute),
    Dev(DevRoute),
}

#[derive(Debug, Clone)]
pub enum HomeRoute {
    Home,
    Search(Option<Box<SearchQueryParams>>),
    Help,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchQueryParams {
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub q: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(default)]
    pub is_rated: Option<bool>,

    #[serde(default)]
    #[serde(serialize_with = "shared::domain::ser::csv_encode_uuids")]
    #[serde(deserialize_with = "shared::domain::ser::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub age_ranges: Vec<AgeRangeId>,

    #[serde(default)]
    #[serde(serialize_with = "shared::domain::ser::csv_encode_uuids")]
    #[serde(deserialize_with = "shared::domain::ser::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub affiliations: Vec<AffiliationId>,

    #[serde(default)]
    #[serde(serialize_with = "shared::domain::ser::csv_encode_uuids")]
    #[serde(deserialize_with = "shared::domain::ser::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<CategoryId>,
}

#[derive(Debug, Clone)]
pub enum UserRoute {
    NoAuth,
    Settings,
    RegisterOauth(OauthData),
    LoginOauth(OauthData),
    Login(LoginQuery),
    Register(RegisterQuery),
    ContinueRegistration(Option<OAuthUserProfile>),
    SendEmailConfirmation(String), //the email address
    VerifyEmail(String),           //the token
    PasswordReset(String),         //the token
    RegisterComplete,
    SchoolStart(PlanType),
    SchoolEnd,
    Subscribe(PlanType),
}

#[derive(Debug, Clone)]
pub enum CommunityRoute {
    Landing,
    Search(Box<CommunitySearchQuery>),
    Members(CommunityMembersRoute),
    Circles(CommunityCirclesRoute),
    Courses,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CommunitySearchQuery {
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub q: String,
}

#[derive(Debug, Clone)]
pub enum CommunityMembersRoute {
    List,
    Member(UserId),
}

#[derive(Debug, Clone)]
pub enum CommunityCirclesRoute {
    List,
    Circle(CircleId),
}

#[derive(Debug, Clone)]
pub enum KidsRoute {
    StudentCode(Option<String>),
}

#[derive(Debug, Clone)]
pub enum AdminRoute {
    Landing,
    Categories,
    Locale,
    JigCuration(AdminJigCurationRoute),
    ResourceCuration(AdminResourceCurationRoute),
    Schools(AdminSchoolsRoute),
    Images,
    Users(AdminUsersRoute),
    ImageSearch(Option<ImageSearchQuery>),
    ImageAdd,
    ImageTags,
    ImageMeta(ImageId, bool), //flag is for if it's a new image
    Export,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegisterQuery {
    /// user tried logging in before creating an account
    #[serde(default)]
    pub login_before_register: bool,
}

impl RegisterQuery {
    pub fn login_before_register() -> Self {
        Self {
            login_before_register: true,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoginQuery {
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub redirect: String,

    /// user with basic auth tried using OAuth
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub basic_tried_oauth: bool,
}

impl LoginQuery {
    pub fn redirect(redirect: String) -> Self {
        Self {
            redirect,
            ..Default::default()
        }
    }
    pub fn basic_tried_oauth() -> Self {
        Self {
            basic_tried_oauth: true,
            ..Default::default()
        }
    }
}

impl AdminRoute {
    pub fn allowed_user_scope(&self, scopes: &[UserScope]) -> bool {
        if scopes.contains(&UserScope::Admin) {
            return true;
        }

        match self {
            Self::Landing => true,
            Self::Categories => scopes.contains(&UserScope::ManageCategory),
            Self::Locale => false,
            Self::JigCuration(_) => scopes.contains(&UserScope::AdminAsset),
            Self::ResourceCuration(_) => scopes.contains(&UserScope::AdminAsset),
            Self::Images => scopes.contains(&UserScope::AdminAsset),
            Self::Users(_) => scopes.contains(&UserScope::Admin),
            Self::Schools(_) => scopes.contains(&UserScope::Admin),
            Self::ImageSearch(_) | Self::ImageAdd | Self::ImageTags | Self::ImageMeta(_, _) => {
                scopes.contains(&UserScope::ManageImage)
            }
            Self::Export => scopes.contains(&UserScope::Admin),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AdminJigCurationRoute {
    Table,
    Jig(JigId),
}

#[derive(Debug, Clone)]
pub enum AdminUsersRoute {
    Table,
    User(UserId),
}

#[derive(Debug, Clone)]
pub enum AdminResourceCurationRoute {
    Table,
    Resource(ResourceId),
}

#[derive(Debug, Clone)]
pub enum AdminSchoolsRoute {
    Table,
    School(SchoolId),
}

#[derive(Debug, Clone)]
pub enum AssetRoute {
    Studio,
    JigGallery,
    PlaylistGallery,
    ResourceGallery,
    CourseGallery,
    /// Here for compatibility reasons, can probably go away in a few months
    // RedirectToJig(String),
    Edit(AssetEditRoute),
    Play(AssetPlayRoute),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssetEditRoute {
    Jig(JigId, JigEditRoute),
    Resource(ResourceId, ResourceEditRoute),
    Playlist(PlaylistId, PlaylistEditRoute),
    Course(CourseId, CourseEditRoute),
}

#[derive(Debug, Clone, PartialEq)]
pub enum JigEditRoute {
    Landing,
    Module(ModuleId),
    Publish,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceEditRoute {
    Landing,
    Cover(ModuleId),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlaylistEditRoute {
    Landing,
    Cover(ModuleId),
    Publish,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CourseEditRoute {
    Landing,
    Unit(Option<CourseUnitId>),
    Cover(ModuleId),
    Publish,
}

#[derive(Debug, Clone)]
pub enum AssetPlayRoute {
    Jig(JigId, Option<ModuleId>, JigPlayerOptions),
    Playlist(PlaylistId, PlaylistPlayerOptions),
    Course(CourseId, Option<CourseUnitId>, CoursePlayerOptions),
}

#[derive(Debug, Clone)]
pub enum ModuleRoute {
    Edit(ModuleKind, AssetId, ModuleId),
    Play(ModuleKind, AssetId, ModuleId),
}

#[derive(Debug, Clone)]
pub enum DevRoute {
    Showcase(StringId, String),
    Scratch(StringId, String),
}

//Just for serializing across local routes
#[derive(Serialize, Deserialize)]
struct JsonQuery {
    pub data: String, //json-encoded data as-needed
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OauthData {
    Google(OauthCode),
}
pub type OauthCode = String;

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.into();
        write!(f, "{}", s)
    }
}

impl Route {
    // TODO: add docs when to use redirect, push_state, go_to. And should probably have better naming
    pub fn redirect(self) {
        let location = web_sys::window().unwrap_ji().location();
        let s: String = self.into();
        location.set_href(&s).unwrap_ji();
    }

    pub fn push_state(self) {
        let history = web_sys::window().unwrap_ji().history().unwrap_ji();
        let url: String = self.into();
        let _ = history.push_state_with_url(&JsValue::NULL, "", Some(&url));
    }

    pub fn go_to(&self) {
        dominator::routing::go_to_url(&self.to_string());
        window().scroll_to_with_x_and_y(0.0, 0.0);
    }

    pub fn from_url(url: &str) -> Self {
        let url = Url::new(url).unwrap_ji();
        let paths = url.pathname();
        let paths = paths.split('/').into_iter().skip(1).collect::<Vec<_>>();
        let paths = paths.as_slice();
        let params_map = url.search_params();

        let mut params_string = url.search();
        if params_string.len() > 1 {
            // if there's more then one char than it's a '?', so remove it
            params_string = params_string[1..params_string.len()].to_string();
        }

        match paths {
            [""] => Self::Home(HomeRoute::Home),
            ["home", "search"] => {
                let search: SearchQueryParams = serde_qs::from_str(&params_string).unwrap_ji();
                Self::Home(HomeRoute::Search(Some(Box::new(search))))
            }
            ["home", "help"] => Self::Home(HomeRoute::Help),
            ["kids"] => Self::Kids(KidsRoute::StudentCode(None)),
            ["kids", code] => Self::Kids(KidsRoute::StudentCode(Some(code.to_string()))),
            ["dev", "showcase", id] => {
                let page = params_map.get("page").unwrap_or_default();
                Self::Dev(DevRoute::Showcase(id.to_string(), page))
            }
            ["dev", "scratch", id] => {
                let page = params_map.get("page").unwrap_or_default();
                Self::Dev(DevRoute::Scratch(id.to_string(), page))
            }
            ["community"] => Self::Community(CommunityRoute::Landing),
            ["community", "search"] => {
                let search: CommunitySearchQuery = serde_qs::from_str(&params_string).unwrap_ji();
                Self::Community(CommunityRoute::Search(Box::new(search)))
            }
            ["community", "members"] => {
                Self::Community(CommunityRoute::Members(CommunityMembersRoute::List))
            }
            ["community", "members", user_id] => {
                let user_id = UserId::from_str(user_id).unwrap_ji();
                Self::Community(CommunityRoute::Members(CommunityMembersRoute::Member(
                    user_id,
                )))
            }
            ["community", "circles"] => {
                Self::Community(CommunityRoute::Circles(CommunityCirclesRoute::List))
            }
            ["community", "circles", circle_id] => {
                let circle_id = CircleId::from_str(circle_id).unwrap_ji();
                Self::Community(CommunityRoute::Circles(CommunityCirclesRoute::Circle(
                    circle_id,
                )))
            }
            ["community", "courses"] => Self::Community(CommunityRoute::Courses),
            ["user", "settings"] => Self::User(UserRoute::Settings),
            ["user", "login"] => {
                let query = serde_qs::from_str(&params_string).unwrap_ji();
                Self::User(UserRoute::Login(query))
            }
            ["user", "register"] => {
                let query = serde_qs::from_str(&params_string).unwrap_ji();
                Self::User(UserRoute::Register(query))
            }

            ["user", "register-oauth"] => {
                if let Some(code) = params_map.get("code") {
                    let data = OauthData::Google(code);
                    Self::User(UserRoute::RegisterOauth(data))
                } else {
                    Self::User(UserRoute::NoAuth)
                }
            }
            ["user", "login-oauth"] => {
                if let Some(code) = params_map.get("code") {
                    let data = OauthData::Google(code);
                    Self::User(UserRoute::LoginOauth(data))
                } else {
                    Self::User(UserRoute::NoAuth)
                }
            }
            ["user", "continue-registration"] => {
                if params_string.is_empty() {
                    Self::User(UserRoute::ContinueRegistration(None))
                } else {
                    let oauth_profile: OAuthUserProfile =
                        serde_qs::from_str(&params_string).unwrap_ji();
                    Self::User(UserRoute::ContinueRegistration(Some(oauth_profile)))
                }
            }
            ["user", "send-email-confirmation", email] => {
                Self::User(UserRoute::SendEmailConfirmation(email.to_string()))
            }
            ["user", "verify-email", token] => {
                Self::User(UserRoute::VerifyEmail(token.to_string()))
            }
            ["user", "password-reset", token] => {
                Self::User(UserRoute::PasswordReset(token.to_string()))
            }
            ["user", "register-complete"] => Self::User(UserRoute::RegisterComplete),
            ["user", "no-auth"] => Self::User(UserRoute::NoAuth),
            ["user", "school-start", plan_type] => {
                let plan_type = serde_qs::from_str(plan_type).unwrap_ji();
                Self::User(UserRoute::SchoolStart(plan_type))
            }
            ["user", "school-end"] => Self::User(UserRoute::SchoolEnd),
            ["user", "subscribe", plan_type] => {
                let plan_type = serde_qs::from_str(plan_type).unwrap_ji();
                Self::User(UserRoute::Subscribe(plan_type))
            }
            ["admin", "jig-curation"] => {
                Self::Admin(AdminRoute::JigCuration(AdminJigCurationRoute::Table))
            }
            ["admin", "jig-curation", jig_id] => {
                let jig_id = JigId::from_str(jig_id).unwrap_ji();
                Self::Admin(AdminRoute::JigCuration(AdminJigCurationRoute::Jig(jig_id)))
            }
            ["admin", "resource-curation"] => Self::Admin(AdminRoute::ResourceCuration(
                AdminResourceCurationRoute::Table,
            )),
            ["admin", "resource-curation", resource_id] => {
                let resource_id = ResourceId::from_str(resource_id).unwrap_ji();
                Self::Admin(AdminRoute::ResourceCuration(
                    AdminResourceCurationRoute::Resource(resource_id),
                ))
            }
            ["admin", "schools"] => Self::Admin(AdminRoute::Schools(AdminSchoolsRoute::Table)),
            ["admin", "schools", school_id] => {
                let school_id = SchoolId::from_str(school_id).unwrap_ji();
                Self::Admin(AdminRoute::Schools(AdminSchoolsRoute::School(school_id)))
            }
            ["admin", "images"] => Self::Admin(AdminRoute::Images),
            ["admin", "users"] => Self::Admin(AdminRoute::Users(AdminUsersRoute::Table)),
            ["admin", "users", user_id] => {
                let user_id = UserId::from_str(user_id).unwrap_ji();
                Self::Admin(AdminRoute::Users(AdminUsersRoute::User(user_id)))
            }
            ["admin", "locale"] => Self::Admin(AdminRoute::Locale),
            ["admin", "categories"] => Self::Admin(AdminRoute::Categories),
            ["admin", "image-search"] => {
                if params_string.is_empty() {
                    Self::Admin(AdminRoute::ImageSearch(None))
                } else {
                    let search: ImageSearchQuery = serde_qs::from_str(&params_string).unwrap_ji();
                    Self::Admin(AdminRoute::ImageSearch(Some(search)))
                }
            }
            ["admin", "image-add"] => Self::Admin(AdminRoute::ImageAdd),
            ["admin", "image-tags"] => Self::Admin(AdminRoute::ImageTags),
            ["admin", "image-meta", id, flag] => {
                let id = ImageId::from_str(id).unwrap_ji();
                Self::Admin(AdminRoute::ImageMeta(id, bool::from_str(flag).unwrap_ji()))
            }
            ["admin", "export"] => Self::Admin(AdminRoute::Export),
            ["admin"] => Self::Admin(AdminRoute::Landing),
            // ["jig", "edit", path] => Self::Asset(AssetRoute::RedirectToJig(path.to_string())),
            ["asset", "edit", "studio"] => Self::Asset(AssetRoute::Studio),
            ["asset", "edit", "jig-gallery"] => Self::Asset(AssetRoute::JigGallery),
            ["asset", "edit", "playlist-gallery"] => Self::Asset(AssetRoute::PlaylistGallery),
            ["asset", "edit", "resource-gallery"] => Self::Asset(AssetRoute::ResourceGallery),
            ["asset", "edit", "course-gallery"] => Self::Asset(AssetRoute::CourseGallery),
            ["asset", "edit", "jig", jig_id, "publish"] => Self::Asset(AssetRoute::Edit(
                AssetEditRoute::Jig(JigId::from_str(jig_id).unwrap_ji(), JigEditRoute::Publish),
            )),
            ["asset", "edit", "jig", "debug"] => Self::Asset(AssetRoute::Edit(
                AssetEditRoute::Jig(JigId::from_u128(0), JigEditRoute::Landing),
            )),
            ["asset", "edit", "jig", jig_id] => Self::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                JigId::from_str(jig_id).unwrap_ji(),
                JigEditRoute::Landing,
            ))),
            ["asset", "edit", "jig", jig_id, module_id] => {
                Self::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                    JigId::from_str(jig_id).unwrap_ji(),
                    JigEditRoute::Module(ModuleId::from_str(module_id).unwrap_ji()),
                )))
            }
            ["asset", "edit", "resource", "debug"] => Self::Asset(AssetRoute::Edit(
                AssetEditRoute::Resource(ResourceId::from_u128(0), ResourceEditRoute::Landing),
            )),
            ["asset", "edit", "resource", resource_id] => {
                Self::Asset(AssetRoute::Edit(AssetEditRoute::Resource(
                    ResourceId::from_str(resource_id).unwrap_ji(),
                    ResourceEditRoute::Landing,
                )))
            }
            ["asset", "edit", "resource", resource_id, "cover", cover_id] => {
                Self::Asset(AssetRoute::Edit(AssetEditRoute::Resource(
                    ResourceId::from_str(resource_id).unwrap_ji(),
                    ResourceEditRoute::Cover(ModuleId::from_str(cover_id).unwrap_ji()),
                )))
            }
            ["asset", "edit", "playlist", playlist_id, "publish"] => {
                Self::Asset(AssetRoute::Edit(AssetEditRoute::Playlist(
                    PlaylistId::from_str(playlist_id).unwrap_ji(),
                    PlaylistEditRoute::Publish,
                )))
            }
            ["asset", "edit", "playlist", "debug"] => Self::Asset(AssetRoute::Edit(
                AssetEditRoute::Playlist(PlaylistId::from_u128(0), PlaylistEditRoute::Landing),
            )),
            ["asset", "edit", "playlist", playlist_id] => {
                Self::Asset(AssetRoute::Edit(AssetEditRoute::Playlist(
                    PlaylistId::from_str(playlist_id).unwrap_ji(),
                    PlaylistEditRoute::Landing,
                )))
            }
            ["asset", "edit", "playlist", playlist_id, "cover", cover_id] => {
                Self::Asset(AssetRoute::Edit(AssetEditRoute::Playlist(
                    PlaylistId::from_str(playlist_id).unwrap_ji(),
                    PlaylistEditRoute::Cover(ModuleId::from_str(cover_id).unwrap_ji()),
                )))
            }
            ["asset", "edit", "course", course_id] => {
                Self::Asset(AssetRoute::Edit(AssetEditRoute::Course(
                    CourseId::from_str(course_id).unwrap_ji(),
                    CourseEditRoute::Landing,
                )))
            }
            ["asset", "edit", "course", course_id, "unit"] => {
                Self::Asset(AssetRoute::Edit(AssetEditRoute::Course(
                    CourseId::from_str(course_id).unwrap_ji(),
                    CourseEditRoute::Unit(None),
                )))
            }
            ["asset", "edit", "course", course_id, "unit", unit_id] => {
                Self::Asset(AssetRoute::Edit(AssetEditRoute::Course(
                    CourseId::from_str(course_id).unwrap_ji(),
                    CourseEditRoute::Unit(Some(CourseUnitId::from_str(unit_id).unwrap_ji())),
                )))
            }
            ["asset", "edit", "course", course_id, "cover", cover_id] => {
                Self::Asset(AssetRoute::Edit(AssetEditRoute::Course(
                    CourseId::from_str(course_id).unwrap_ji(),
                    CourseEditRoute::Cover(ModuleId::from_str(cover_id).unwrap_ji()),
                )))
            }
            ["asset", "edit", "course", course_id, "publish"] => {
                Self::Asset(AssetRoute::Edit(AssetEditRoute::Course(
                    CourseId::from_str(course_id).unwrap_ji(),
                    CourseEditRoute::Publish,
                )))
            }
            ["asset", "play", "jig", "debug"] => {
                let search: JigPlayerOptions = serde_qs::from_str(&params_string).unwrap_ji();

                Self::Asset(AssetRoute::Play(AssetPlayRoute::Jig(
                    JigId::from_u128(0),
                    Some(ModuleId::from_u128(0)),
                    search,
                )))
            }
            ["asset", "play", "jig", jig_id] => {
                let mut options: JigPlayerOptions = serde_qs::from_str(&params_string).unwrap_ji();

                // if url param `draft=true` set draft_or_live to draft.
                // Here for legacy reasons, since this was the way we used to specify draft
                if is_param_bool("draft") {
                    options.draft_or_live = DraftOrLive::Draft;
                };

                Self::Asset(AssetRoute::Play(AssetPlayRoute::Jig(
                    JigId::from_str(jig_id).unwrap_ji(),
                    None,
                    options,
                )))
            }

            ["asset", "play", "jig", jig_id, module_id] => {
                let search: JigPlayerOptions = serde_qs::from_str(&params_string).unwrap_ji();

                Self::Asset(AssetRoute::Play(AssetPlayRoute::Jig(
                    JigId::from_str(jig_id).unwrap_ji(),
                    Some(ModuleId::from_str(module_id).unwrap_ji()),
                    search,
                )))
            }

            ["asset", "play", "playlist", playlist_id] => {
                let search: PlaylistPlayerOptions = serde_qs::from_str(&params_string).unwrap_ji();

                Self::Asset(AssetRoute::Play(AssetPlayRoute::Playlist(
                    PlaylistId::from_str(playlist_id).unwrap_ji(),
                    search,
                )))
            }

            ["asset", "play", "course", course_id] => {
                let search: CoursePlayerOptions = serde_qs::from_str(&params_string).unwrap_ji();

                Self::Asset(AssetRoute::Play(AssetPlayRoute::Course(
                    CourseId::from_str(course_id).unwrap_ji(),
                    None,
                    search,
                )))
            }

            ["asset", "play", "course", course_id, "unit", unit_id] => {
                let search: CoursePlayerOptions = serde_qs::from_str(&params_string).unwrap_ji();

                Self::Asset(AssetRoute::Play(AssetPlayRoute::Course(
                    CourseId::from_str(course_id).unwrap_ji(),
                    Some(CourseUnitId::from_str(unit_id).unwrap_ji()),
                    search,
                )))
            }

            ["jig", play_or_edit] | ["jig", play_or_edit, _] | ["jig", play_or_edit, _, _] => {
                let url: String = url.pathname();
                let mut url: Vec<&str> = url.split('/').collect();
                url.remove(0);
                url.remove(0);
                url.remove(0);
                let url = url.join("/");
                let url = format!("/asset/{}/jig/{}", play_or_edit, url);
                let _ = web_sys::window().unwrap_ji().location().set_pathname(&url);

                unreachable!()
            }

            ["module", kind, "edit", asset_type, "debug"]
            | ["module", kind, "edit", asset_type, "debug", "debug"] => {
                let asset_id = AssetType::try_from(*asset_type)
                    .unwrap_ji()
                    .to_asset_id(Uuid::from_u128(0));
                Self::Module(ModuleRoute::Edit(
                    ModuleKind::from_str(kind).expect_ji("unknown module kind!"),
                    asset_id,
                    ModuleId::from_u128(0),
                ))
            }
            ["module", kind, "edit", asset_type, asset_id, module_id] => {
                let asset_id = AssetType::try_from(*asset_type)
                    .unwrap_ji()
                    .to_asset_id(Uuid::from_str(asset_id).unwrap_ji());
                Self::Module(ModuleRoute::Edit(
                    ModuleKind::from_str(kind).expect_ji("unknown module kind!"),
                    asset_id,
                    ModuleId::from_str(module_id).unwrap_ji(),
                ))
            }
            ["module", kind, "play", asset_type, "debug"]
            | ["module", kind, "play", asset_type, "debug", "debug"] => {
                let asset_id = AssetType::try_from(*asset_type)
                    .unwrap_ji()
                    .to_asset_id(Uuid::from_u128(0));
                Self::Module(ModuleRoute::Play(
                    ModuleKind::from_str(kind).expect_ji("unknown module kind!"),
                    asset_id,
                    ModuleId::from_u128(0),
                ))
            }
            ["module", kind, "play", asset_type, asset_id, module_id] => {
                let asset_id = AssetType::try_from(*asset_type)
                    .unwrap_ji()
                    .to_asset_id(Uuid::from_str(asset_id).unwrap_ji());
                Self::Module(ModuleRoute::Play(
                    ModuleKind::from_str(kind).expect_ji("unknown module kind!"),
                    asset_id,
                    ModuleId::from_str(module_id).unwrap_ji(),
                ))
            }

            _ => Self::NotFound,
        }
    }
}

impl From<Route> for String {
    fn from(route: Route) -> Self {
        (&route).into()
    }
}

impl From<&Route> for String {
    fn from(route: &Route) -> Self {
        match route {
            Route::Home(route) => match route {
                HomeRoute::Home => "/".to_string(),
                HomeRoute::Search(search) => match search {
                    None => "/home/search".to_string(),
                    Some(search) => {
                        let query = serde_qs::to_string(&search).unwrap_ji();
                        format!("/home/search?{}", query)
                    }
                },
                HomeRoute::Help => "/home/help".to_string(),
            },
            Route::Kids(route) => match route {
                KidsRoute::StudentCode(code) => match code {
                    Some(code) => format!("/kids/{}", code),
                    None => "/kids".to_string(),
                },
            },
            Route::Community(route) => match route {
                CommunityRoute::Landing => "/community".to_string(),
                CommunityRoute::Search(search) => {
                    let query = serde_qs::to_string(&search).unwrap_ji();
                    format!("/community/search?{}", query)
                }
                CommunityRoute::Members(route) => match route {
                    CommunityMembersRoute::List => "/community/members".to_string(),
                    CommunityMembersRoute::Member(user_id) => {
                        format!("/community/members/{}", user_id)
                    }
                },
                CommunityRoute::Circles(route) => match route {
                    CommunityCirclesRoute::List => "/community/circles".to_string(),
                    CommunityCirclesRoute::Circle(circle_id) => {
                        format!("/community/circles/{}", circle_id.0)
                    }
                },
                CommunityRoute::Courses => "/community/courses".to_string(),
            },
            Route::Dev(route) => match route {
                DevRoute::Showcase(id, page) => format!("/dev/showcase/{}?page={}", id, page),
                DevRoute::Scratch(id, page) => format!("/dev/scratch/{}?page={}", id, page),
            },
            Route::User(route) => match route {
                UserRoute::Settings => "/user/settings".to_string(),
                UserRoute::ContinueRegistration(oauth_profile) => match oauth_profile {
                    None => "/user/continue-registration".to_string(),
                    Some(oauth_profile) => {
                        let query = serde_qs::to_string(&oauth_profile).unwrap_ji();
                        format!("/user/continue-registration?{}", query)
                    }
                },
                UserRoute::Login(redirect) => {
                    let query = serde_qs::to_string(&redirect).unwrap_ji();
                    format!("/user/login?{}", query)
                }
                UserRoute::Register(data) => {
                    let query = serde_qs::to_string(&data).unwrap_ji();
                    format!("/user/register?{}", query)
                }
                UserRoute::RegisterOauth(_) => "/user/register-oauth".to_string(),
                UserRoute::LoginOauth(_) => "/user/login-oauth".to_string(),
                UserRoute::SendEmailConfirmation(email) => {
                    format!("/user/send-email-confirmation/{}", email)
                }
                UserRoute::VerifyEmail(token) => format!("/user/verify-email/{}", token),
                UserRoute::PasswordReset(token) => format!("/user/password-reset/{}", token),
                UserRoute::RegisterComplete => "/user/register-complete".to_string(),
                UserRoute::NoAuth => "/user/no-auth".to_string(),
                UserRoute::SchoolStart(plan_type) => {
                    let query = serde_qs::to_string(&plan_type).unwrap_ji();
                    format!("/user/school-start?{}", query)
                }
                UserRoute::SchoolEnd => "/user/school-end".to_string(),
                UserRoute::Subscribe(plan_type) => {
                    let query = serde_qs::to_string(&plan_type).unwrap_ji();
                    format!("/user/subscribe?{}", query)
                }
            },
            Route::Admin(route) => match route {
                AdminRoute::Landing => "/admin".to_string(),
                AdminRoute::JigCuration(curation_route) => match curation_route {
                    AdminJigCurationRoute::Table => "/admin/jig-curation".to_string(),
                    AdminJigCurationRoute::Jig(jig_id) => {
                        format!("/admin/jig-curation/{}", jig_id.0)
                    }
                },
                AdminRoute::ResourceCuration(curation_route) => match curation_route {
                    AdminResourceCurationRoute::Table => "/admin/resource-curation".to_string(),
                    AdminResourceCurationRoute::Resource(resource_id) => {
                        format!("/admin/resource-curation/{}", resource_id.0)
                    }
                },
                AdminRoute::Schools(schools_route) => match schools_route {
                    AdminSchoolsRoute::Table => "/admin/schools".to_string(),
                    AdminSchoolsRoute::School(school_id) => {
                        format!("/admin/schools/{}", school_id.0)
                    }
                },
                AdminRoute::Images => "/admin/images".to_string(),
                AdminRoute::Users(users_route) => match users_route {
                    AdminUsersRoute::Table => "/admin/users".to_string(),
                    AdminUsersRoute::User(user_id) => {
                        format!("/admin/users/{}", user_id.0)
                    }
                },
                AdminRoute::Locale => "/admin/locale".to_string(),
                AdminRoute::Categories => "/admin/categories".to_string(),
                AdminRoute::ImageSearch(search) => match search {
                    None => "/admin/image-search".to_string(),
                    Some(search) => {
                        let query = serde_qs::to_string(&search).unwrap_ji();
                        format!("/admin/image-search?{}", query)
                    }
                },
                AdminRoute::ImageAdd => "/admin/image-add".to_string(),
                AdminRoute::ImageTags => "/admin/image-tags".to_string(),
                AdminRoute::ImageMeta(id, is_new) => {
                    format!("/admin/image-meta/{}/{}", id.0, is_new)
                }
                AdminRoute::Export => "/admin/export".to_string(),
            },
            Route::Asset(route) => match route {
                AssetRoute::Studio => "/asset/edit/studio".to_string(),
                AssetRoute::JigGallery => "/asset/edit/jig-gallery".to_string(),
                AssetRoute::PlaylistGallery => "/asset/edit/playlist-gallery".to_string(),
                AssetRoute::ResourceGallery => "/asset/edit/resource-gallery".to_string(),
                AssetRoute::CourseGallery => "/asset/edit/course-gallery".to_string(),
                AssetRoute::Edit(route) => match route {
                    AssetEditRoute::Jig(jig_id, route) => match route {
                        JigEditRoute::Landing => {
                            format!("/asset/edit/jig/{}", jig_id.0)
                        }
                        JigEditRoute::Module(module_id) => {
                            format!("/asset/edit/jig/{}/{}", jig_id.0, module_id.0)
                        }
                        JigEditRoute::Publish => {
                            format!("/asset/edit/jig/{}/publish", jig_id.0)
                        }
                    },
                    AssetEditRoute::Resource(resource_id, route) => match route {
                        ResourceEditRoute::Landing => {
                            format!("/asset/edit/resource/{}", resource_id.0)
                        }
                        ResourceEditRoute::Cover(cover_id) => {
                            format!(
                                "/asset/edit/resource/{}/cover/{}",
                                resource_id.0, cover_id.0
                            )
                        }
                    },
                    AssetEditRoute::Playlist(playlist_id, route) => match route {
                        PlaylistEditRoute::Landing => {
                            format!("/asset/edit/playlist/{}", playlist_id.0)
                        }
                        PlaylistEditRoute::Cover(cover_id) => {
                            format!(
                                "/asset/edit/playlist/{}/cover/{}",
                                playlist_id.0, cover_id.0
                            )
                        }
                        PlaylistEditRoute::Publish => {
                            format!("/asset/edit/playlist/{}/publish", playlist_id.0)
                        }
                    },
                    AssetEditRoute::Course(course_id, route) => match route {
                        CourseEditRoute::Landing => {
                            format!("/asset/edit/course/{}", course_id.0)
                        }
                        CourseEditRoute::Unit(unit_id) => match unit_id {
                            Some(unit_id) => {
                                format!("/asset/edit/course/{}/unit/{}", course_id.0, unit_id.0)
                            }
                            None => {
                                format!("/asset/edit/course/{}/unit", course_id.0)
                            }
                        },
                        CourseEditRoute::Cover(cover_id) => {
                            format!("/asset/edit/course/{}/cover/{}", course_id.0, cover_id.0)
                        }
                        CourseEditRoute::Publish => {
                            format!("/asset/edit/course/{}/publish", course_id.0)
                        }
                    },
                },
                AssetRoute::Play(route) => match route {
                    AssetPlayRoute::Jig(jig_id, module_id, player_settings) => {
                        let query = serde_qs::to_string(&player_settings).unwrap_ji();
                        if let Some(module_id) = module_id {
                            format!("/asset/play/jig/{}/{}?{}", jig_id.0, module_id.0, query)
                        } else {
                            format!("/asset/play/jig/{}?{}", jig_id.0, query)
                        }
                    }
                    AssetPlayRoute::Playlist(playlist_id, player_settings) => {
                        let query = serde_qs::to_string(&player_settings).unwrap_ji();
                        format!("/asset/play/playlist/{}?{}", playlist_id.0, query)
                    }
                    AssetPlayRoute::Course(course_id, unit_id, player_settings) => {
                        let query = serde_qs::to_string(&player_settings).unwrap_ji();
                        if let Some(unit_id) = unit_id {
                            format!("/asset/play/course/{}/{}?{}", course_id.0, unit_id.0, query)
                        } else {
                            format!("/asset/play/course/{}?{}", course_id.0, query)
                        }
                    }
                },
            },
            Route::Module(route) => match route {
                ModuleRoute::Edit(kind, asset_id, module_id) => format!(
                    "/module/{}/edit/{}/{}/{}",
                    kind.as_str(),
                    AssetType::from(asset_id),
                    asset_id.uuid(),
                    module_id.0
                ),
                ModuleRoute::Play(kind, asset_id, module_id) => format!(
                    "/module/{}/play/{}/{}/{}",
                    kind.as_str(),
                    AssetType::from(asset_id),
                    asset_id.uuid(),
                    module_id.0
                ),
            },
            Route::NotFound => "/404".to_string(),
        }
    }
}

//todo - rename to get_* for consistency
pub fn is_param_bool(param: &str) -> bool {
    match get_param(param) {
        None => false,
        Some(value) => value == "true",
    }
}
pub fn get_param_index(param: &str) -> Option<usize> {
    get_param(param).and_then(|x| x.parse().ok())
}

pub fn get_param(param: &str) -> Option<String> {
    let url: String = dominator::routing::url().get_cloned();
    let url: web_sys::Url = web_sys::Url::new(&url).unwrap_ji();
    let params = url.search_params();

    params.get(param)
}

/// alternative to dominator::on_click_go_to_url
#[macro_export]
macro_rules! on_click_go_to_url {
    ($this:ident, $route:expr) => {{
        // ($this:ident, $route:ident) => {{
        let route = $route;

        $this.event_with_options(
            &dominator::EventOptions::preventable(),
            move |e: $crate::events::Click| {
                e.prevent_default();
                route.go_to();
            },
        )
    }};
}

/// alternative to dominator::link
#[macro_export]
macro_rules! link {
    ($url:expr, { $($methods:tt)* }) => {{
        let url = $url;

        dominator::html!("a", {
            .attr("href", &url.to_string())
            .apply(move |dom| $crate::on_click_go_to_url!(dom, url))
            $($methods)*
        })
    }};
}
