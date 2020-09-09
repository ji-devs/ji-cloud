use web_sys::Url;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    NotFound,
    Temp,
    User(UserRoute),
    Admin(AdminRoute)
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
    ImageSummary(String),
}

impl Route {
    pub fn redirect(self) {
        let location = web_sys::window().unwrap_throw().location();
        let s:String = self.into();
        location.set_href(&s).unwrap_throw();
    }

    //TODO - make this and the From for &str via proc-macro so it only needs to be written once
    pub fn from_url(url:&str) -> Self {
        //take into account possibly different hostname
        let url = Url::new(&url).unwrap_throw();
        let uri_parts = get_uri_parts(&url, None);
        let uri = uri_parts.join("/");
        let route = match uri.as_ref() {
            "user/profile" => Some(Self::User(UserRoute::Profile)),
            "user/signin" => Some(Self::User(UserRoute::Signin)),
            "user/register" => Some(Self::User(UserRoute::Register)),
            "admin/categories" => Some(Self::Admin(AdminRoute::Categories)),
            "admin/images" => Some(Self::Admin(AdminRoute::Images)),
            "admin/image-add" => Some(Self::Admin(AdminRoute::ImageAdd)),
            "temp" => Some(Self::Temp),
            _ => None
        };

        if let Some(route) = route {
            return route;
        }

        if uri_parts[0] == "admin" {
            if uri_parts.len() < 3 {
                return Self::NotFound;
            }

            return {
                match uri_parts[1].as_ref() {
                    "image-edit" => Self::Admin(AdminRoute::ImageEdit(uri_parts[2].clone())),
                    "image-summary" => Self::Admin(AdminRoute::ImageSummary(uri_parts[2].clone())),
                    _ => Self::NotFound
                }
            }
        }
        
        Self::NotFound
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
                    AdminRoute::ImageSummary(id) => format!("/admin/image-summary/{}", id),
                }
            },
            Route::NotFound => "/404".to_string(),
            Route::Temp=> "/temp".to_string()
        }
    }
}

fn get_uri_parts(url:&Url, host_url_base: Option<&'static str>) -> Vec<String> {
    let pathname = &url.pathname();

    if pathname == "" {
        Vec::new()
    } else {
        let uri = get_root(pathname, host_url_base);
        if uri == "" {
            vec![]
        } else {
            uri.split("/").map(|s| s.to_string()).collect()
        }
    }
}

//simple stripping of host dir like if deploying to example.com/foo
fn get_root<'a>(input: &'a str, host_url_base: Option<&'static str>) -> &'a str {
    let stripped = match host_url_base {
        None => input,
        Some(host_dir) => {
            input
                .find(host_dir)
                .map(|len| input.split_at(len + host_dir.len() - 1).1)
                .or(Some(input))
                .unwrap()
        }
    };

    stripped.trim_matches('/')
}
