use super::state::*;
use dominator::clone;
use futures::join;
use shared::{api::endpoints, domain::asset::AssetId};
use std::rc::Rc;
use utils::prelude::*;

mod jig_actions;

impl Gallery {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            join!(
                state.load_assets(),
                state.load_ages(),
            );
        }));
    }

    async fn load_assets(self: &Rc<Self>) {
        let state = self;
        let is_published = match *state.visible_assets.lock_ref() {
            VisibleAssets::All => None,
            VisibleAssets::Published => Some(true),
            VisibleAssets::Draft => Some(false),
        };

        match jig_actions::load_jigs(state, is_published).await {
            Ok(resp) => {
                // Update the total count and increment the next page so that a future call will
                // call the correct page.
                state.total_asset_count.set(Some(resp.total_jig_count));
                *state.next_page.lock_mut() += 1;

                // Append results to the current list.
                let mut new_list = state.assets.lock_ref().to_vec();
                let mut jigs = resp.jigs.into_iter().map(|jig| jig.into()).collect();
                new_list.append(&mut jigs);

                // Update the list with the new list.
                state.assets.lock_mut().replace_cloned(new_list);
            }
            Err(_) => {
                todo!();
            }
        }
    }

    async fn load_ages(self: &Rc<Self>) {
        let state = Rc::clone(self);
        match endpoints::meta::Get::api_with_auth(None).await {
            Err(_e) => {}
            Ok(res) => {
                state.age_ranges.set(res.age_ranges);
            }
        }
    }

    pub fn search_assets(self: &Rc<Self>, q: String) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let is_published = match *state.visible_assets.lock_ref() {
                VisibleAssets::All => None,
                VisibleAssets::Published => Some(true),
                VisibleAssets::Draft => Some(false),
            };

            let assets = jig_actions::search_jigs(q, is_published)
                .await
                .unwrap_ji();
            state.assets.lock_mut().replace_cloned(assets);
        }));
    }

    pub fn load_assets_regular(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.load_assets().await
        }));
    }

    pub fn create_asset(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            jig_actions::create_jig(state.focus).await;
        }));
    }

    pub fn copy_asset(self: &Rc<Self>, asset_id: AssetId) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            let asset = match asset_id {
                AssetId::JigId(jig_id) => jig_actions::copy_jig(jig_id).await,
                AssetId::CourseId(_) => todo!(),
            };
            state.assets.lock_mut().push_cloned(asset.unwrap_ji());
        }));
    }

    pub fn delete_asset(self: &Rc<Self>, asset_id: AssetId) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let result = match asset_id {
                AssetId::JigId(jig_id) => {
                    jig_actions::delete_jig(jig_id).await
                },
                AssetId::CourseId(_) => todo!(),
            };

            match result {
                Ok(_) => {
                    state.assets.lock_mut().retain(|asset| {
                        asset.id() != asset_id
                    });
                },
                Err(_) => {
                    todo!();
                }
            }
        }));
    }
}
