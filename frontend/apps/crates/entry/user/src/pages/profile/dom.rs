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
use super::actions;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use core::routes::{Route, UserRoute};
use shared::domain::user::UserProfile;

pub struct ProfilePage {
    pub status: Mutable<Option<Result<UserProfile, ()>>>,
    pub loader: AsyncLoader
}

impl Drop for ProfilePage {
    fn drop(&mut self) {
        log::info!("cleaned up profile page!");
        //self.signin_loader.cancel();
    }
}

impl ProfilePage {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
            status: Mutable::new(None),
            loader: AsyncLoader::new()
        });

        _self.loader.load(super::actions::load_profile(_self.clone()));

        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::profile(), {
            .with_data_id!("profile", {
                .text_signal(_self.status.signal_ref(|status| {
                    status
                        .as_ref()
                        .map(|status| match status {
                            Ok(profile) => get_profile_string(profile),
                            Err(_) => "not logged in!".to_string()
                        })
                        .unwrap_or("loading...".to_string())
                }))
            })
        })
    }

}

fn get_profile_string(profile:&UserProfile) -> String {
    format!("{:?}", profile)

}


pub struct ProfileEmailChangePage {
}
impl ProfileEmailChangePage {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { });

        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::profile_email_change(), { })
    }

}
