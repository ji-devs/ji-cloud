use web_sys::Url;
use wasm_bindgen::prelude::*;
use shared::domain::{
    image::{ImageId, ImageSearchQuery}, 
    jig::{JigId, JigSearchQuery, module::ModuleId, ModuleKind}, 
    user::UserScope,
    session::OAuthUserProfile
};
use serde::{Serialize, Deserialize};
use std::{fmt::Debug, str::FromStr};
use uuid::Uuid;
use crate::jig::JigPlayerOptions;

use super::unwrap::*;

pub type StringId = String;

#[derive(Debug, Clone)]
pub enum Route {
    NotFound,
    User(UserRoute),
    Kids(KidsRoute),
    Admin(AdminRoute),
    Home(HomeRoute),
    Jig(JigRoute),
    Legacy(LegacyRoute),
    Module(ModuleRoute),
	Dev(DevRoute),
}

#[derive(Debug, Clone)]
pub enum HomeRoute {
    Home,
    Search(Option<JigSearchQuery>),
}

#[derive(Debug, Clone)]
pub enum UserRoute {
    NoAuth,
    Profile(ProfileSection),
    RegisterOauth(OauthData),
    LoginOauth(OauthData),
    Login(String),
    Register,
    ContinueRegistration(Option<OAuthUserProfile>),
    SendEmailConfirmation(String), //the email address
    VerifyEmail(String), //the token 
    PasswordReset(String), //the token 
    RegisterComplete,
}

#[derive(Debug, Clone)]
pub enum KidsRoute {
    StudentCode(Option<String>),
}

#[derive(Debug, Clone)]
pub enum ProfileSection {
    Landing,
    ChangeEmail
}

