use crate::prelude::{LoginQuery, Route, UnwrapJiExt, UserRoute};

pub fn navigate_to_login() {
    let location = web_sys::window().unwrap_ji().location();
    let origin = location.origin().unwrap_ji();

    let redirect = format!(
        "{}{}",
        location.pathname().unwrap_ji(),
        location.search().unwrap_ji()
    );

    let route: String = Route::User(UserRoute::Login(LoginQuery::redirect(redirect))).to_string();

    let url = format!("{}{}", origin, route);

    let _ = location.set_href(&url);
}
