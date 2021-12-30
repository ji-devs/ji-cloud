use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{domain::jig::JigBrowseQuery, api::endpoints};
use utils::{prelude::ApiEndpointExt, routes::{AdminCurationRoute, Route, AdminRoute}};

use super::Curation;

impl Curation {
    pub fn load_data(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            join!(
                state.load_jigs(),
                state.load_meta()
            );
        }));
    }

    async fn load_jigs(self: &Rc<Self>) {
        let req = JigBrowseQuery {
            ..Default::default()
        };

        match endpoints::jig::Browse::api_with_auth(Some(req)).await {
            Err(_) => todo!(),
            Ok(resp) => {
                self.jigs.lock_mut().replace_cloned(resp.jigs);
            }
        };
    }

    async fn load_meta(self: &Rc<Self>) {
        match endpoints::meta::Get::api_with_auth(None).await {
            Err(_) => todo!(),
            Ok(meta) => {
                self.ages.set(
                    meta.age_ranges.into_iter().map(|age| {
                        (age.id.clone(), age)
                    }).collect()
                );

                self.goals.set(
                    meta.goals.into_iter().map(|goal| {
                        (goal.id.clone(), goal)
                    }).collect()
                );

                self.affiliations.set(
                    meta.affiliations.into_iter().map(|affiliation| {
                        (affiliation.id.clone(), affiliation)
                    }).collect()
                );
            }
        };
    }

    pub fn navigate_to(self: &Rc<Self>, route: AdminCurationRoute) {
        self.route.set(route.clone());
        Route::Admin(AdminRoute::Curation(route)).push_state();
    }
}
