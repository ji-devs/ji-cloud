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
    api::{
        result::ResultResponse,
        endpoints::user::Profile
    },
};
use crate::{
    pages::signin::on_signin_success
};

pub(super) type ProfileResult = Option<Result<User, NoSuchUserError>>;


pub(super) fn profile_signal(profile:&Mutable<ProfileResult>) -> impl Signal<Item = ProfileResult> {
    profile
        .signal_cloned()
        .map_future(|profile:ProfileResult| async move {
            match profile {
                None => Profile::fetch().await,
                Some(profile) => profile
            }
        })
}
