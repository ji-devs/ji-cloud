use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use shared::{
    api::endpoints::{ApiEndpoint, user::*, self},
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
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, HtmlInputElement};
use dominator::{Dom, html, events, clone, with_node};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::utils::templates;
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use core::{
    routes::{Route, UserRoute},
    fetch::{api_with_token, api_with_auth},
    storage,
};
use std::collections::HashSet;
use crate::utils::firebase::*;
use super::super::data::*;

pub struct RegisterStep3 {
    pub status: Mutable<Option<RegisterStatus>>,
    pub step: Rc<Mutable<Step>>,
    pub data: Rc<RefCell<RegisterData>>,
    pub age_range_choices: MutableVec<(Id, String)>,
    pub subject_choices: MutableVec<(Id, String)>,
    pub affiliation_choices: MutableVec<(Id, String)>,
    pub loader: AsyncLoader
}

type Id = String;

impl RegisterStep3 {
    pub fn new(step:Rc<Mutable<Step>>, data: Rc<RefCell<RegisterData>>) -> Rc<Self> {
        let _self = Rc::new(Self { 
            status: Mutable::new(None),
            loader: AsyncLoader::new(),
            age_range_choices: MutableVec::new(),
            subject_choices: MutableVec::new(),
            affiliation_choices: MutableVec::new(),
            data,
            step
        });

        let _self_clone = _self.clone();

        spawn_local(async move {
            let options = MetaOptions::load().await.unwrap_throw(); 
            _self.age_range_choices.lock_mut().replace_cloned(options.age_ranges);
            _self.subject_choices.lock_mut().replace_cloned(options.subjects);
            _self.affiliation_choices.lock_mut().replace_cloned(options.affiliations);
        });

        _self_clone
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


            .with_data_id!("subjects", {
                .children_signal_vec(
                    _self.subject_choices.signal_vec_cloned().map(clone!(_self => move |(id, label)| {
                        elem!(templates::checkbox(&id, &label), {
                            .with_data_id!(id => HtmlInputElement, {
                                .with_node!(input => {
                                    .event(clone!(_self => move |_: events::Change| { 
                                        let mut data = _self.data.borrow_mut();
                                        let checked = input.checked(); 
                                        if checked {
                                            data.subjects.insert(id.clone());
                                        } else {
                                            data.subjects.remove(&id);
                                        }
                                    }))
                                })
                            })
                        })
                    }))
                )
            })
            .with_data_id!("affiliations", {
                .children_signal_vec(
                    _self.affiliation_choices.signal_vec_cloned().map(clone!(_self => move |(id, label)| {
                        elem!(templates::checkbox(&id, &label), {
                            .with_data_id!(id => HtmlInputElement, {
                                .with_node!(input => {
                                    .event(clone!(_self => move |_: events::Change| { 
                                        let mut data = _self.data.borrow_mut();
                                        let checked = input.checked(); 
                                        if checked {
                                            data.affiliations.insert(id.clone());
                                        } else {
                                            data.affiliations.remove(&id);
                                        }
                                    }))
                                })
                            })
                        })
                    }))
                )
            })
            .with_data_id!("age_ranges", {
                .children_signal_vec(
                    _self.age_range_choices.signal_vec_cloned().map(clone!(_self => move |(id, label)| {
                        elem!(templates::checkbox(&id, &label), {
                            .with_data_id!(id => HtmlInputElement, {
                                .with_node!(input => {
                                    .event(clone!(_self => move |_: events::Change| { 
                                        let mut data = _self.data.borrow_mut();
                                        let checked = input.checked(); 
                                        if checked {
                                            data.age_ranges.insert(id.clone());
                                        } else {
                                            data.age_ranges.remove(&id);
                                        }
                                    }))
                                })
                            })
                        })
                    }))
                )
            })

            .with_data_id!("complete", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.loader.load(clone!(_self => async move {
                        //TODO - get selected from HashSets
                        match create_user(_self.data.borrow().clone()).await {
                            Ok(csrf) => {

                                if _self.data.borrow().confirmed_email {
                                    storage::save_csrf_token(&csrf);
                                    _self.step.set(Step::Final);
                                } else {
                                    let route:String = Route::User(UserRoute::SendEmailConfirmation).into();
                                    dominator::routing::go_to_url(&route);
                                }
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
        location: data.location_json,
    };

    let resp:Result<RegisterSuccess, RegisterError> = api_with_token(&Register::PATH, &data.token.unwrap_or_default(), Register::METHOD, Some(req)).await;


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


//TODO - move to _core
#[derive(Debug, Clone)]
pub struct MetaOptions {
    pub subjects: Vec<(Id, String)>,
    pub styles: Vec<(Id, String)>,
    pub age_ranges: Vec<(Id, String)>,
    pub affiliations: Vec<(Id, String)>,
}

impl MetaOptions {
    pub async fn load() -> Result<Self, ()> {
        //Probably doesn't need auth - just regular fetch from awsm_web
        let resp:Result<GetResponse, ()> = api_with_auth::<_, _, ()>(&endpoints::meta::Get::PATH, endpoints::meta::Get::METHOD, None).await;
        resp
            .map_err(|err| {
                //log::error!("{:?}", err);
                ()
            })
            .map(|res| {
                Self {
                    subjects: 
                        res.subjects
                            .into_iter()
                            .map(|subject| {
                                let label = subject.display_name; 
                                let id = subject.id.0.to_string();
                                (id, label)
                            })
                            .collect(),
                    styles: 
                        res.styles
                            .into_iter()
                            .map(|style| {
                                let label = style.display_name; 
                                let id = style.id.0.to_string();
                                (id, label)
                            })
                            .collect(),
                    age_ranges: 
                        res.age_ranges
                            .into_iter()
                            .map(|age_range| {
                                let label = age_range.display_name; 
                                let id = age_range.id.0.to_string();
                                (id, label)
                            })
                            .collect(),
                    affiliations: 
                        res.affiliations
                            .into_iter()
                            .map(|affiliation| {
                                let label = affiliation.display_name; 
                                let id = affiliation.id.0.to_string();
                                (id, label)
                            })
                            .collect(),
                }
            })
    }
}
