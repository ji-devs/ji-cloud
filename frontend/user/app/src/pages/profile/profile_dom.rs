use std::rc::Rc;
use serde::{Serialize, Deserialize};
use wasm_bindgen::{UnwrapThrowExt, JsCast};
use dominator::{Dom, svg, class, text, html, clone, events, link};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal, MapFuture, MutableSignalCloned },
};
use ji_cloud_shared::{
    user::{UserRole, User, NoSuchUserError},
    auth::{RegisterRequest, RegisterSuccess, RegisterError},
    frontend::fetch
};
use crate::{
    pages::signin::on_signin_success
};
use super::profile::{profile_signal, ProfileResult};


pub struct ProfileDom {
    profile: Mutable<ProfileResult>
}

impl ProfileDom {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            profile: Mutable::new(None),
        })
    }

    pub fn render(comp: Rc<Self>) -> Dom {
        html!("div", {
            .child_signal(
                profile_signal(&comp.profile)
                    .map(clone!(comp => move |profile:ProfileResult| {
                        Some(match profile {
                            None => Self::render_waiting(comp.clone()),
                            Some(profile) => match profile {
                                Ok(profile) => Self::render_profile(comp.clone(), &profile),
                                Err(err) => Self::render_error(comp.clone(), &err)
                            }
                        })
                    }))
            )
        })
    }
    
    fn render_waiting(comp: Rc<Self>) -> Dom {
        html!("div", {.text("waiting for future to resolve")})
    }
    fn render_profile(comp: Rc<Self>, user: &User) -> Dom {
        html!("div", {
            .text(&format!("hello {}!", user.display_name))
        })
    }
    fn render_error(comp: Rc<Self>, err: &NoSuchUserError) -> Dom {
        html!("div", {.text("Got an error!!")})
    }
}
