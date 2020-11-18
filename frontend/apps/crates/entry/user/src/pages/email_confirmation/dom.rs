use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use utils::settings::SETTINGS;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, HtmlInputElement};
use dominator::{Dom, html, events, clone};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::{templates, firebase::*};
use awsm_web::{
    dom::*,
};
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use utils::routes::*;
use shared::domain::user::UserProfile;
use gloo_timers::future::TimeoutFuture;

pub struct SendEmailConfirmationPage {
    pub status: Mutable<Status>,
    pub loader: AsyncLoader
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    FirstSending,
    AdditionalSending,
    SentBanner,
    SentFinished,
    NoUser,
    UnknownFirebaseError,
    TechnicalError
}

impl Status {
    pub fn from_firebase_err(err:JsValue) -> Self {
        match serde_wasm_bindgen::from_value::<FirebaseError>(err) {
            Ok(err) => {
                let code:&str = err.code.as_ref();
                let status = match code {
                    "internal/no-user" => Self::NoUser,
                    /*
                    "auth/wrong-password" => Self::BadPassword,
                    "auth/user-not-found" => Self::NoSuchUser,
                    "auth/invalid-email" => Self::InvalidEmail,
                    "internal/confirm-email" => Self::ConfirmEmail,
                    */
                    _ => {
                        log::warn!("firebase error: {}", code);
                        Self::UnknownFirebaseError
                    }
                };
                status
            },
            Err(_) => {
                Self::TechnicalError
            }

        }
    }
}
impl SendEmailConfirmationPage {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
            status: Mutable::new(Status::FirstSending),
            loader: AsyncLoader::new()
        });

        let _self_clone = _self.clone();

        _self_clone.loader.load(async move {
            send_email_confirmation(_self.clone()).await;
        });

        _self_clone
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        html!("div", {
            .child_signal(_self.status.signal_ref(clone!(_self => move |status| {
                if *status == Status::FirstSending {
                    None
                } else if *status == Status::NoUser {
                    let route:String = Route::NoAuth.into();
                    dominator::routing::go_to_url(&route);
                    None
                } else {
                    Some(
                        elem!(templates::send_email_confirmation(), {
                            .with_data_id!("sent-notification", {
                                .apply_if(*status != Status::SentBanner, |dom| dom.class("hidden"))
                            })
                            .with_data_id!("status", {
                                .text({
                                    match status {
                                        Status::AdditionalSending => "Re-sending...",
                                        Status::TechnicalError => "Technical error!",
                                        Status::UnknownFirebaseError => "Unknown Firebase error!",
                                        _ => ""
                                    }
                                })
                            })
                            .with_data_id!("resend-email", {
                                .apply_if(*status == Status::SentBanner, |dom| dom.class("hidden"))
                                .event(clone!(_self => move |evt:events::Click| {
                                    _self.loader.load(clone!(_self => async move {
                                        _self.status.set(Status::AdditionalSending);
                                        send_email_confirmation(_self.clone()).await;
                                    }));
                                }))
                            })
                            .with_data_id!("change-email", {
                                .event(|evt:events::Click| {
                                    let route:String = Route::User(UserRoute::Profile(ProfileSection::ChangeEmail)).into();
                                    dominator::routing::go_to_url(&route);
                                })
                            })
                        })
                    )
                }
            })))
        })
    }

}

async fn send_email_confirmation(dom:Rc<SendEmailConfirmationPage>) {
    let base_url = unsafe { SETTINGS.get_unchecked().remote_target.pages_url() };
    let route:String = Route::User(UserRoute::GotEmailConfirmation).into();
    let url = format!("{}{}", base_url, route);
    let token_promise = unsafe { firebase_send_confirmation_email(&url) };
    match JsFuture::from(token_promise).await {
        Ok(_) => {
            dom.status.set(Status::SentBanner);
            TimeoutFuture::new(2_000).await;
            dom.status.set(Status::SentFinished);
        },
        Err(err) => {
            dom.status.set(Status::from_firebase_err(err));
        }
    }

}

pub struct GotEmailConfirmationPage {
}

impl GotEmailConfirmationPage {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { });

        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::got_email_confirmation(), { })
    }

}
