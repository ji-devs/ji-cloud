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
use crate::utils::templates;
use awsm_web::dom::*;
use super::actions::{self, RegisterStatus};
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use core::routes::{Route, UserRoute};

pub struct RegisterPage {
    pub step: Mutable<Step> 
}

#[derive(Clone, Copy, Debug)]
pub enum Step {
    One,
    Two,
    Three,
    ConfirmEmail,
    Final
}
impl RegisterPage  {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
            step: Mutable::new(Step::One) 
        });
        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        html!("div", {
            .child_signal(_self.step.signal_ref(clone!(_self => move |step| {
                Some(match step {
                    Step::One => RegisterStep1::render(RegisterStep1::new()),
                    _ => html!("div", {.text("TODO")})
                })
            })))
        })
    }
}

pub struct RegisterStep1 {
    pub refs: RefCell<Option<RegisterPageRefs>>,
    pub status: Mutable<Option<RegisterStatus>>,
    pub loader: AsyncLoader
}

impl RegisterStep1 {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
            refs: RefCell::new(None),
            status: Mutable::new(None),
            loader: AsyncLoader::new()
        });


        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::register_step1(), {
            .with_data_id!("login-link", {
                .event(clone!(_self => move |_evt:events::Click| {

                    let route:String = Route::User(UserRoute::Signin).into();
                    dominator::routing::go_to_url(&route);
                }))
            })
            .with_data_id!("google-register", {
                .event(clone!(_self => move |evt:events::Click| {
                    /*
                    let tos = _self.refs.borrow();
                    let tos = &tos.as_ref().unwrap_throw().terms_of_service;

                    if !tos.check_validity() {
                        tos.report_validity();
                    } else {
                        _self.status.set(Some(RegisterStatus::Busy));
                        _self.loader.load(actions::register_google(_self.clone()));
                    }
                    */
                    _self.status.set(Some(RegisterStatus::Busy));
                    //_self.loader.load(actions::register_google(_self.clone()));
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
            .after_inserted(clone!(_self => move |elem| {
                _self.stash_refs(elem)
            }))
        })
    }

    fn stash_refs(&self, parent:HtmlElement) {
        *self.refs.borrow_mut() = Some(RegisterPageRefs::new(&parent));
    }

}

pub struct RegisterPageRefs {
    username: HtmlInputElement,
    family_name: HtmlInputElement,
    given_name: HtmlInputElement,
}

impl RegisterPageRefs {
    pub fn new(parent:&HtmlElement) -> Self {
        Self {
            username: parent.select(&data_id("username")),
            family_name: parent.select(&data_id("family-name")),
            given_name: parent.select(&data_id("given-name")),
        }
    }

    pub fn username(&self) -> String {
        self.username.value()
    }
    pub fn family_name(&self) -> String {
        self.family_name.value()
    }
    pub fn given_name(&self) -> String {
        self.given_name.value()
    }

}
