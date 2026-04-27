use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use dominator::clone;
use futures_signals::signal::Mutable;
use shared::{
    api::endpoints::{course, jig, playlist},
    domain::{
        asset::AssetId,
        course::{unit::CourseUnitId, CourseShareUrlPath, CourseShareUrlRequest},
        jig::JigShareUrlPath,
        module::ModuleId,
        playlist::{PlaylistShareUrlPath, PlaylistShareUrlRequest},
    },
};
use utils::asset::{
    AssetPlayerOptions, CoursePlayerOptions, JigPlayerOptions, PlaylistPlayerOptions,
};
use utils::routes::{AssetPlayRoute, AssetRoute, Route};
use utils::{bail_on_err, prelude::*};

use super::PreviewPopupCallbacks;

pub struct PlayerPopup {
    pub asset_id: AssetId,
    pub module_id: Option<ModuleId>,
    pub unit_id: Option<CourseUnitId>,
    pub player_options: AssetPlayerOptions,
    pub signed_url: Mutable<Option<String>>,
    pub loader: AsyncLoader,
    pub open: Mutable<bool>,
    pub callbacks: PreviewPopupCallbacks,
    pub close_button_shown: Mutable<bool>,
}

impl PlayerPopup {
    pub fn new(
        asset_id: AssetId,
        module_id: Option<ModuleId>,
        unit_id: Option<CourseUnitId>,
        player_options: AssetPlayerOptions,
        callbacks: PreviewPopupCallbacks,
    ) -> Rc<Self> {
        Rc::new(Self {
            asset_id,
            module_id,
            player_options,
            signed_url: Mutable::new(None),
            loader: AsyncLoader::new(),
            unit_id,
            open: Mutable::new(true),
            callbacks,
            close_button_shown: Mutable::new(true),
        })
    }

    pub fn new_signed_url(
        asset_id: AssetId,
        signed_url: String,
        player_options: AssetPlayerOptions,
        callbacks: PreviewPopupCallbacks,
    ) -> Rc<Self> {
        Rc::new(Self {
            asset_id,
            module_id: None,
            player_options,
            signed_url: Mutable::new(Some(signed_url)),
            loader: AsyncLoader::new(),
            unit_id: None,
            open: Mutable::new(true),
            callbacks,
            close_button_shown: Mutable::new(true),
        })
    }

    pub fn new_default_player_options(
        asset_id: AssetId,
        callbacks: PreviewPopupCallbacks,
    ) -> Rc<Self> {
        let player_options = match asset_id {
            AssetId::JigId(_) => JigPlayerOptions::default().into(),
            AssetId::PlaylistId(_) => PlaylistPlayerOptions::default().into(),
            AssetId::ResourceId(_) => unreachable!(),
            AssetId::CourseId(_) => CoursePlayerOptions::default().into(),
        };
        Rc::new(Self {
            asset_id,
            module_id: None,
            player_options,
            signed_url: Mutable::new(None),
            loader: AsyncLoader::new(),
            unit_id: None,
            open: Mutable::new(true),
            callbacks,
            close_button_shown: Mutable::new(true),
        })
    }

    pub fn new_default_player_options_with_jig_quota(
        asset_id: AssetId,
        callbacks: PreviewPopupCallbacks,
    ) -> Rc<Self> {
        let player_options = match asset_id {
            AssetId::JigId(_) => JigPlayerOptions {
                quota: true,
                ..Default::default()
            }
            .into(),
            AssetId::PlaylistId(_) => PlaylistPlayerOptions::default().into(),
            AssetId::ResourceId(_) => unreachable!(),
            AssetId::CourseId(_) => CoursePlayerOptions::default().into(),
        };
        Rc::new(Self {
            asset_id,
            module_id: None,
            player_options,
            signed_url: Mutable::new(None),
            loader: AsyncLoader::new(),
            unit_id: None,
            open: Mutable::new(true),
            callbacks,
            close_button_shown: Mutable::new(true),
        })
    }

    pub fn load_signed_url(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let signed_url = if state.player_options.is_draft() {
                Some(state.draft_iframe_url())
            } else {
                match (state.asset_id, state.module_id, &state.player_options, state.unit_id) {
                (AssetId::JigId(jig_id), _module_id, AssetPlayerOptions::Jig(player_options), _unit_id) => {
                    let req = shared::domain::jig::JigShareUrlRequest {
                        direction: player_options.direction,
                        scoring: player_options.scoring,
                    };

                    let res = jig::ShareUrl::api_no_auth(JigShareUrlPath(jig_id), Some(req))
                        .await
                        .toast_on_err();
                    let res = bail_on_err!(res);
                    Some(if player_options.is_student {
                        res.student_share_url
                    } else {
                        res.share_url
                    })
                }
                (AssetId::PlaylistId(playlist_id), _module_id, AssetPlayerOptions::Playlist(_player_options), _unit_id) => {
                    let req = PlaylistShareUrlRequest {};

                    let res = playlist::ShareUrl::api_no_auth(PlaylistShareUrlPath(playlist_id), Some(req))
                        .await
                        .toast_on_err();
                    let res = bail_on_err!(res);
                    Some(res.share_url)
                }
                (AssetId::CourseId(course_id), _module_id, AssetPlayerOptions::Course(_player_options), _unit_id) => {
                    let req = CourseShareUrlRequest {};

                    let res = course::ShareUrl::api_no_auth(CourseShareUrlPath(course_id), Some(req))
                        .await
                        .toast_on_err();
                    let res = bail_on_err!(res);
                    Some(res.share_url)
                }
                _ => {
                    panic!("Invalid asset id/module id/player_options combinations")
                }
                }
            };

            state.signed_url.set(signed_url);
        }));
    }

    fn draft_iframe_url(&self) -> String {
        let route = match (
            self.asset_id,
            self.module_id,
            &self.player_options,
            self.unit_id,
        ) {
            (
                AssetId::JigId(jig_id),
                module_id,
                AssetPlayerOptions::Jig(player_options),
                _unit_id,
            ) => Route::Asset(AssetRoute::Play(AssetPlayRoute::Jig(
                jig_id,
                module_id,
                player_options.clone(),
            ))),
            (
                AssetId::PlaylistId(playlist_id),
                _module_id,
                AssetPlayerOptions::Playlist(player_options),
                _unit_id,
            ) => Route::Asset(AssetRoute::Play(AssetPlayRoute::Playlist(
                playlist_id,
                player_options.clone(),
            ))),
            (
                AssetId::CourseId(course_id),
                _module_id,
                AssetPlayerOptions::Course(player_options),
                unit_id,
            ) => Route::Asset(AssetRoute::Play(AssetPlayRoute::Course(
                course_id,
                unit_id,
                player_options.clone(),
            ))),
            _ => panic!("Invalid asset id/module id/player_options combinations"),
        }
        .to_string();

        SETTINGS.get().unwrap_ji().remote_target.spa_iframe(&route)
    }
}
