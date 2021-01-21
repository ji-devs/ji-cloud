use dominator::{Dom, html, clone, with_node};
use futures_signals::signal::Mutable;
use std::rc::Rc;
use super::{state::*, actions};
use web_sys::HtmlInputElement;
use utils::{events, routes::*};
use crate::firebase::*;
use crate::google_maps::*;
use crate::register::state::{Step, Step2Data};

const STR_SUBMIT:&'static str = "Submit";
const STR_LOCATION_LABEL:&'static str = "Location*";
const STR_TERMS_LABEL:&'static str = "I have read the terms and conditions (legal text…)";
const STR_LANGUAGE_LABEL:&'static str = "Preferred language of communication*";
const STR_GDPR_LABEL:&'static str = "I would like to receive educational resources (GDPR legal text….)";

pub struct Step2Page {
}

impl Step2Page {
    pub fn render(step: Mutable<Step>, init_data: Step2Data) -> Dom {
        let state = Rc::new(State::new(step, init_data));

        html!("page-register-step2", {
            .children(&mut [
                // TODO - make custom element
                /*
                html!("input-text", {
                    .property("slot", "location")
                    .property("label", STR_LOCATION_LABEL)
                    .property("mode", "text")
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
                }),
                */
                html!("input-text", {
                    .property("slot", "username")
                    .property("label", STR_LANGUAGE_LABEL)
                    .property("mode", "text")
                }),
                html!("input-checkbox", {
                    .property("slot", "checkbox")
                    .property("label", STR_TERMS_LABEL)
                }),
                html!("input-checkbox", {
                    .property("slot", "checkbox")
                    .property("label", STR_GDPR_LABEL)
                }),
                html!("button-rect", {
                    .property("slot", "submit")
                    .property("color", "red")
                    .property("size", "medium")
                    .text(STR_SUBMIT)
                }),
                html!("footer-register-login", {
                    .property("slot", "footer")
                }),
            ])
        })
            
    }
}

