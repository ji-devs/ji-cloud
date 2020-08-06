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

pub struct SigninPage {
    pub refs: RefCell<Option<SigninPageRefs>>,
    pub status: Mutable<Option<SigninStatus>>,
    pub signin_loader: AsyncLoader
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
            refs: RefCell::new(None),
            status: Mutable::new(None),
            signin_loader: AsyncLoader::new()
        });


        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::signin(), {
            .with_data_id!("signin", {
                .event(clone!(_self => move |_evt:events::Click| {
                    _self.status.set(Some(SigninStatus::Busy));
                    _self.signin_loader.load(actions::signin_email(_self.clone()));
                }))
            })
            .with_data_id!("google-signin", {
                .event(clone!(_self => move |_evt:events::Click| {
                    _self.status.set(Some(SigninStatus::Busy));
                    _self.signin_loader.load(actions::signin_google(_self.clone()));

                }))
            })
            .after_inserted(clone!(_self => move |elem| {
                _self.stash_refs(elem)
            }))
        })
    }

    fn stash_refs(&self, parent:HtmlElement) {
        *self.refs.borrow_mut() = Some(SigninPageRefs::new(&parent));
    }

}

pub struct SigninPageRefs {
    email: HtmlInputElement,
    pw: HtmlInputElement,
}

impl SigninPageRefs {
    pub fn new(parent:&HtmlElement) -> Self {
        Self {
            email: parent.select(&data_id("email")),
            pw: parent.select(&data_id("pw")),
        }
    }

    pub fn get_email(&self) -> String {
        self.email.value()
    }
    pub fn get_pw(&self) -> String {
        self.pw.value()
    }
}
