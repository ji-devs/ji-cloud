use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, HtmlInputElement};
use dominator::{Dom, html, events, clone};
use dominator_helpers::{elem, with_data_id, futures::AsyncLoader};
use crate::templates;
use awsm_web::dom::*;
use super::actions::{self, SigninStatus};
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use utils::routes::{Route, UserRoute};

pub struct SigninPage {
    pub status: Mutable<Option<SigninStatus>>,
    pub refs: RefCell<Option<SigninPageRefs>>,
    reveal_pw: Mutable<bool>,
    pub loader: AsyncLoader
}

impl Drop for SigninPage {
    fn drop(&mut self) {
        log::info!("cleaned up signin page!");
        //self.signin_loader.cancel();
    }
}

impl SigninPage {
    pub fn new() -> Rc<Self> {


        let _self = Rc::new(Self { 
            status: Mutable::new(None),
            reveal_pw: Mutable::new(false),
            loader: AsyncLoader::new(),
            refs: RefCell::new(None),
        });


        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::signin(), {
            .future(_self.status.signal_cloned().for_each(|status| {
                if let Some(status) = status {
                    match status {
                        SigninStatus::ConfirmEmail => {
                            let route:String = Route::User(UserRoute::SendEmailConfirmation).into();
                            dominator::routing::go_to_url(&route);
                        },
                        SigninStatus::NoSuchDbUser(user) => {
                            let route:String = Route::User(UserRoute::ContinueRegistration(user)).into();
                            dominator::routing::go_to_url(&route);
                        },
                        _ => {}
                    }
                }
                ready(())
            }))

            .with_data_id!("email-signin", {
                .event(clone!(_self => move |_evt:events::Click| {
                    _self.status.set(Some(SigninStatus::Busy));
                    _self.loader.load(clone!(_self => async move {

                        let SigninInfo {email, password} = _self.refs.borrow().as_ref().unwrap_throw().get_info();
                        match actions::signin_email(&email, &password).await {
                            Ok(csrf) => {
                                actions::do_success(&_self, csrf);
                            },
                            Err(err) => {
                                _self.status.set(err);
                            }
                        }
                    }));
                }))
            })
            .with_data_id!("google-signin", {
                .event(clone!(_self => move |_evt:events::Click| {
                    _self.status.set(Some(SigninStatus::Busy));
                    _self.loader.load(clone!(_self => async move {
                        match actions::signin_google().await {
                            Ok(csrf) => {
                                actions::do_success(&_self, csrf);
                            },
                            Err(err) => {
                                _self.status.set(err);
                            }
                        }
                    }));

                }))
            })
            .with_data_id!("register-link", {
                .event(clone!(_self => move |_evt:events::Click| {

                    let route:String = Route::User(UserRoute::Register).into();
                    dominator::routing::go_to_url(&route);
                }))
            })
            .with_data_id!("forgot-password", {
                .event(clone!(_self => move |_evt:events::Click| {
                    _self.loader.load(clone!(_self => async move {

                        let SigninInfo {email, ..} = _self.refs.borrow().as_ref().unwrap_throw().get_info();
                        match actions::forgot_password(&email).await {
                            Ok(csrf) => {
                                _self.status.set(Some(SigninStatus::PasswordResetSent));
                            },
                            Err(err) => {
                                _self.status.set(Some(err));
                            }
                        }
                    }));
                }))
            })
            .with_data_id!("status-message", {
                .text_signal(_self.status.signal_ref(|status| {
                    status
                        .as_ref()
                        .map(|status| status.to_string())
                        .unwrap_or("".to_string())
                }))
            })

            .with_data_id!("password", {
                .property_signal("type", _self.reveal_pw.signal().map(|reveal| {
                    if reveal { "text" } else { "password" }
                }))
            })
            .with_data_id!("reveal-pw", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.reveal_pw.replace_with(|x| !*x);
                }))
            })
            .after_inserted(clone!(_self => move |elem| {
                *_self.refs.borrow_mut() = Some(SigninPageRefs::new(&elem));
            }))
        })
    }
}
pub struct SigninPageRefs {
    email: HtmlInputElement,
    password: HtmlInputElement,
}

pub struct SigninInfo {
    pub email: String, 
    pub password: String, 
}

impl SigninPageRefs {
    pub fn new(parent:&HtmlElement) -> Self {
        Self {
            email: parent.select(&data_id("email")),
            password: parent.select(&data_id("password")),
        }
    }

    pub fn get_info(&self) -> SigninInfo {
        let email = self.email.value();
        let password = self.password.value();
    
        SigninInfo { email, password } 
    }
}
