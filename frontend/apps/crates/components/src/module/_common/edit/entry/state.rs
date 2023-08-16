use dominator::{clone, DomHandle};
use dominator_helpers::signals::EitherSignal;
use futures_signals::signal::{always, Mutable, Signal, SignalExt};
use shared::domain::asset::{Asset, AssetId, AssetType, DraftOrLive, PrivacyLevel};
use shared::domain::course::CourseGetDraftPath;
use shared::domain::playlist::PlaylistGetDraftPath;
use shared::domain::resource::ResourceGetDraftPath;

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::future::Future;
use std::rc::Rc;

use crate::module::_common::edit::history::state::HistoryState;
use dominator_helpers::{futures::AsyncLoader, signals::OptionSignal};

//use super::actions::{HistoryChangeFn, HistoryUndoRedoFn};
use super::{actions::*, base::state::*, choose::state::*};
use shared::{
    api::endpoints::{self, module::*},
    domain::{
        jig::{
            AudioEffects, JigAdminData, JigData, JigGetDraftPath, JigId, JigPlayerSettings,
            JigResponse,
        },
        module::{
            body::{BodyExt, ModeExt, StepExt},
            ModuleBody, ModuleGetDraftPath, ModuleId,
        },
    },
};
use utils::{languages::Language, prelude::*};

use crate::audio::mixer::AUDIO_MIXER;

pub struct GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
{
    pub phase:
        Mutable<Rc<Phase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>>,
    pub(super) asset: RefCell<Option<Asset>>,
    pub(super) opts: StateOpts<RawData>,
    pub(super) raw_loader: AsyncLoader,
    pub(super) screenshot_loader: Rc<AsyncLoader>,
    pub(super) save_loader: Rc<AsyncLoader>,
    pub(super) history: RefCell<Option<Rc<HistoryStateImpl<RawData>>>>,
    pub(super) raw_loaded: Mutable<bool>,
    pub(super) page_body_switcher: AsyncLoader,
    pub(super) dom_body_handle: Mutable<Option<DomHandle>>,
    pub(super) reset_from_history_loader: AsyncLoader,
    pub(super) on_init_ready: RefCell<Option<Box<dyn Fn()>>>,
}

pub enum Phase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
{
    Init,
    Choose(Rc<Choose<RawData, Mode, Step>>),
    Base(Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>),
}

#[derive(Debug, Clone)]
pub struct StateOpts<RawData> {
    pub skip_save_for_debug: bool,
    pub skip_load_jig: bool,
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    //the step which is for previewing
    pub is_main_scrollable: bool,
    pub force_raw: Option<RawData>,
}

impl<RawData> StateOpts<RawData> {
    pub fn new(asset_id: AssetId, module_id: ModuleId) -> Self {
        Self {
            skip_save_for_debug: false,
            skip_load_jig: false,
            asset_id,
            module_id,
            is_main_scrollable: true,
            force_raw: None,
        }
    }
}

/*
 * Note: the idea is to create the top-level state
 * and then pass it down here
 */

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InitSource {
    ForceRaw,
    Load,
    History,
    ChooseMode,
}

impl<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>
    GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>
