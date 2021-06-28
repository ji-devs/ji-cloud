use std::rc::Rc;
use std::cell::RefCell;
use shared::{
    api::endpoints::{ApiEndpoint, self, jig::module::*},
    error::{EmptyError, MetadataNotFound},
    domain::jig::{*, module::{*, body::{Body, ThemeId, ThemeChoice, ModeExt, BodyExt, StepExt}}},
};
use dominator::{clone, Dom};
use futures_signals::{
    map_ref,
    signal::{self, Mutable, ReadOnlyMutable,  SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use std::convert::{TryFrom, TryInto};
use std::future::Future;
use dominator_helpers::{
    signals::OptionSignal,
    futures::AsyncLoader,
};
use crate::font_loader::{FontLoader, Font};
use utils::{settings::SETTINGS, prelude::*, iframe::*};
use std::marker::PhantomData;
use crate::audio_mixer::AudioMixer;
use uuid::Uuid;

pub struct GenericState <RawData, Mode, Step, Base> 
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Base: BaseExt + 'static,
    Step: StepExt + 'static
{
    pub(super) phase: Mutable<Rc<Phase<RawData, Base>>>,
    pub(super) jig: RefCell<Option<Jig>>,
    pub(super) opts: StateOpts<RawData>,
    pub(super) raw_loader: AsyncLoader,
    pub(super) page_body_switcher: AsyncLoader,
    pub(super) audio_mixer: RefCell<Option<AudioMixer>>,
    pub(super) on_init_ready: RefCell<Option<Box<dyn Fn()>>>,
    phantom: PhantomData<(Mode, Step)>,
}

pub trait DomRenderable {
    fn render(state: Rc<Self>) -> Dom;
}

pub trait BaseExt: DomRenderable {
}

pub type RawDirect = bool;

pub enum Phase <RawData, Base> 
{
    Init,
    WaitingIframeRaw(Rc<Box<dyn Fn(RawData)>>),
    Playing(Rc<Base>, RawDirect),
}


impl <RawData, Base> Phase <RawData, Base> 
{
    pub fn waiting_iframe_raw(&self) -> bool {
        match self {
            Self::WaitingIframeRaw(_) => true,
            _ => false
        }
    }
}

pub enum InitSource {
    ForceRaw,
    Load,
    IframeData
}
#[derive(Debug, Clone)]
pub struct StateOpts<RawData> {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub force_raw: Option<RawData>, 
    pub force_raw_even_in_iframe: bool,
    pub skip_load_jig: bool,
    pub load_fonts: bool,
}

impl <RawData> StateOpts<RawData> {
    pub fn new(jig_id: JigId, module_id: ModuleId) -> Self {
        Self {
            jig_id,
            module_id,
            force_raw: None,
            force_raw_even_in_iframe: false,
            skip_load_jig: false,
            load_fonts: true,
        }
    }
}

pub struct InitFromRawArgs<RawData, Mode, Step> 
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static
{
    pub audio_mixer: AudioMixer, 
    pub jig_id: JigId, 
    pub module_id: ModuleId, 
    pub jig: Jig, 
    pub raw: RawData,
    pub source: InitSource,
    pub theme_id: ThemeId,
    phantom: PhantomData<(Mode, Step)>
}

impl <RawData, Mode, Step> InitFromRawArgs <RawData, Mode, Step> 
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static
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
            Some(theme_choice) => {
                match theme_choice {
                    ThemeChoice::Jig => jig.theme.clone(),
                    ThemeChoice::Override(theme_id) => theme_id
                }
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
            phantom: PhantomData
        }
    }
}

