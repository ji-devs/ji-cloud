use utils::{prelude::*, routes::{Route, KidsRoute}};
use shared::{
    api::{endpoints::user::Profile, ApiEndpoint},
    domain::user::UserProfile,
    error::EmptyError,
};
use std::rc::Rc;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt}
};
use dominator::{Dom, html, clone};
use super::student_code;

pub struct Router {
    profile: Mutable<Option<Option<UserProfile>>>,
}

impl Router {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            profile: Mutable::new(None),
        })
    }
}

impl Router {
    pub fn render(state: Rc<Self>) -> Dom {

        html!("div", {
            .future(clone!(state => async move {
                let (result, status) = api_with_auth_status::<UserProfile, EmptyError, ()>(&Profile::PATH, Profile::METHOD, None).await;

                match status  {
                    401 | 403 => {
                        state.profile.set(Some(None));
                    }
                    _ => {
                        match result {
                            Err(_) => {
                                log::info!("error fetching profile");
                            },
                            Ok(profile) => {
                                state.profile.set(Some(Some(profile)));
                            }
                        }
                    }
                };
            }))
            .children_signal_vec(
                map_ref!{
                    let route = dominator::routing::url().signal_ref(|url| Route::from_url(&url)),
                    let profile = state.profile.signal_cloned()
                        => move {
                            let mut children:Vec<Dom> = Vec::new();

                            children.push(components::page_header::dom::render(
                                    Rc::new(components::page_header::state::State::new()),
                                    None
                            ));

                            let dom = match route.clone() {
                                Route::Kids(route) => {
                                    match route.clone() {
                                        KidsRoute::StudentCode => {
                                            Some(student_code::dom::render(Rc::new(student_code::state::State::new())))
                                        },
                                    }
                                }
                                _ => None

                            };

                            if let Some(dom) = dom {
                                children.push(dom);
                            }


                            children
                        }
                }
                .to_signal_vec()
            )
        })
    }
}

