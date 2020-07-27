use core::routes::{Route, UserRoute};
use gloo_events::EventListener;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use awsm_web::dom::{select, get_element_by_id};
use web_sys::HtmlElement;
use shipyard::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use crate::{
    components::*,
    utils::firebase::get_firebase_signin_google,
};

use core::fetch::user::fetch_signin;

enum Provider {
    Google
}
pub fn init(dom_root:DomRootView, doc:DocumentView, templates: TemplateManagerView, world:WorldView) {

    //let dom_root:DomRootView = storages.borrow();
    //let templates:TemplateManagerView = storages.borrow();
    dom_root.append_child(&templates.signin()).unwrap_throw();

    let elem:HtmlElement = get_element_by_id(&doc, "google-signin");

    let on_click = EventListener::new(&elem, "click", {
        let world = world.clone();
        move |_| {
            world.run_with_data(start, Provider::Google);
            /*
            let state = { *world.borrow::<UniqueView<SigninState>>() };
            match state {
                SigninState::None => {
                    world.run_with_data(start, Provider::Google);
                },
                _ => {
                    log::info!("already busy signing in...");
                }
            }
            */
        }
    });

    let listeners = vec![on_click];

    world.add_unique(SigninState::None);
    world.add_unique_non_send_sync(SigninListeners(listeners));
}

fn start(
    provider:Provider, 
    dom_root:DomRootView, 
    doc:DocumentView, 
    templates: TemplateManagerView,
    mut state:UniqueViewMut<SigninState>,
) {
    *state = SigninState::Signin;
    
    let token_promise = match provider {
        Provider::Google => unsafe { get_firebase_signin_google() }
    };

    spawn_local(async move {
        match JsFuture::from(token_promise).await {
            Ok(token) => {
                let token = token.as_string().unwrap_throw();
                match fetch_signin(&token).await {
                    Ok(status) => {
                        let csrf = &status.csrf;
                        //storage::save_csrf_token(csrf);
                        //Route::User(UserRoute::Profile).redirect();
                        log::info!("logged in! csrf: {}", csrf);
                    },
                    Err(err) => {
                        log::info!("could not log in!");
                    }
                }
            },
            Err(_) => log::info!("error logging in!")
        }
    });
}
