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
use web_sys::{HtmlElement, HtmlInputElement, HtmlSelectElement};
use dominator::{Dom, html, events, clone, with_node};
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

pub struct RegisterStep2 {
    pub tos: RefCell<Option<HtmlInputElement>>,
    pub status: Mutable<Option<RegisterStatus>>,
    pub step: Rc<Mutable<Step>>,
    pub data: Rc<RefCell<RegisterData>>,
}

impl RegisterStep2 {
    pub fn new(step:Rc<Mutable<Step>>, data: Rc<RefCell<RegisterData>>) -> Rc<Self> {
        let _self = Rc::new(Self { 
            tos: RefCell::new(None),
            status: Mutable::new(None),
            data,
            step
        });


        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::register_step2(), {
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

                    let tos = _self.tos.borrow();
                    let tos = tos.as_ref().unwrap_throw();

                    let data = _self.data.borrow();

                    if !tos.check_validity() {
                        tos.report_validity();
                    } else if data.lang.is_none() {
                        _self.status.set(Some(RegisterStatus::Language));
                    } else if data.geocode.is_none() {
                        _self.status.set(Some(RegisterStatus::Geocode));
                    } else {
                        _self.step.set(Step::Three);
                    }
                }))
            })

            .with_data_id!("lang" => HtmlSelectElement, {
                .with_node!(select => {
                    .event(clone!(_self => move |_: events::Change| { 
                        let mut data = _self.data.borrow_mut();
                        log::info!("{}", select.value());
                        data.lang = Some(select.value());
                    }))
                })
            })

            .with_data_id!("geocode" => HtmlInputElement, {
                .with_node!(input => {
                    .event(clone!(_self => move |_: events::Change| { 
                        let mut data = _self.data.borrow_mut();
                        let value = input.value();
                        data.geocode = {
                            if value.is_empty() {
                                None
                            } else {
                                Some(value)
                            }
                        };
                    }))
                })
            })

            .with_data_id!("edu_resources" => HtmlInputElement, {
                .with_node!(input => {
                    .event(clone!(_self => move |_: events::Change| { 
                        let mut data = _self.data.borrow_mut();
                        data.edu_resources = input.checked(); 
                    }))
                })
            })

            .after_inserted(clone!(_self => move |elem| {
                *_self.tos.borrow_mut() = Some(elem.select(&data_id("tos")));
            }))
        })
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
