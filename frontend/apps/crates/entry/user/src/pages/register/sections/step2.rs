use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
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
use dominator_helpers::{elem, with_data_id};
use crate::templates;
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use utils::{
    routes::{Route, UserRoute},
    fetch::api_with_token,
    storage,
};
use crate::firebase::*;
use super::super::data::*;
use crate::google_maps::*;

pub struct RegisterStep2 {
    pub tos: RefCell<Option<HtmlInputElement>>,
    pub maps_callback: RefCell<Option<Closure<dyn FnMut(String)>>>,
    pub status: Mutable<Option<RegisterStatus>>,
    pub step: Rc<Mutable<Step>>,
    pub data: Rc<RefCell<RegisterData>>,
}

impl RegisterStep2 {
    pub fn new(step:Rc<Mutable<Step>>, data: Rc<RefCell<RegisterData>>) -> Rc<Self> {
        let _self = Rc::new(Self { 
            tos: RefCell::new(None),
            maps_callback: RefCell::new(None),
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
                    } else if data.location_json.is_none() {
                        _self.status.set(Some(RegisterStatus::Location));
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

            .with_data_id!("location" => HtmlInputElement, {
                .after_inserted(clone!(_self => move |elem| {

                    let cb = Closure::wrap(Box::new(clone!(_self => move |location_json:String| {
                        let mut data = _self.data.borrow_mut();
                        if location_json == "" {
                            data.location_json = None;
                        } else {
                            let value:serde_json::Value = serde_json::from_str(&location_json).unwrap_throw();
                            data.location_json = Some(value);
                        }
                    })) as Box<dyn FnMut(String)>);

                    //doesn't actually have to be unsafe but rust-analyzer doesn't like it
                    unsafe { bind_google_maps(elem, &cb); }

                    *_self.maps_callback.borrow_mut() = Some(cb);
                }))

                .event(|_:events::Focus| {
                    //doesn't actually have to be unsafe but rust-analyzer doesn't like it
                    unsafe { geolocate(); }
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

