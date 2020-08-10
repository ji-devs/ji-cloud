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
    pub refs: RefCell<Option<RegisterPageRefs>>,
    pub status: Mutable<Option<RegisterStatus>>,
    pub loader: AsyncLoader
}

impl Drop for RegisterPage {
    fn drop(&mut self) {
        log::info!("cleaned up register page!");
        //self.signin_loader.cancel();
    }
}

impl RegisterPage {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
            refs: RefCell::new(None),
            status: Mutable::new(None),
            loader: AsyncLoader::new()
        });


        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::register(), {
            .with_data_id!("login-link", {
                .event(clone!(_self => move |_evt:events::Click| {
                    dominator::routing::go_to_url( Route::User(UserRoute::Signin).into());
                }))
            })
            .with_data_id!("google-register", {
                .event(clone!(_self => move |evt:events::Click| {
                    let tos = _self.refs.borrow();
                    let tos = &tos.as_ref().unwrap_throw().terms_of_service;

                    if !tos.check_validity() {
                        tos.report_validity();
                    } else {
                        _self.status.set(Some(RegisterStatus::Busy));
                        _self.loader.load(actions::register_google(_self.clone()));
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
    terms_of_service: HtmlInputElement,
}

impl RegisterPageRefs {
    pub fn new(parent:&HtmlElement) -> Self {
        Self {
            terms_of_service: parent.select(&data_id("terms_of_service")),
        }
    }

}
