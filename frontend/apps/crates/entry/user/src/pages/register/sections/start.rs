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
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::templates;
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use utils::{
    routes::{Route, UserRoute},
    storage,
};
use crate::firebase::*;
use super::super::data::*;

pub struct RegisterStart {
    pub refs: RefCell<Option<RegisterPageRefs>>,
    pub status: Mutable<Option<RegisterStatus>>,
    pub step: Rc<Mutable<Step>>,
    pub data: Rc<RefCell<RegisterData>>,
    reveal_pw: Mutable<bool>,
    pub loader: AsyncLoader
}

impl RegisterStart {
    pub fn new(step:Rc<Mutable<Step>>, data:Rc<RefCell<RegisterData>>) -> Rc<Self> {
        let _self = Rc::new(Self { 
            refs: RefCell::new(None),
            status: Mutable::new(None),
            loader: AsyncLoader::new(),
            reveal_pw: Mutable::new(false),
            data,
            step,
        });


        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::register_start(), {

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
            .with_data_id!("signin-link", {
                .event(clone!(_self => move |_evt:events::Click| {

                    let route:String = Route::User(UserRoute::Signin).into();
                    dominator::routing::go_to_url(&route);
                }))
            })
            .with_data_id!("google-register", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.status.set(Some(RegisterStatus::Busy));
                    _self.loader.load(clone!(_self => async move {
                        match register_google().await {
                            Ok(info) => {
                                let mut data = _self.data.borrow_mut();
                                data.token = Some(info.token);
                                data.email= Some(info.email);
                                data.confirmed_email = info.email_verified;
                                _self.step.set(Step::One);
                            },
                            Err(maybeError) => {
                                _self.status.set(maybeError);
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

            .with_data_id!("submit", {
                .event(clone!(_self => move |evt:events::Click| {
                    let refs = _self.refs.borrow();
                    let refs = refs.as_ref().unwrap_throw();

                
                    match (refs.email(), refs.pw()) {
                        (Err(err), _) | (_, Err(err)) => _self.status.set(Some(err)),

                        (Ok(email), Ok(pw)) => {
                            _self.status.set(Some(RegisterStatus::Busy));
                            _self.loader.load(clone!(_self => async move {
                                match register_email(&email, &pw).await {
                                    Ok(info) => {
                                        let mut data = _self.data.borrow_mut();
                                        data.token = Some(info.token);
                                        data.email= Some(email);
                                        data.confirmed_email = info.email_verified;
                                        _self.step.set(Step::One);
                                    },
                                    Err(err) => {
                                        _self.status.set(Some(err));
                                    }
                                }
                            }));
                        }
                    }
                }))
            })

            .after_inserted(clone!(_self => move |elem| {
                _self.stash_refs(elem);
                //Self::set_debug_data(_self);
            }))
        })
    }

    fn stash_refs(&self, parent:HtmlElement) {
        *self.refs.borrow_mut() = Some(RegisterPageRefs::new(&parent));
    }

}

pub struct RegisterPageRefs {
    email: HtmlInputElement,
    pw: HtmlInputElement,
}

impl RegisterPageRefs {
    pub fn new(parent:&HtmlElement) -> Self {
        Self {
            email: parent.select(&data_id("email")),
            pw: parent.select(&data_id("password")),
        }
    }

    pub fn pw(&self) -> Result<String, RegisterStatus> {
        let pw:String = self.pw.value();

        if pw.is_empty() {
            Err(RegisterStatus::EmptyPw)
        } else {
            Ok(pw)
        }
    }
    pub fn email(&self) -> Result<String, RegisterStatus> {
        let email:String = self.email.value();

        if email.is_empty() {
            Err(RegisterStatus::EmptyEmail)
        } else {
            Ok(email)
        }
    }

}

//Actions
pub async fn register_email(email: &str, pw: &str) -> Result<EmailUserInfo, RegisterStatus> {
    let token_promise = unsafe { firebase_register_email(email, pw) };

    let firebase_res = JsFuture::from(token_promise).await
        .map(|info| {
            let user:EmailUserInfo = serde_wasm_bindgen::from_value(info).unwrap_throw();
            user
        })
        .map_err(|err| {
            match serde_wasm_bindgen::from_value::<FirebaseError>(err) {
                Ok(err) => {
                    let code:&str = err.code.as_ref();
                    match code {
                        "auth/email-already-in-use" => RegisterStatus::EmailExists,
                        "auth/weak-password" => RegisterStatus::PwWeak,
                        _ => {
                            log::warn!("firebase error: {}", code);
                            RegisterStatus::UnknownFirebase
                        }
                    }
                },
                Err(uhh) => {
                    RegisterStatus::Technical
                }
            }
        });

    if let Ok(user) = firebase_res {
        if !firebase_id_exists(user.firebase_id.clone()).await {
            Ok(user)
        } else {
            Err(RegisterStatus::IdExists)
        }
    } else {
        firebase_res
    }
}

pub async fn register_google() -> Result<GoogleUserInfo, Option<RegisterStatus>> {
    let token_promise = unsafe { firebase_register_google() };

    let firebase_res = JsFuture::from(token_promise).await
        .map(|info| {
            serde_wasm_bindgen::from_value::<GoogleUserInfo>(info).unwrap_throw()
        })
        .map_err(|err| {
            None
            /*
            match serde_wasm_bindgen::from_value::<FirebaseError>(err) {
                Ok(err) => {
                    match err.code.as_ref() {
                        "auth/email-already-in-use" => RegisterStatus::EmailExists,
                        "auth/weak-password" => RegisterStatus::PwWeak,
                        _ => RegisterStatus::UnknownFirebase
                    }
                },
                Err(uhh) => {
                    RegisterStatus::Technical
                }
            }
            */
        });


    if let Ok(info) = firebase_res {
        if !firebase_id_exists(info.firebase_id.clone()).await {
            Ok(info)
        } else {
            Err(Some(RegisterStatus::IdExists))
        }
    } else {
        firebase_res
    }
}

use shared::{
    api::{ApiEndpoint, endpoints::user::UserLookup},
    domain::user::{UserLookupQuery, OtherUser},
    error::{auth::RegisterError, user::NoSuchUserError},
};

use utils::{
    fetch::api_no_auth,
};
pub async fn firebase_id_exists(firebase_id:String) -> bool {
    let query = UserLookupQuery {
        id: None,
        firebase_id: Some(firebase_id),
        name: None
    };

    let resp:Result<OtherUser, NoSuchUserError> = api_no_auth(&UserLookup::PATH, UserLookup::METHOD, Some(query)).await;

    resp.is_ok()

}
