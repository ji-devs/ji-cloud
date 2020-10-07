use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use shared::{
    api::endpoints::{ApiEndpoint, user::*,},
    domain::auth::{RegisterRequest, RegisterSuccess},
    error::{
        auth::RegisterError,
        user::NoSuchUserError
    }
};
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
    path::api_url,
    routes::{Route, UserRoute},
    fetch::api_with_token,
    storage,
};
use crate::utils::firebase::*;
use super::super::data::*;

pub struct RegisterStep1 {
    pub refs: RefCell<Option<RegisterPageRefs>>,
    pub status: Mutable<Option<RegisterStatus>>,
    pub step: Rc<Mutable<Step>>,
    pub data: Rc<RefCell<RegisterData>>,
    pub loader: AsyncLoader
}

impl RegisterStep1 {
    pub fn new(step:Rc<Mutable<Step>>, data: Rc<RefCell<RegisterData>>) -> Rc<Self> {
        let _self = Rc::new(Self { 
            refs: RefCell::new(None),
            status: Mutable::new(None),
            loader: AsyncLoader::new(),
            data,
            step
        });


        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::register_step1(), {
            .with_data_id!("status-message", {
                .text_signal(_self.status.signal_ref(|status| {
                    status
                        .as_ref()
                        .map(|status| status.to_string())
                        .unwrap_or("".to_string())
                }))
            })

            .with_data_id!("next", {
                .event(clone!(_self => move |evt:events::Click| {
                    let refs = _self.refs.borrow();
                    let refs = refs.as_ref().unwrap_throw();

                
                    match refs.get_basic_info() {
                        Err(err) => _self.status.set(Some(err)),

                        Ok(info) => {
                            _self.status.set(Some(RegisterStatus::Busy));
                            _self.loader.load(clone!(_self => async move {
                                { 
                                    let mut data = _self.data.borrow_mut();
                                    data.user_name = Some(info.user_name);
                                    data.given_name = Some(info.given_name);
                                    data.family_name = Some(info.family_name);
                                }
                                Self::complete_registration(_self.clone(), &_self.data.borrow()).await;
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

    async fn complete_registration(_self: Rc<Self>, data:&RegisterData) {
        let RegisterData { token, email, user_name, given_name, family_name} = data;
        let token = token.as_ref().unwrap_throw().to_string();
        let email = email.as_ref().unwrap_throw().to_string();
        let user_name = user_name.as_ref().unwrap_throw().to_string();
        let given_name = given_name.as_ref().unwrap_throw().to_string();
        let family_name = family_name.as_ref().unwrap_throw().to_string();

        match create_user(token, user_name, given_name, family_name, email).await {
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
    }
    fn stash_refs(&self, parent:HtmlElement) {
        *self.refs.borrow_mut() = Some(RegisterPageRefs::new(&parent));
    }

    fn set_debug_data(_self:Rc<Self>) {
        /*
        let refs = _self.refs.borrow_mut();
        let mut refs = refs.as_ref().unwrap_throw();
        
        refs.given_name.set_value("David");
        refs.family_name.set_value("Komer");
        refs.over_18.set_checked(true);
        refs.user_name.set_value("dakom");
        */
    }

}

pub struct RegisterPageRefs {
    given_name: HtmlInputElement,
    family_name: HtmlInputElement,
    user_name: HtmlInputElement,
    over_18: HtmlInputElement,
}

struct BasicInfo {
    given_name: String, 
    family_name: String, 
    user_name: String, 
}

impl RegisterPageRefs {
    pub fn new(parent:&HtmlElement) -> Self {
        Self {
            given_name: parent.select(&data_id("given-name")),
            family_name: parent.select(&data_id("family-name")),
            user_name: parent.select(&data_id("user-name")),
            over_18: parent.select(&data_id("over-18")),
        }
    }

    pub fn get_basic_info(&self) -> Result<BasicInfo, RegisterStatus> {
        let user_name = self.user_name.value();
        let family_name = self.family_name.value();
        let given_name = self.given_name.value();
        
        if given_name.is_empty() {
            Err(RegisterStatus::EmptyGivenName)
        } else if family_name.is_empty() {
            Err(RegisterStatus::EmptyLastName)
        } else if user_name.is_empty() {
            Err(RegisterStatus::EmptyUserName)
        } else if !self.over_18.check_validity() {
            self.over_18.report_validity();
            Err(RegisterStatus::Over18)
        } else {
            Ok(BasicInfo {
                user_name,
                family_name,
                given_name
            })
        }
    }

    pub fn over_18(&self) -> bool {
        self.over_18.checked()
    }

}

//Actions
pub async fn create_user(
    token: String,
    username: String,
    given_name: String,
    family_name: String,
    email: String,
) -> Result<String, RegisterStatus> {
    let req = RegisterRequest {
        username,
        email,
        given_name,
        family_name,
        over_18: true,
        language: "en".to_string(),
        locale: "en".to_string(),
        timezone: chrono_tz::Tz::Asia__Jerusalem,
        opt_into_edu_resources: true,
        organization: Some("ji".to_string()),
        affiliations: vec![],
        age_ranges: vec![],
        subjects: vec![],
        geocode: None,
    };

    let resp:Result<RegisterSuccess, RegisterError> = api_with_token(&api_url(Register::PATH), &token, Register::METHOD, Some(req)).await;


    match resp {
        Ok(resp) => match resp {
            RegisterSuccess::Signin(csrf) => Ok(csrf),
            RegisterSuccess::ConfirmEmail => Err(RegisterStatus::ConfirmEmail)
        }, 
        Err(err) => {
            let status = match err {
                RegisterError::TakenId => RegisterStatus::IdExists,
                RegisterError::TakenEmail => RegisterStatus::EmailExists,
                RegisterError::EmptyDisplayName => RegisterStatus::EmptyUserName,
                _ => RegisterStatus::Technical
            };

            Err(status)
        }
    }
}
