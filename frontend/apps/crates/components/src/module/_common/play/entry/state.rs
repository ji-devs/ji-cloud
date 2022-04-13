use crate::audio::mixer::AUDIO_MIXER;
use dominator::{clone, Dom, DomHandle};
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::Instructions;
use shared::{
    api::endpoints::{self, jig::module::*, ApiEndpoint},
    domain::jig::{
        module::{
            body::{BodyExt, ModeExt, StepExt, ThemeId},
            *,
        },
        *,
    },
    error::EmptyError,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::future::Future;
use std::marker::PhantomData;
use std::rc::Rc;
use utils::languages::Language;
use utils::{iframe::*, prelude::*};

pub struct GenericState<RawData, Mode, Step, Base>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Base: BaseExt + 'static,
    Step: StepExt + 'static,
{
    pub(super) phase: Mutable<Rc<InitPhase<RawData, Base>>>,
    pub(super) jig: RefCell<Option<JigData>>,
    pub(super) opts: StateOpts<RawData>,
    pub(super) raw_loader: AsyncLoader,
    pub(super) page_body_switcher: AsyncLoader,
    pub(super) dom_body_handle: Mutable<Option<DomHandle>>,
    phantom: PhantomData<(Mode, Step)>,
}

impl<RawData, Mode, Step, Base> GenericState<RawData, Mode, Step, Base>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Base: BaseExt + 'static,
    Step: StepExt + 'static,
{
    pub fn new<InitFromRawFn, InitFromRawOutput>(
        opts: StateOpts<RawData>,
        init_from_raw: InitFromRawFn,
    ) -> Rc<Self>
    where
        InitFromRawFn:
            Fn(InitFromRawArgs<RawData, Mode, Step>) -> InitFromRawOutput + Clone + 'static,
        InitFromRawOutput: Future<Output = Rc<Base>>,
        <RawData as TryFrom<ModuleBody>>::Error: std::fmt::Debug,
    {
        let loading_kind = {
            let direct_data = opts.force_raw.as_ref().and_then(|data| {
                if opts.force_raw_even_in_iframe || !should_get_iframe_data() {
                    Some(data.clone())
                } else {
                    None
                }
            });

            match direct_data {
                Some(data) => LoadingKind::Direct(data),
                None => {
                    if should_get_iframe_data() {
                        LoadingKind::Iframe
                    } else {
                        LoadingKind::Remote
                    }
                }
            }
        };

        let is_draft: bool = utils::routes::is_param_bool("draft");

        let _self = Rc::new(Self {
            opts,
            jig: RefCell::new(None),
            phase: Mutable::new(Rc::new(InitPhase::Loading(loading_kind))),
            raw_loader: AsyncLoader::new(),
            page_body_switcher: AsyncLoader::new(),
            dom_body_handle: Mutable::new(None),
            phantom: PhantomData,
        });

        _self.raw_loader.load(clone!(_self, init_from_raw => async move {
            *_self.jig.borrow_mut() = {
                if _self.opts.skip_load_jig {
                    Some(JigData {
                        draft_or_live: DraftOrLive::Draft,
                        display_name: String::from("debug!"),
                        modules: Vec::new(),
                        age_ranges: Vec::new(),
                        affiliations: Vec::new(),
                        language: String::from(Language::default().code()),
                        categories: Vec::new(),
                        additional_resources: Vec::new(),
                        description: String::from("debug"),
                        last_edited: None,
                        theme: ThemeId::default(),
                        audio_background: None,
                        audio_effects: AudioEffects::default(),
                        default_player_settings: JigPlayerSettings::default(),
                        privacy_level: PrivacyLevel::default(),
                        locked: true,
                        other_keywords: String::from(""),
                        translated_keywords: String::from(""),
                        translated_description: HashMap::new(),
                    })
                } else {

                    let resp = {
                        if is_draft {
                            let path = endpoints::jig::GetDraft::PATH.replace("{id}",&_self.opts.jig_id.0.to_string());
                            api_no_auth::<JigResponse, EmptyError, ()>(&path, endpoints::jig::GetDraft::METHOD, None).await
                        } else {
                            let path = endpoints::jig::GetLive::PATH.replace("{id}",&_self.opts.jig_id.0.to_string());
                            api_no_auth::<JigResponse, EmptyError, ()>(&path, endpoints::jig::GetLive::METHOD, None).await
                        }
                    };

                    match resp {
                        Ok(resp) => {
                            Some(resp.jig_data)
                        },
                        Err(_) => {
                            panic!("error loading jig!")
                        },
                    }
                }
            };


            let jig = _self.jig.borrow().as_ref().unwrap_ji().clone();

            AUDIO_MIXER.with(|mixer| mixer.set_from_jig(&jig));

            let raw_source_player = match _self.phase.get_cloned().loading_kind_unchecked() {
                LoadingKind::Direct(raw) => Some((raw.clone(), InitSource::ForceRaw, false)),
                LoadingKind::Iframe => {
                    _self.phase.set(Rc::new(InitPhase::WaitingIframeRaw(
                        Rc::new(Box::new(clone!(init_from_raw, _self => move |raw| {
                            _self.raw_loader.load(clone!(init_from_raw, _self => async move {

                                let (jig_id, module_id, jig) = (
                                    _self.opts.jig_id,
                                    _self.opts.module_id,
                                    _self.jig.borrow().as_ref().unwrap_ji().clone()
                                );
                                let base = init_from_raw(InitFromRawArgs::new(jig_id, module_id, jig, raw, InitSource::IframeData)).await;

                                _self.phase.set(Rc::new(InitPhase::Ready(Ready {
                                    base,
                                    jig_player: false,
                                })));
                            }));
                        })))
                    )));

                    None
                },
                LoadingKind::Remote => {

                    let resp = {
                        if is_draft {
                            let path = GetDraft::PATH
                                .replace("{id}",&_self.opts.jig_id.0.to_string())
                                .replace("{module_id}",&_self.opts.module_id.0.to_string());

                            api_no_auth::<ModuleResponse, EmptyError, ()>(&path, GetDraft::METHOD, None).await
                        } else {
                            let path = GetLive::PATH
                                .replace("{id}",&_self.opts.jig_id.0.to_string())
                                .replace("{module_id}",&_self.opts.module_id.0.to_string());

                            api_no_auth::<ModuleResponse, EmptyError, ()>(&path, GetLive::METHOD, None).await
                        }
                    };

                    match resp {
                        Ok(resp) => {
                            let body = resp.module.body;
                            Some((body.try_into().unwrap_ji(), InitSource::Load, true))
                        },
                        Err(_) => {
                            panic!("error loading module!")
                        }
                    }
                }
            };

            if let Some((raw, init_source, jig_player)) = raw_source_player {

                let (jig_id, module_id, jig) = (
                    _self.opts.jig_id,
                    _self.opts.module_id,
                    _self.jig.borrow().as_ref().unwrap_ji().clone()
                );
                let base = init_from_raw(InitFromRawArgs::new(jig_id, module_id, jig, raw, init_source)).await;

                _self.phase.set(Rc::new(InitPhase::Ready(Ready {
                    base,
                    jig_player,
                })));
            }
        }));

        _self
    }
}

#[derive(Debug, Clone)]
pub struct StateOpts<RawData> {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub force_raw: Option<RawData>,
    pub force_raw_even_in_iframe: bool,
    pub skip_load_jig: bool,
    pub skip_play: bool,
}

impl<RawData> StateOpts<RawData> {
    pub fn new(jig_id: JigId, module_id: ModuleId) -> Self {
        Self {
            jig_id,
            module_id,
            force_raw: None,
            force_raw_even_in_iframe: false,
            skip_load_jig: false,
            skip_play: false,
        }
    }
}

pub type RawDirect = bool;

pub enum InitPhase<RawData, Base> {
    Loading(LoadingKind<RawData>),
    WaitingIframeRaw(Rc<Box<dyn Fn(RawData)>>),
    Ready(Ready<Base>),
}

pub struct Ready<Base> {
    pub base: Rc<Base>,
    pub jig_player: bool,
}

impl<RawData, Base> InitPhase<RawData, Base> {
    pub fn waiting_iframe_raw(&self) -> bool {
        matches!(self, Self::Loading(LoadingKind::Iframe))
    }

    pub fn loading_kind_unchecked(&self) -> &LoadingKind<RawData> {
        match self {
            Self::Loading(kind) => kind,
            _ => panic!("not loading kind!"),
        }
    }
}

pub enum LoadingKind<RawData> {
    Direct(RawData),
    Remote,
    Iframe,
}

pub enum InitSource {
    ForceRaw,
    Load,
    IframeData,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ModulePlayPhase {
    Preload,
    Init,
    Playing,
    Ending(Option<ModuleEnding>),
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ModuleEnding {
    Positive,
    Negative,
    Next,
}

pub struct InitFromRawArgs<RawData, Mode, Step>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: JigData,
    pub raw: RawData,
    pub source: InitSource,
    pub theme_id: ThemeId,
    pub play_phase: Mutable<ModulePlayPhase>,
    phantom: PhantomData<(Mode, Step)>,
}

impl<RawData, Mode, Step> InitFromRawArgs<RawData, Mode, Step>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    pub fn new(
        jig_id: JigId,
        module_id: ModuleId,
        jig: JigData,
        raw: RawData,
        source: InitSource,
    ) -> Self {
        let theme_id = match raw.get_theme() {
            Some(theme_id) => theme_id,
            None => {
                log::warn!("this shouldn't happen! playing a module with no theme id...");
                ThemeId::default()
            }
        };

        Self {
            jig_id,
            module_id,
            theme_id,
            jig,
            raw,
            source,
            play_phase: Mutable::new(if RawData::has_preload() {
                ModulePlayPhase::Preload
            } else {
                ModulePlayPhase::Init
            }),
            phantom: PhantomData,
        }
    }
}

pub trait DomRenderable {
    fn render(state: Rc<Self>) -> Dom;
}

pub trait BaseExt: DomRenderable {
    fn play(_state: Rc<Self>) {}
    fn get_instructions(&self) -> Option<Instructions> {
        None
    }
    fn get_timer_minutes(&self) -> Option<u32> {
        None
    }

    fn set_play_phase(&self, phase: ModulePlayPhase) {
        self.play_phase().set_neq(phase);
    }
    fn play_phase(&self) -> Mutable<ModulePlayPhase>;
}
