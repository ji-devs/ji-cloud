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
use crate::utils::templates;
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use core::{
    routes::{Route, UserRoute},
    storage,
};
use crate::utils::firebase::*;
use super::super::data::*;

pub struct RegisterStart {
    pub refs: RefCell<Option<RegisterPageRefs>>,
    pub status: Mutable<Option<RegisterStatus>>,
    pub loader: AsyncLoader
}

impl RegisterStart {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
            refs: RefCell::new(None),
            status: Mutable::new(None),
            loader: AsyncLoader::new()
        });


        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::register_start(), {
            .with_data_id!("signin-link", {
                .event(clone!(_self => move |_evt:events::Click| {

                    let route:String = Route::User(UserRoute::Signin).into();
                    dominator::routing::go_to_url(&route);
                }))
            })
            .with_data_id!("google-register", {
                .event(clone!(_self => move |evt:events::Click| {
                    let refs = _self.refs.borrow();
                    let refs = refs.as_ref().unwrap_throw();
                    match refs.get_basic_info() {
                        Err(err) => _self.status.set(Some(err)),
                        Ok(basic_info) => {
                            _self.status.set(Some(RegisterStatus::Busy));
                            _self.loader.load(clone!(_self => async move {
                                match register_google().await {
                                    Ok(info) => {
                                        Self::complete_registration(_self.clone(), info.token, basic_info, info.email).await;
                                    },
                                    Err(maybeError) => {
                                        _self.status.set(maybeError);
                                    }
                                }
                            }));
                        }
                    }
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

                
                    match (refs.get_basic_info(), refs.pw(), refs.email()) {
                        (Err(err), _, _)
                        | (_, Err(err), _) 
                        | (_, _, Err(err)) => _self.status.set(Some(err)),

                        (Ok(basic_info), Ok(pw), Ok(email)) => {
                            _self.status.set(Some(RegisterStatus::Busy));
                            _self.loader.load(clone!(_self => async move {
                                match register_email(&email, &pw).await {
                                    Ok(token) => {
                                        Self::complete_registration(_self.clone(), token, basic_info, email).await;
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

    async fn complete_registration(_self: Rc<Self>, token:String, basic_info: BasicInfo, email: String) {
        /*
        let BasicInfo {username, given_name, family_name} = basic_info;
        match create_user(token, username, given_name, family_name, email).await {
            Ok(csrf) => {
                storage::save_csrf_token(&csrf);

                let route:String = Route::User(UserRoute::Profile).into();
                dominator::routing::go_to_url(&route);

                ///generally speaking this kind of thing isn't necessary
                ///futures will just resolve and be dropped as part of the flow
                ///but because the oauth flow here opens a separate window
                ///it's more at risk to leave dangling Futures
                ///specifically, here, dangling futures which hold the Rc that holds it
                ///thereby creating a cycle, we need to break by cancelling that future
                ///see: https://github.com/jewish-interactive/ji-cloud/issues/78
                _self.loader.cancel();
            },
            Err(err) => {
                _self.status.set(Some(err));
            }
        }
        */
    }
    fn stash_refs(&self, parent:HtmlElement) {
        *self.refs.borrow_mut() = Some(RegisterPageRefs::new(&parent));
    }

    fn set_debug_data(_self:Rc<Self>) {
        let refs = _self.refs.borrow_mut();
        let mut refs = refs.as_ref().unwrap_throw();

        refs.given_name.set_value("David");
        refs.family_name.set_value("Komer");
        refs.pw.set_value("1234567");
        refs.pw_confirm.set_value("1234567");
        refs.email.set_value("david.komer@gmail.com");
        refs.over_18.set_checked(true);
        refs.username.set_value("dakom");
    }

}

pub struct RegisterPageRefs {
    given_name: HtmlInputElement,
    family_name: HtmlInputElement,
    username: HtmlInputElement,
    pw: HtmlInputElement,
    pw_confirm: HtmlInputElement,
    email: HtmlInputElement,
    over_18: HtmlInputElement,
}

struct BasicInfo {
    given_name: String, 
    family_name: String, 
    username: String, 
}

impl RegisterPageRefs {
    pub fn new(parent:&HtmlElement) -> Self {
        Self {
            given_name: parent.select(&data_id("given-name")),
            family_name: parent.select(&data_id("family-name")),
            username: parent.select(&data_id("username")),
            pw: parent.select(&data_id("pw")),
            pw_confirm: parent.select(&data_id("pw-confirm")),
            email: parent.select(&data_id("email")),
            over_18: parent.select(&data_id("over-18")),
        }
    }

    pub fn get_basic_info(&self) -> Result<BasicInfo, RegisterStatus> {
        let username = self.username.value();
        let family_name = self.family_name.value();
        let given_name = self.given_name.value();
        
        if given_name.is_empty() {
            Err(RegisterStatus::EmptyGivenName)
        } else if family_name.is_empty() {
            Err(RegisterStatus::EmptyLastName)
        } else if username.is_empty() {
            Err(RegisterStatus::EmptyUserName)
        } else if !self.over_18.check_validity() {
            self.over_18.report_validity();
            Err(RegisterStatus::Over18)
        } else {
            Ok(BasicInfo {
                username,
                family_name,
                given_name
            })
        }
    }

    pub fn pw(&self) -> Result<String, RegisterStatus> {
        let pw:String = self.pw.value();
        let pw_confirm:String = self.pw_confirm.value();

        if pw.is_empty() {
            Err(RegisterStatus::EmptyPw)
        } else if pw != pw_confirm {
            Err(RegisterStatus::PwMismatch)
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

    pub fn over_18(&self) -> bool {
        self.over_18.checked()
    }

}

//Actions
pub async fn register_email(email: &str, pw: &str) -> Result<String, RegisterStatus> {
    let token_promise = unsafe { firebase_register_email(email, pw) };

    JsFuture::from(token_promise).await
        .map(|info| {
            let user:EmailRegisterInfo = serde_wasm_bindgen::from_value(info).unwrap_throw();
            user.token
        })
        .map_err(|err| {
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
        })
}

pub async fn register_google() -> Result<GoogleRegisterInfo, Option<RegisterStatus>> {
    let token_promise = unsafe { firebase_register_google() };

    JsFuture::from(token_promise).await
        .map(|info| {
            serde_wasm_bindgen::from_value::<GoogleRegisterInfo>(info).unwrap_throw()
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
        })
}
