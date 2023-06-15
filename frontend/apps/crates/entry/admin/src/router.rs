use components::page_header::PageHeader;
use shared::{
    api::endpoints::user::Profile,
    domain::user::{GetProfilePath, UserProfile},
};
use std::rc::Rc;
use utils::{
    component::Component,
    prelude::*,
    routes::{AdminRoute, Route},
};

use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};

use crate::{
    categories::dom::CategoriesPage,
    export::Export,
    image_table::ImageTable,
    images::{
        add::dom::ImageAddPage, meta::dom::ImageMetaPage, search::dom::ImageSearchPage,
        tags::ImageTags,
    },
    jig_curation::JigCuration,
    locale::{dom::LocalePage, state::LoaderState as LocaleLoaderState},
    resource_curation::ResourceCuration,
    schools::Schools,
    sidebar::Sidebar,
    users::Users,
};
use std::cell::RefCell;
pub struct Router {
    app: RefCell<Option<AppState>>,
    profile: Mutable<Option<Option<UserProfile>>>,
}

impl Router {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            app: RefCell::new(None),
            profile: Mutable::new(None),
        })
    }
}

pub enum AppState {
    Locale(Rc<LocaleLoaderState>),
}

impl Router {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("div", {
            .future(clone!(state => async move {
                let (result, status) = Profile::api_with_auth_status(GetProfilePath(), None).await;

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
                    let route = dominator::routing::url().signal_ref(|url| Route::from_url(url)),
                    let profile = state.profile.signal_cloned()
                        => move {
                            // let header = components::page_header::dom::render(
                            //     Rc::new(components::page_header::state::State::new()),
                            //     None,
                            //     None,
                            //     false,
                            // );

                            let header = PageHeader::new(Default::default()).render();

                            let mut children = vec![header];

                            if let Some(profile) = profile {
                                let dom = match route.clone() {
                                    Route::Admin(route) => {
                                        let locked = match profile {
                                            None => true,
                                            Some(user) => !route.allowed_user_scope(&user.scopes)
                                        };

                                        if locked {
                                            Some(state.with_child(route, html!("h1", {
                                                .text("Not Authorized")
                                            })))
                                        } else {
                                            match route.clone() {
                                                AdminRoute::Landing => Some(state.with_child(route, html!("empty-fragment"))),
                                                AdminRoute::Categories=> Some(state.with_child(route, CategoriesPage::render())),
                                                AdminRoute::Locale => {
                                                    let app_state = Rc::new(LocaleLoaderState::new());
                                                    *state.app.borrow_mut() = Some(AppState::Locale(app_state.clone()));
                                                    Some(state.with_child(route, LocalePage::render(app_state)))
                                                },
                                                AdminRoute::ImageAdd => Some(state.with_child(route, ImageAddPage::render())),
                                                AdminRoute::ImageMeta(id, is_new) => Some(state.with_child(route, ImageMetaPage::render(id, is_new))),
                                                AdminRoute::ImageSearch(query) => Some(state.with_child(route, ImageSearchPage::render(query))),
                                                AdminRoute::ImageTags => Some(state.with_child(route, ImageTags::render(ImageTags::new()))),
                                                AdminRoute::Users(users_route) => Some(state.with_child(route, Users::new(users_route).render())),
                                                AdminRoute::JigCuration(curation_route) => Some(state.with_child(route, JigCuration::new(curation_route).render())),
                                                AdminRoute::ResourceCuration(curation_route) => Some(state.with_child(route, ResourceCuration::new(curation_route).render())),
                                                AdminRoute::Schools(schools_route) => Some(state.with_child(route, Schools::new(schools_route).render())),
                                                AdminRoute::Images => Some(state.with_child(route, ImageTable::new().render())),
                                                AdminRoute::Export => Some(state.with_child(route, Export::new().render())),
                                            }
                                        }
                                    }
                                    _ => None

                                };

                                if let Some(dom) = dom {
                                    children.push(dom);
                                }

                            }

                            children
                        }
                }
                .to_signal_vec()
            )
        })
    }

    fn with_child(&self, route: AdminRoute, dom: Dom) -> Dom {
        html!("admin-shell", {
            .child(Sidebar::render(Sidebar::new(route, self.profile.read_only())))
            .child(dom)
        })
    }
}
