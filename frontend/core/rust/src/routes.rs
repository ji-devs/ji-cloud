use web_sys::Url;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    NotFound,
    User(UserRoute),
    Admin(AdminRoute),
    Jig(JigRoute),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRoute {
    Profile,
    Signin,
    Register,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdminRoute {
    Categories,
    Images,
    ImageAdd,
    ImageEdit(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JigRoute {
    Gallery,
    Edit(String),
    Play(String, JigPlayMode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JigPlayMode {
    Producer,
    Audience 
}

impl Route {
    pub fn redirect(self) {
        let location = web_sys::window().unwrap_throw().location();
        let s:String = self.into();
        location.set_href(&s).unwrap_throw();
    }

    pub fn from_url(url:&str) -> Self {
        let url = Url::new(&url).unwrap_throw();
        let paths = url.pathname();
        let paths = paths.split("/").into_iter().skip(1).collect::<Vec<_>>();
        let paths = paths.as_slice();

        log::info!("{:?}", paths);

        match paths {
            ["user", "profile"] => Self::User(UserRoute::Profile),
            ["user", "signin"] => Self::User(UserRoute::Signin),
            ["user", "register"] => Self::User(UserRoute::Register),
            ["admin", "categories"] => Self::Admin(AdminRoute::Categories),
            ["admin", "images"] => Self::Admin(AdminRoute::Images),
            ["admin", "image-add"] => Self::Admin(AdminRoute::ImageAdd),
            ["admin", "image-edit", id] => Self::Admin(AdminRoute::ImageEdit(id.to_string())),
            ["jig", "gallery"] => Self::Jig(JigRoute::Gallery),
            ["jig", "edit", id] => Self::Jig(JigRoute::Edit(id.to_string())),
            ["jig", "play", id] => Self::Jig(JigRoute::Play(id.to_string(), JigPlayMode::Audience)),
            ["jig", "play-producer", id] => Self::Jig(JigRoute::Play(id.to_string(), JigPlayMode::Producer)),
            _ => Self::NotFound
        }
    }
}


impl From<Route> for String {
    fn from(route:Route) -> Self {
        match route {
            Route::User(route) => {
                match route {
                    UserRoute::Profile => "/user/profile".to_string(),
                    UserRoute::Signin => "/user/signin".to_string(),
                    UserRoute::Register => "/user/register".to_string(),
                }
            },
            Route::Admin(route) => {
                match route {
                    AdminRoute::Categories => "/admin/categories".to_string(),
                    AdminRoute::Images => "/admin/images".to_string(),
                    AdminRoute::ImageAdd => "/admin/image-add".to_string(),
                    AdminRoute::ImageEdit(id) => format!("/admin/image-edit/{}", id),
                }
            },
            Route::Jig(route) => {
                match route {
                    JigRoute::Gallery => "/jig/gallery".to_string(),
                    JigRoute::Edit(id) => format!("/jig/edit/{}", id),
                    JigRoute::Play(id, mode) => {
                        match mode {
                            JigPlayMode::Audience => format!("/jig/play/{}", id),
                            JigPlayMode::Producer => format!("/jig/play-producer/{}", id),
                        }
                    }
                }
            },
            Route::NotFound => "/404".to_string(),
        }
    }
}