where
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
    RawData: BodyExt<Mode, Step> + 'static,
{
    pub fn new<BaseInitFromRawFn, BaseInitFromRawOutput>(
        opts: StateOpts<RawData>,
        init_from_raw: BaseInitFromRawFn,
    ) -> Rc<Self>
    where
        BaseInitFromRawFn:
            Fn(BaseInitFromRawArgs<RawData, Mode, Step>) -> BaseInitFromRawOutput + Clone + 'static,
        BaseInitFromRawOutput:
            Future<Output = BaseInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
        <RawData as TryFrom<ModuleBody>>::Error: std::fmt::Debug,
    {
        let _self = Rc::new(Self {
            opts,
            asset: RefCell::new(None),
            phase: Mutable::new(Rc::new(Phase::Init)),
            history: RefCell::new(None),
            raw_loaded: Mutable::new(false),
            raw_loader: AsyncLoader::new(),
            screenshot_loader: Rc::new(AsyncLoader::new()),
            save_loader: Rc::new(AsyncLoader::new()),
            page_body_switcher: AsyncLoader::new(),
            dom_body_handle: Mutable::new(None),
            reset_from_history_loader: AsyncLoader::new(),
            on_init_ready: RefCell::new(None),
        });

        *_self.on_init_ready.borrow_mut() = Some(Box::new(clone!(_self => move || {
            _self.raw_loader.load(clone!(_self, init_from_raw => async move {
                if !_self.opts.skip_load_jig {
                    *_self.asset.borrow_mut() = {
                            let resp = match _self.opts.asset_id {
                                AssetId::JigId(jig_id) => {
                                    endpoints::jig::GetDraft::api_no_auth(
                                        JigGetDraftPath(jig_id.clone()),
                                        None
                                    )
                                        .await
                                        .map(|jig| Asset::Jig(jig))
                                },
                                AssetId::ResourceId(resource_id) => {
                                    endpoints::resource::GetDraft::api_no_auth(
                                        ResourceGetDraftPath(resource_id.clone()),
                                        None
                                    )
                                        .await
                                        .map(|resource| Asset::Resource(resource))
                                },
                                AssetId::PlaylistId(playlist_id) => {
                                    endpoints::playlist::GetDraft::api_no_auth(
                                        PlaylistGetDraftPath(playlist_id.clone()),
                                        None
                                    )
                                        .await
                                        .map(|playlist| Asset::Playlist(playlist))
                                },
                                AssetId::CourseId(course_id) => {
                                    endpoints::course::GetDraft::api_no_auth(
                                        CourseGetDraftPath(course_id.clone()),
                                        None
                                    )
                                        .await
                                        .map(|course| Asset::Course(course))
                                }
                            };

                            match resp {
                                Ok(asset) => {
                                    Some(asset)
                                },
                                Err(_) => {
                                    panic!("error loading jig!")
                                },
                            }
                    };
                } else {
                    *_self.asset.borrow_mut() = Some(Asset::Jig(JigResponse {
                        id: JigId::from_u128(0),
                        admin_data: JigAdminData {
                            rating: None,
                            blocked: false,
                            curated: true,
                            premium: false,
                        },
                        creator_id: None,
                        author_id: None,
                        author_name: None,
                        published_at: None,
                        likes: 0,
                        plays: 0,
                        live_up_to_date: false,
                        is_liked: false,
                        jig_data: JigData {
                            created_at: shared::Utc::now(),
                            last_edited: None,
                            draft_or_live: DraftOrLive::Draft,
                            display_name: String::from("debug!"),
                            modules: Vec::new(),
                            age_ranges: Vec::new(),
                            affiliations: Vec::new(),
                            language: String::from(Language::default().code()),
                            categories: Vec::new(),
                            additional_resources: Vec::new(),
                            description: String::from("debug"),
                            theme: ThemeId::default(),
                            audio_background: None,
                            audio_effects: AudioEffects::default(),
                            default_player_settings: JigPlayerSettings::default(),
                            privacy_level: PrivacyLevel::default(),
                            locked: true,
                            other_keywords: String::from(""),
                            translated_keywords: String::from(""),
                            translated_description: HashMap::new(),
                        },
                    }));
                }

                let asset = _self.asset.borrow().clone().unwrap_ji();
                if let Asset::Jig(jig) = &asset {
                    AUDIO_MIXER.with(|mixer| mixer.set_from_jig(&jig.jig_data));
                };

                let (raw, init_source) = {
                    if let Some(force_raw) = _self.opts.force_raw.clone() {
                        (force_raw, InitSource::ForceRaw)
                    } else {
                        let resp = {
                            GetDraft::api_no_auth(
                                ModuleGetDraftPath(AssetType::from(&_self.opts.asset_id), _self.opts.module_id.clone()),
                                None
                            ).await
                        };

                        match resp {
                            Ok(resp) => {
                                let body = resp.module.body;
                                (body.try_into().unwrap_ji(), InitSource::Load)
                            },
                            Err(_) => {
                                panic!("error loading module!")
                            }
                        }
                    }
                };

                let history = Rc::new(HistoryState::new(
                    raw.clone(),
                    super::actions::save_history(
                        _self.opts.skip_save_for_debug,
                        _self.screenshot_loader.clone(),
                        _self.save_loader.clone(),
                        _self.opts.asset_id,
                        _self.opts.module_id,
                    ),
                    Self::reset_from_history(_self.clone(), init_from_raw.clone())
                ));

                *_self.history.borrow_mut() = Some(history.clone());

                let (asset_id, module_id) = (
                    _self.opts.asset_id,
                    _self.opts.module_id,
                );

                if raw.requires_choose_mode() {
                    Self::change_phase_choose(_self.clone(), init_from_raw);
                } else {

                    Self::change_phase_base(
                        _self.clone(),
                        init_from_raw.clone(),
                        BaseInitFromRawArgs::new(
                            asset_id,
                            module_id,
                            asset,
                            raw,
                            init_source,
                            history.clone()
                        )
                    ).await;
                }

                _self.raw_loaded.set_neq(true);
            }));
        })));

        //for editor we'll just init right away, no need to create the audio context
        //reverting is very easy if that becomes a problem (just create the audio context above and
        //uncomment this):
        (_self.on_init_ready.borrow().as_ref().unwrap_ji())();
        _self
    }

    pub fn step_signal(&self) -> impl Signal<Item = Option<Step>> {
        self.phase
            .signal_cloned()
            .switch(|phase| match phase.as_ref() {
                Phase::Choose(_) => OptionSignal::new(None),
                Phase::Init => OptionSignal::new(None),
                Phase::Base(app_base) => OptionSignal::new(Some(app_base.step.signal_cloned())),
            })
    }

    pub fn is_post_preview_signal(&self) -> impl Signal<Item = bool> {
        self.phase
            .signal_cloned()
            .switch(|phase| match phase.as_ref() {
                Phase::Choose(_) => EitherSignal::Left(always(false)),
                Phase::Init => EitherSignal::Left(always(false)),
                Phase::Base(app_base) => EitherSignal::Right(app_base.jig_is_post_preview.signal()),
            })
    }
}
