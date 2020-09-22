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
use super::actions::{self, SigninStatus};
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use core::routes::{Route, UserRoute};

pub struct SigninPage {
    pub status: Mutable<Option<SigninStatus>>,
    pub loader: AsyncLoader
}

impl Drop for SigninPage {
    fn drop(&mut self) {
        log::info!("cleaned up signin page!");
        //self.signin_loader.cancel();
    }
}

impl SigninPage {
    pub fn new() -> Rc<Self> {


        let _self = Rc::new(Self { 
            status: Mutable::new(None),
            loader: AsyncLoader::new()
        });


        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::signin(), {
            /*
            .with_data_id!("signin", {
                .event(clone!(_self => move |_evt:events::Click| {
                    _self.status.set(Some(SigninStatus::Busy));
                    _self.loader.load(actions::signin_email(_self.clone()));
                }))
            })
            */
            .with_data_id!("google-signin", {
                .event(clone!(_self => move |_evt:events::Click| {
                    _self.status.set(Some(SigninStatus::Busy));
                    _self.loader.load(actions::signin_google(_self.clone()));

                }))
            })
            .with_data_id!("register-link", {
                .event(clone!(_self => move |_evt:events::Click| {

                    let route:String = Route::User(UserRoute::Register).into();
                    dominator::routing::go_to_url(&route);
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
        })
    }


}
