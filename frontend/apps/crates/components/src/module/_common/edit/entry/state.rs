#![feature(type_alias_impl_trait)]
use dominator::clone;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
};
use uuid::Uuid;

use std::cell::RefCell;
use std::convert::{TryFrom, TryInto};
use std::future::Future;
use std::rc::Rc;

use crate::module::_common::edit::history::state::HistoryState;
use dominator_helpers::{futures::AsyncLoader, signals::OptionSignal};

//use super::actions::{HistoryChangeFn, HistoryUndoRedoFn};
use super::{actions::*, base::state::*, choose::state::*};
use shared::domain::jig::{
    module::{
        body::{BodyExt, StepExt},
        ModuleId,
    },
    JigId,
};
use shared::{
    api::endpoints::{self, jig::module::*, ApiEndpoint},
    domain::jig::{
        module::{body::ModeExt, *},
        *,
    },
    error::EmptyError,
};
use utils::{languages::LANGUAGE_CODE_EN, prelude::*};

use crate::audio_mixer::AudioMixer;

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
    pub(super) jig: RefCell<Option<Jig>>,
    pub(super) opts: StateOpts<RawData>,
    pub(super) raw_loader: AsyncLoader,
    pub(super) screenshot_loader: Rc<AsyncLoader>,
    pub(super) save_loader: Rc<AsyncLoader>,
    pub(super) history: RefCell<Option<Rc<HistoryStateImpl<RawData>>>>,
    pub(super) raw_loaded: Mutable<bool>,
    pub(super) page_body_switcher: AsyncLoader,
    pub(super) reset_from_history_loader: AsyncLoader,
    pub(super) audio_mixer: AudioMixer,
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
    pub jig_id: JigId,
    pub module_id: ModuleId,
    //the step which is for previewing
    pub is_main_scrollable: bool,
    pub force_raw: Option<RawData>,
}

impl<RawData> StateOpts<RawData> {
    pub fn new(jig_id: JigId, module_id: ModuleId) -> Self {
        Self {
            skip_save_for_debug: false,
            skip_load_jig: false,
            jig_id,
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
            jig: RefCell::new(None),
            phase: Mutable::new(Rc::new(Phase::Init)),
            history: RefCell::new(None),
            raw_loaded: Mutable::new(false),
            raw_loader: AsyncLoader::new(),
            screenshot_loader: Rc::new(AsyncLoader::new()),
            save_loader: Rc::new(AsyncLoader::new()),
            page_body_switcher: AsyncLoader::new(),
            reset_from_history_loader: AsyncLoader::new(),
            audio_mixer: AudioMixer::new(None),
            on_init_ready: RefCell::new(None),
        });

        *_self.on_init_ready.borrow_mut() = Some(Box::new(clone!(_self => move || {
            _self.raw_loader.load(clone!(_self, init_from_raw => async move {
                if !_self.opts.skip_load_jig {
                    *_self.jig.borrow_mut() = {

                            let path = endpoints::jig::Get::PATH.replace("{id}",&_self.opts.jig_id.0.to_string());

                            match api_with_auth::<JigResponse, EmptyError, ()>(&path, endpoints::jig::Get::METHOD, None).await {
                                Ok(resp) => {
                                    Some(resp.jig)
                                },
                                Err(_) => {
                                    panic!("error loading jig!")
                                },
                            }
                    };
                } else {
                    *_self.jig.borrow_mut() = Some(Jig {
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
                    });
                }

                let jig = _self.jig.borrow().clone().unwrap_ji();
                _self.audio_mixer.set_from_jig(&jig);

                let (raw, init_source) = {
                    if let Some(force_raw) = _self.opts.force_raw.clone() {
                        (force_raw, InitSource::ForceRaw)
                    } else {
                        let path = Get::PATH
                            .replace("{id}",&_self.opts.jig_id.0.to_string())
                            .replace("{module_id}",&_self.opts.module_id.0.to_string());

                        match api_with_auth::<ModuleResponse, EmptyError, ()>(&path, Get::METHOD, None).await {
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
                        _self.opts.jig_id.clone(),
                        _self.opts.module_id.clone(),
                    ),
                    Self::reset_from_history(_self.clone(), init_from_raw.clone())
                ));

                *_self.history.borrow_mut() = Some(history.clone());

                let (jig_id, module_id) = (
                    _self.opts.jig_id.clone(),
                    _self.opts.module_id.clone(),
                );

                if raw.requires_choose_mode() {
                    Self::change_phase_choose(_self.clone(), init_from_raw);
                } else {

                    Self::change_phase_base(
                        _self.clone(),
                        init_from_raw.clone(),
                        BaseInitFromRawArgs::new(
                            _self.audio_mixer.clone(),
                            jig_id,
                            module_id,
                            jig,
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

    pub fn preview_mode_signal(&self) -> impl Signal<Item = Option<PreviewMode>> {
        self.phase
            .signal_cloned()
            .switch(|phase| match phase.as_ref() {
                Phase::Choose(_) => OptionSignal::new(None),
                Phase::Init => OptionSignal::new(None),
                Phase::Base(app_base) => OptionSignal::new(
                    Some(app_base.preview_mode.signal_cloned())
                )
            })
            .map(|x| x.flatten())
            
    }

    pub fn is_preview_signal(&self) -> impl Signal<Item = bool> {
        self.preview_mode_signal().map(|x| x.is_some())
    }
}