impl <RawData, Mode, Step, Base> GenericState <RawData, Mode, Step, Base> 
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Base: BaseExt + 'static,
    Step: StepExt + 'static
{
    pub fn new<InitFromRawFn, InitFromRawOutput>(
        opts: StateOpts<RawData>, 
        init_from_raw: InitFromRawFn, 
    ) -> Rc<Self>
    where
        InitFromRawFn: Fn(InitFromRawArgs<RawData, Mode, Step>) -> InitFromRawOutput + Clone + 'static,
        InitFromRawOutput: Future<Output = Rc<Base>>,
        <RawData as TryFrom<ModuleBody>>::Error: std::fmt::Debug
    {
        

        let _self = Rc::new(Self {
            opts,
            jig: RefCell::new(None),
            phase: Mutable::new(Rc::new(Phase::Init)),
            raw_loader: AsyncLoader::new(),
            page_body_switcher: AsyncLoader::new(),
            audio_mixer: RefCell::new(None),
            on_init_ready: RefCell::new(None),
            phantom: PhantomData
        });

        *_self.on_init_ready.borrow_mut() = Some(Box::new(clone!(_self => move || {
            _self.raw_loader.load(clone!(_self, init_from_raw => async move {

                if _self.opts.load_fonts {
                    FontLoader::new().load_all().await;
                }

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
                            language: String::from("en"),
                            categories: Vec::new(),
                            publish_at: None,
                            additional_resources: Vec::new(),
                            description: String::from("debug"),
                            last_edited: None,
                            is_public: false,
                            direction: TextDirection::default(),
                            display_score: false,
                            theme: ThemeId::default(),
                            audio_background: None,
                            audio_effects: AudioEffects::default() 
                        })
                    } else {
                        let path = endpoints::jig::Get::PATH.replace("{id}",&_self.opts.jig_id.0.to_string());

                        match api_with_auth::<JigResponse, EmptyError, ()>(&path, endpoints::jig::Get::METHOD, None).await {
                            Ok(resp) => {
                                Some(resp.jig)
                            },
                            Err(_) => {
                                panic!("error loading jig!")
                            },
                        }
                    }
                };


                let audio_ctx = Some(web_sys::AudioContext::new().unwrap_ji());
                let jig = _self.jig.borrow().as_ref().unwrap_ji().clone();
                *_self.audio_mixer.borrow_mut() = Some(AudioMixer::new(audio_ctx, &jig));

                let wait_iframe_data = should_get_iframe_data();

                let raw:Option<RawData> = _self.opts.force_raw.clone()
                    .and_then(|raw| {
                        if !wait_iframe_data || _self.opts.force_raw_even_in_iframe {
                            Some(raw)
                        } else {
                            None
                        }
                    });

                let raw_source = match raw {
                    Some(raw) => Some((raw, InitSource::ForceRaw)),
                    None => {
                        if wait_iframe_data {
                            _self.phase.set(Rc::new(Phase::WaitingIframeRaw(
                                Rc::new(Box::new(clone!(init_from_raw, _self => move |raw| {
                                    _self.raw_loader.load(clone!(init_from_raw, _self => async move {

                                        let (jig_id, module_id, jig) = (
                                            _self.opts.jig_id.clone(),
                                            _self.opts.module_id.clone(),
                                            _self.jig.borrow().as_ref().unwrap_ji().clone()
                                        );
                                        let base = init_from_raw(InitFromRawArgs::new(_self.get_audio_mixer(), jig_id, module_id, jig, raw, InitSource::IframeData)).await;

                                        _self.phase.set(Rc::new(Phase::Playing(base, true)));
                                    }));
                                })))
                            )));

                            None
                        } else {
                            let path = Get::PATH
                                .replace("{id}",&_self.opts.jig_id.0.to_string())
                                .replace("{module_id}",&_self.opts.module_id.0.to_string());

                            match api_with_auth::<ModuleResponse, EmptyError, ()>(&path, Get::METHOD, None).await {
                                Ok(resp) => {
                                    let body = resp.module.body;
                                    Some((body.try_into().unwrap_ji(), InitSource::Load))
                                },
                                Err(_) => {
                                    panic!("error loading module!")
                                }
                            }
                        } 
                    }
                };

                if let Some((raw, init_source)) = raw_source {

                    let (jig_id, module_id, jig) = (
                        _self.opts.jig_id.clone(),
                        _self.opts.module_id.clone(),
                        _self.jig.borrow().as_ref().unwrap_ji().clone()
                    );
                    let base = init_from_raw(InitFromRawArgs::new(_self.get_audio_mixer(), jig_id, module_id, jig, raw, init_source)).await;

                    _self.phase.set(Rc::new(Phase::Playing(base, false)));
                }
            }));
        })));

        _self
    }

    /// this should always be fine.. the only reason to delay
    /// is so that the audio mixer can be created with the jig info
    /// and in some cases that happens after self is created
    pub fn get_audio_mixer(&self) -> AudioMixer {
        self.audio_mixer.borrow().as_ref().unwrap_ji().clone()
    }
}

