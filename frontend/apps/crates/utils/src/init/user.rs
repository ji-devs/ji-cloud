use once_cell::sync::OnceCell;
use shared::{
    api::{endpoints::user::Profile, ApiEndpoint},
    domain::user::UserProfile,
    error::EmptyError,
};
use dominator::clone;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use crate::{
    fetch::*,
    unwrap::*
};

static USER: OnceCell<Mutable<Option<UserProfile>>> = OnceCell::new();

pub(crate) async fn init() {
        
        let (result, status) = Profile::api_with_auth_status(None).await;

        let user = match status  {
            401 | 403 => {
                None
            }
            _ => {
                match result {
                    Err(_) => {
                        log::info!("error fetching profile");
                        None
                    },
                    Ok(user) => {
                        Some(user)
                    }
                }
            }
        };

        USER.set(Mutable::new(user));
}

pub fn set_user(user:Option<UserProfile>) {
    USER.get().unwrap_ji().set(user);
}

pub fn user_signal() -> impl Signal<Item = Option<UserProfile>> {
    USER.get().unwrap_ji().signal_cloned()
}


pub fn user_map_signal_ref<A>(f: impl FnOnce(&UserProfile) -> A + Clone + 'static) -> impl Signal<Item = Option<A>> {
    USER.get().unwrap_ji().signal_ref(clone!(f => move |user| user.as_ref().map(f.clone())))
}

pub fn user_logged_in_signal() -> impl Signal<Item = bool> {
    user_map_signal_ref(|_| ())
        .map(|x| x.is_some())
}

pub fn get_user() -> Option<UserProfile> {
    USER.get().unwrap_ji().get_cloned()
}
