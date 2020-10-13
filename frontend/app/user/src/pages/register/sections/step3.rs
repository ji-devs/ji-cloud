use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use shared::{
    api::endpoints::{ApiEndpoint, user::*,},
    domain::{
        auth::{RegisterRequest, RegisterSuccess},
        meta::*,
    },
    error::{
        auth::RegisterError,
        user::NoSuchUserError
    }
};
use uuid::Uuid;
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

pub struct RegisterStep3 {
    pub status: Mutable<Option<RegisterStatus>>,
    pub step: Rc<Mutable<Step>>,
    pub data: Rc<RefCell<RegisterData>>,
    pub loader: AsyncLoader
}

impl RegisterStep3 {
    pub fn new(step:Rc<Mutable<Step>>, data: Rc<RefCell<RegisterData>>) -> Rc<Self> {
        let _self = Rc::new(Self { 
            status: Mutable::new(None),
            loader: AsyncLoader::new(),
            data,
            step
        });


        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::register_step3(), {
            .with_data_id!("status-message", {
                .text_signal(_self.status.signal_ref(|status| {
                    status
                        .as_ref()
                        .map(|status| status.to_string())
                        .unwrap_or("".to_string())
                }))
            })

            .with_data_id!("complete", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.loader.load(clone!(_self => async move {
                        match create_user(_self.data.borrow().clone()).await {
                            Ok(csrf) => {
                                storage::save_csrf_token(&csrf);

                                _self.step.set(Step::Final);
                                //let route:String = Route::User(UserRoute::Profile).into();
                                //dominator::routing::go_to_url(&route);

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
                    }));
                }))
            })
        })
    }
}


//Actions
pub async fn create_user(data: RegisterData) -> Result<String, RegisterStatus> {
    let req = RegisterRequest {
        username: data.user_name.unwrap_or_default(),
        email: data.email.unwrap_or_default(),
        given_name: data.given_name.unwrap_or_default(),
        family_name: data.family_name.unwrap_or_default(),
        over_18: true,
        language: data.lang.unwrap_or_default(), 
        locale: "en".to_string(),
        timezone: chrono_tz::Tz::Asia__Jerusalem,
        opt_into_edu_resources: data.edu_resources,
        organization: Some("ji".to_string()),
        affiliations: data.affiliations
            .into_iter()
            .map(|id| Uuid::parse_str(&id).unwrap_throw())
            .map(|id| AffiliationId(id))
            .collect(),
        age_ranges: data.age_ranges
            .into_iter()
            .map(|id| Uuid::parse_str(&id).unwrap_throw())
            .map(|id| AgeRangeId(id))
            .collect(),
        subjects: data.subjects
            .into_iter()
            .map(|id| Uuid::parse_str(&id).unwrap_throw())
            .map(|id| SubjectId(id))
            .collect(),
        geocode: data.geocode,
    };

    let resp:Result<RegisterSuccess, RegisterError> = api_with_token(&api_url(Register::PATH), &data.token.unwrap_or_default(), Register::METHOD, Some(req)).await;


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