#[derive(Debug, Clone)]
pub enum AdminRoute {
    Landing,
    Categories,
    Locale,
    Jigs,
    ImageSearch(Option<ImageSearchQuery>),
    ImageAdd,
    ImageTags,
    ImageMeta(ImageId, bool), //flag is for if it's a new image
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
            Self::Jigs => scopes.contains(&UserScope::AdminJig),
            Self::ImageSearch(_) | Self::ImageAdd | Self::ImageTags | Self::ImageMeta(_, _) => {
                scopes.contains(&UserScope::ManageImage)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum LegacyRoute {
    Play(StringId, Option<StringId>) 
}

#[derive(Debug, Clone)]
pub enum JigRoute {
    Gallery,
    Edit(JigId, JigEditRoute),
    Play(JigId, Option<ModuleId>, JigPlayerOptions) 
}

#[derive(Debug, Clone, PartialEq)]
pub enum JigEditRoute {
    Landing,
    Module(ModuleId),
    Publish,
    PostPublish,
}

#[derive(Debug, Clone)]
pub enum JigPlayMode {
    Producer,
    Audience 
}

#[derive(Debug, Clone)]
pub enum ModuleRoute {
    Edit(ModuleKind, JigId, ModuleId),
    Play(ModuleKind, JigId, ModuleId),
}

#[derive(Debug, Clone)]
pub enum DevRoute {
    Showcase(StringId, String),
    Scratch(StringId, String),
}

//Just for serializing across local routes
#[derive(Serialize, Deserialize)]
struct JsonQuery {
    pub data: String  //json-encoded data as-needed
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OauthData {
    Google(OauthCode)
}
pub type OauthCode = String;

impl Route {
    pub fn redirect(self) {
        let location = web_sys::window().unwrap_ji().location();
        let s:String = self.into();
        location.set_href(&s).unwrap_ji();
    }

    pub fn replace_state(self) {
        let history = web_sys::window().unwrap_ji().history().unwrap_ji();
        let url:String = self.into();
        let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&url));
    }

	pub fn to_string(&self) -> String {
		self.into()
	}
	
    pub fn from_url(url:&str) -> Self {
        let url = Url::new(&url).unwrap_ji();
        let paths = url.pathname();
        let paths = paths.split("/").into_iter().skip(1).collect::<Vec<_>>();
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
                let search:JigSearchQuery = serde_qs::from_str(&params_string).unwrap_ji();
                Self::Home(HomeRoute::Search(Some(search)))
            },
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
            ["user", "profile"] => Self::User(UserRoute::Profile(ProfileSection::Landing)),
            ["user", "profile", "change-email"] => Self::User(UserRoute::Profile(ProfileSection::ChangeEmail)),
            ["user", "login"] => {
                let redirect = params_map.get("redirect").unwrap_or_default();
                Self::User(UserRoute::Login(redirect))
            },
            ["user", "register"] => Self::User(UserRoute::Register),

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
                    let oauth_profile:Option<OAuthUserProfile> = serde_qs::from_str(&params_string).unwrap_ji();
                    Self::User(UserRoute::ContinueRegistration(oauth_profile))
                }
            },
            ["user", "send-email-confirmation", email] => Self::User(UserRoute::SendEmailConfirmation(email.to_string())),
            ["user", "verify-email", token] => Self::User(UserRoute::VerifyEmail(token.to_string())),
            ["user", "password-reset", token] => Self::User(UserRoute::PasswordReset(token.to_string())),
            ["user", "register-complete"] => Self::User(UserRoute::RegisterComplete),
            ["user", "no-auth"] => Self::User(UserRoute::NoAuth),
            ["admin", "jigs"] => Self::Admin(AdminRoute::Jigs),
            ["admin", "locale"] => Self::Admin(AdminRoute::Locale),
            ["admin", "categories"] => Self::Admin(AdminRoute::Categories),
            ["admin", "image-search"] => {
                if params_string.is_empty() {
                    Self::Admin(AdminRoute::ImageSearch(None))
                } else {
                    let search:ImageSearchQuery = serde_qs::from_str(&params_string).unwrap_ji();
                    Self::Admin(AdminRoute::ImageSearch(Some(search)))
                }
            },
            ["admin", "image-add"] => Self::Admin(AdminRoute::ImageAdd),
            ["admin", "image-tags"] => Self::Admin(AdminRoute::ImageTags),
            ["admin", "image-meta", id, flag] => {
                let id = ImageId(Uuid::from_str(id).unwrap_ji());
                Self::Admin(AdminRoute::ImageMeta(id, bool::from_str(flag).unwrap_ji()))
            },
            ["admin"] => Self::Admin(AdminRoute::Landing),
            ["jig", "edit", "gallery"] => Self::Jig(JigRoute::Gallery),
            ["jig", "edit", jig_id, "publish"] => Self::Jig(JigRoute::Edit(
                JigId(Uuid::from_str(jig_id).unwrap_ji()),
                JigEditRoute::Publish
            )),
            ["jig", "edit", jig_id, "post-publish"] => Self::Jig(JigRoute::Edit(
                JigId(Uuid::from_str(jig_id).unwrap_ji()),
                JigEditRoute::PostPublish
            )),
            ["jig", "edit", "debug"] => Self::Jig(JigRoute::Edit(
                    JigId(Uuid::from_u128(0)),
                    JigEditRoute::Landing
            )),
            ["jig", "edit", jig_id] => Self::Jig(JigRoute::Edit(
                    JigId(Uuid::from_str(jig_id).unwrap_ji()),
                    JigEditRoute::Landing
            )),
            ["jig", "edit", jig_id, module_id] => Self::Jig(JigRoute::Edit(
                    JigId(Uuid::from_str(jig_id).unwrap_ji()),
                    JigEditRoute::Module(ModuleId(Uuid::from_str(module_id).unwrap_ji()))
            )),
            ["jig", "play", "debug"] => {
                let search:JigPlayerOptions = serde_qs::from_str(&params_string).unwrap_ji();

                Self::Jig(JigRoute::Play(
                    JigId(Uuid::from_u128(0)),
                    Some(ModuleId(Uuid::from_u128(0))),
                    search,
                ))
            },
            ["jig", "play", jig_id] => {
                let search:JigPlayerOptions = serde_qs::from_str(&params_string).unwrap_ji();

                Self::Jig(JigRoute::Play(
                    JigId(Uuid::from_str(jig_id).unwrap_ji()),
                    None,
                    search,
                ))
            },

            ["jig", "play", jig_id, module_id] => {
                let search:JigPlayerOptions = serde_qs::from_str(&params_string).unwrap_ji();

                Self::Jig(JigRoute::Play(
                    JigId(Uuid::from_str(jig_id).unwrap_ji()),
                    Some(ModuleId(Uuid::from_str(module_id).unwrap_ji())),
                    search,
                ))
            },
            ["legacy", "play", jig_id] => Self::Legacy(LegacyRoute::Play(jig_id.to_string(), None)),
            ["legacy", "play", jig_id, module_id] => Self::Legacy(LegacyRoute::Play(jig_id.to_string(), Some(module_id.to_string()))),
            ["module", kind, "edit", "debug"] => {
                Self::Module(ModuleRoute::Edit(
                        ModuleKind::from_str(kind).expect_ji("unknown module kind!"), 
                        JigId(Uuid::from_u128(0)),
                        ModuleId(Uuid::from_u128(0)),
                ))
            },
            ["module", kind, "edit", jig_id, module_id] => {
                Self::Module(ModuleRoute::Edit(
                        ModuleKind::from_str(kind).expect_ji("unknown module kind!"), 
                        JigId(Uuid::from_str(jig_id).unwrap_ji()),
                        ModuleId(Uuid::from_str(module_id).unwrap_ji()),
                ))
            },
            ["module", kind, "play", "debug"] => {
                Self::Module(ModuleRoute::Play(
                        ModuleKind::from_str(kind).expect_ji("unknown module kind!"), 
                        JigId(Uuid::from_u128(0)),
                        ModuleId(Uuid::from_u128(0)),
                ))
            },
            ["module", kind, "play", jig_id, module_id] => {
                Self::Module(ModuleRoute::Play(
                        ModuleKind::from_str(kind).expect_ji("unknown module kind!"), 
                        JigId(Uuid::from_str(jig_id).unwrap_ji()),
                        ModuleId(Uuid::from_str(module_id).unwrap_ji()),
                ))
            },

            _ => Self::NotFound
        }
    }
}

