use web_sys::Url;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    NotFound,
    Temp,
    User(UserRoute),
    Admin(AdminRoute)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRoute {
    Profile,
    Signin,
    Register,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminRoute {
    Categories,
}

impl Route {
    pub fn redirect(self) {
        web_sys::window()
            .unwrap_throw()
            .location()
            .set_href(self.into())
            .unwrap_throw();
    }

    //TODO - make this and the From for &str via proc-macro so it only needs to be written once
    pub fn from_url(url:&str) -> Self {
        //take into account possibly different hostname
        let url = Url::new(&url).unwrap_throw();
        let uri_parts = get_uri_parts(&url, None);
        let uri = uri_parts.join("/");
        match uri.as_ref() {
            "user/profile" => Self::User(UserRoute::Profile),
            "user/signin" => Self::User(UserRoute::Signin),
            "user/register" => Self::User(UserRoute::Register),
            "admin/categories" => Self::Admin(AdminRoute::Categories),
            "temp" => Self::Temp, 
            _ => Self::NotFound
        }
    }
}


impl From<Route> for &str {
    fn from(route:Route) -> Self {
        match route {
            Route::User(route) => {
                match route {
                    UserRoute::Profile => "/user/profile",
                    UserRoute::Signin => "/user/signin",
                    UserRoute::Register => "/user/register",
                }
            },
            Route::Admin(route) => {
                match route {
                    AdminRoute::Categories=> "/admin/categories",
                }
            },
            Route::NotFound => "/404",
            Route::Temp=> "/temp"
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
