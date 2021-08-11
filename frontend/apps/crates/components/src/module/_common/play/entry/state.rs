use crate::audio_mixer::AudioMixer;
use dominator::{clone, Dom};
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::Instructions;
use shared::{
    api::endpoints::{self, jig::module::*, ApiEndpoint},
    domain::jig::{
        module::{
            body::{BodyExt, ModeExt, StepExt, ThemeChoice, ThemeId},
            *,
        },
        *,
    },
    error::EmptyError,
};
use std::cell::RefCell;
use std::convert::{TryFrom, TryInto};
use std::future::Future;
use std::marker::PhantomData;
use std::rc::Rc;
use utils::languages::LANGUAGE_CODE_EN;
use utils::{iframe::*, prelude::*};
use uuid::Uuid;

pub struct GenericState<RawData, Mode, Step, Base>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Base: BaseExt + 'static,
    Step: StepExt + 'static,
{
    pub(super) phase: Mutable<Rc<Phase<RawData, Base>>>,
    pub(super) jig: RefCell<Option<Jig>>,
    pub(super) opts: StateOpts<RawData>,
    pub(super) raw_loader: AsyncLoader,
    pub(super) page_body_switcher: AsyncLoader,
    pub(super) audio_mixer: AudioMixer,
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

        let _self = Rc::new(Self {
            opts,
            jig: RefCell::new(None),
            phase: Mutable::new(Rc::new(Phase::Loading(loading_kind))),
            raw_loader: AsyncLoader::new(),
            page_body_switcher: AsyncLoader::new(),
            audio_mixer: AudioMixer::new(None),
            phantom: PhantomData,
        });

        _self.raw_loader.load(clone!(_self, init_from_raw => async move {
            *_self.jig.borrow_mut() = {
                if _self.opts.skip_load_jig {
                    Some(Jig {
                        id: JigId(Uuid::from_u128(0)),
                        display_name: String::from("debug!"),
                        modules: Vec::new(),
                        age_ranges: Vec::new(),
                        affiliations: Vec::new(),
                        goals: Vec::new(),
                        creator_id: None,
                        author_id: None,
                        language: String::from(LANGUAGE_CODE_EN),
                        categories: Vec::new(),
                        publish_at: None,
                        additional_resources: Vec::new(),
                        description: String::from("debug"),
                        last_edited: None,
                        is_public: false,
                        theme: ThemeId::default(),
                        audio_background: None,
                        audio_effects: AudioEffects::default(),
                        default_player_settings: JigPlayerSettings::default(),
                    })
                } else {
                    let path = endpoints::jig::Get::PATH.replace("{id}",&_self.opts.jig_id.0.to_string());

                    match api_no_auth::<JigResponse, EmptyError, ()>(&path, endpoints::jig::Get::METHOD, None).await {
                        Ok(resp) => {
                            Some(resp.jig)
                        },
                        Err(_) => {
                            panic!("error loading jig!")
                        },
                    }
                }
            };


            let jig = _self.jig.borrow().as_ref().unwrap_ji().clone();

            _self.audio_mixer.set_from_jig(&jig);

            let raw_source_player = match _self.phase.get_cloned().loading_kind_unchecked() {
                LoadingKind::Direct(raw) => Some((raw.clone(), InitSource::ForceRaw, false)),
                LoadingKind::Iframe => {
                    _self.phase.set(Rc::new(Phase::WaitingIframeRaw(
                        Rc::new(Box::new(clone!(init_from_raw, _self => move |raw| {
                            _self.raw_loader.load(clone!(init_from_raw, _self => async move {

                                let (jig_id, module_id, jig) = (
                                    _self.opts.jig_id.clone(),
                                    _self.opts.module_id.clone(),
                                    _self.jig.borrow().as_ref().unwrap_ji().clone()
                                );
                                let base = init_from_raw(InitFromRawArgs::new(_self.audio_mixer.clone(), jig_id, module_id, jig, raw, InitSource::IframeData)).await;

                                _self.phase.set(Rc::new(Phase::Ready(Ready {
                                    base, 
                                    jig_player: false,
                                    play_started: Mutable::new(false)
                                })));
                            }));
                        })))
                    )));

                    None
                },
                LoadingKind::Remote => {
                    let path = Get::PATH
                        .replace("{id}",&_self.opts.jig_id.0.to_string())
                        .replace("{module_id}",&_self.opts.module_id.0.to_string());

                    match api_no_auth::<ModuleResponse, EmptyError, ()>(&path, Get::METHOD, None).await {
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
                    _self.opts.jig_id.clone(),
                    _self.opts.module_id.clone(),
                    _self.jig.borrow().as_ref().unwrap_ji().clone()
                );
                let base = init_from_raw(InitFromRawArgs::new(_self.audio_mixer.clone(), jig_id, module_id, jig, raw, init_source)).await;

                _self.phase.set(Rc::new(Phase::Ready(Ready {
                    base, 
                    jig_player,
                    play_started: Mutable::new(false)
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

pub enum Phase<RawData, Base> {
    Loading(LoadingKind<RawData>),
    WaitingIframeRaw(Rc<Box<dyn Fn(RawData)>>),
    Ready(Ready<Base>),
}

pub struct Ready<Base> {
    pub base: Rc<Base>,
    pub jig_player: bool,
    pub play_started: Mutable<bool>,
}

impl<RawData, Base> Phase<RawData, Base> {
    pub fn waiting_iframe_raw(&self) -> bool {
        match self {
            Self::Loading(kind) => match kind {
                LoadingKind::Iframe => true,
                _ => false,
            },
            _ => false,
        }
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
pub struct InitFromRawArgs<RawData, Mode, Step>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    pub audio_mixer: AudioMixer,
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: Jig,
    pub raw: RawData,
    pub source: InitSource,
    pub theme_id: ThemeId,
    phantom: PhantomData<(Mode, Step)>,
}

impl<RawData, Mode, Step> InitFromRawArgs<RawData, Mode, Step>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    pub fn new(
        audio_mixer: AudioMixer,
        jig_id: JigId,
        module_id: ModuleId,
        jig: Jig,
        raw: RawData,
        source: InitSource,
    ) -> Self {
        let theme_id = match raw.get_theme() {
            Some(theme_choice) => match theme_choice {
                ThemeChoice::Jig => jig.theme.clone(),
                ThemeChoice::Override(theme_id) => theme_id,
            },
            None => {
                log::warn!("this shouldn't happen! playing a module with no theme id...");
                ThemeId::default()
            }
        };

        Self {
            audio_mixer,
            jig_id,
            module_id,
            theme_id,
            jig,
            raw,
            source,
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
}