impl From<Route> for String {
	fn from(route:Route) -> Self {
		(&route).into()
	}
}

impl From<&Route> for String {
    fn from(route:&Route) -> Self {
        match route {
			Route::Home(route) => {
                match route {
                    HomeRoute::Home => "/".to_string(),
                    HomeRoute::Search(search) => {
                        match search {
                            None => "/home/search".to_string(),
                            Some(search) => {
                                let query = serde_qs::to_string(&search).unwrap_ji();
                                format!("/home/search?{}", query)
                            }
                        }
                    },
				}
			},
			Route::Kids(route) => {
                match route {
                    KidsRoute::StudentCode(code) => {
                        match code {
                            Some(code) => format!("/kids/{}", code),
                            None => "/kids".to_string(),
                        }
                    },
				}
			},
			Route::Dev(route) => {
                match route {
                    DevRoute::Showcase(id, page) => format!("/dev/showcase/{}?page={}", id, page),
                    DevRoute::Scratch(id, page) => format!("/dev/scratch/{}?page={}", id, page)
				}
			},
            Route::User(route) => {
                match route {
                    UserRoute::Profile(ProfileSection::Landing) => "/user/profile".to_string(),
                    UserRoute::Profile(ProfileSection::ChangeEmail) => "/user/profile/change-email".to_string(),
                    UserRoute::ContinueRegistration(oauth_profile) => {
                        match oauth_profile {
                            None => "/user/continue-registration".to_string(),
                            Some(oauth_profile) => {
                                let query = serde_qs::to_string(&oauth_profile).unwrap_ji();
                                format!("/user/continue-registration?{}", query)
                            }
                        }
                    },
                    UserRoute::Login(redirect) => {
                        format!("/user/login?redirect={}", redirect)
                    },
                    UserRoute::Register => "/user/register".to_string(),
                    UserRoute::RegisterOauth(_) => "/user/register-oauth".to_string(),
                    UserRoute::LoginOauth(_) => "/user/login-oauth".to_string(),
                    UserRoute::SendEmailConfirmation(email) => format!("/user/send-email-confirmation/{}", email),
                    UserRoute::VerifyEmail(token) => format!("/user/verify-email/{}", token),
                    UserRoute::PasswordReset(token) => format!("/user/password-reset/{}", token),
                    UserRoute::RegisterComplete => "/user/register-complete".to_string(),
                    UserRoute::NoAuth => "/user/no-auth".to_string(),
                }
            },
            Route::Admin(route) => {
                match route {
                    AdminRoute::Landing => "/admin".to_string(),
                    AdminRoute::Jigs => "/admin/jigs".to_string(),
                    AdminRoute::Locale => "/admin/locale".to_string(),
                    AdminRoute::Categories => "/admin/categories".to_string(),
                    AdminRoute::ImageSearch(search) => {
                        match search {
                            None => "/admin/image-search".to_string(),
                            Some(search) => {
                                let query = serde_qs::to_string(&search).unwrap_ji();
                                format!("/admin/image-search?{}", query)
                            }
                        }
                    }
                    AdminRoute::ImageAdd => "/admin/image-add".to_string(),
                    AdminRoute::ImageTags => "/admin/image-tags".to_string(),
                    AdminRoute::ImageMeta(id, is_new) => format!("/admin/image-meta/{}/{}", id.0.to_string(), is_new),
                }
            },
            Route::Jig(route) => {
                match route {
                    JigRoute::Gallery => "/jig/edit/gallery".to_string(),
                    JigRoute::Edit(jig_id, route) => {
                        match route {
                            JigEditRoute::Landing => format!("/jig/edit/{}", jig_id.0.to_string()),
                            JigEditRoute::Module(module_id) => format!("/jig/edit/{}/{}", jig_id.0.to_string(), module_id.0.to_string()),
                            JigEditRoute::Publish => format!("/jig/edit/{}/publish", jig_id.0.to_string()),
                            JigEditRoute::PostPublish => format!("/jig/edit/{}/post-publish", jig_id.0.to_string()),
                        }
                    }
                    JigRoute::Play(jig_id, module_id, player_settings) => {
                        let query = serde_qs::to_string(&player_settings).unwrap_ji();
                        if let Some(module_id) = module_id {
                            format!("/jig/play/{}/{}?{}", jig_id.0.to_string(), module_id.0.to_string(), query)
                        } else {
                            format!("/jig/play/{}?{}", jig_id.0.to_string(), query)
                        }
                    }
                }
            },
            Route::Legacy(route) => {
                match route {
                    LegacyRoute::Play(jig_id, module_id) => {
                        if let Some(module_id) = module_id {
                            format!("/legacy/play/{}/{}", jig_id, module_id)
                        } else {
                            format!("/legacy/play/{}", jig_id)
                        }
                    }
                }
            },
            Route::Module(route) => {
                match route {
                    ModuleRoute::Edit(kind, jig_id, module_id) => format!("/module/{}/edit/{}/{}", kind.as_str(), jig_id.0.to_string(), module_id.0.to_string()),
                    ModuleRoute::Play(kind, jig_id, module_id) => format!("/module/{}/play/{}/{}", kind.as_str(), jig_id.0.to_string(), module_id.0.to_string()),
                }
            },
            Route::NotFound => "/404".to_string(),
        }
    }
}


pub fn is_param_bool(param: &str) -> bool { 
    let url:String = dominator::routing::url().get_cloned();
    let url:web_sys::Url = web_sys::Url::new(&url).unwrap_ji();
    let params = url.search_params();

    match params.get(param) {
        None => false,
        Some(value) => {
            if value == "true" {
                true
            } else {
                false
            }
        }
    }
}