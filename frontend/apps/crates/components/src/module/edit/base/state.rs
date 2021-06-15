use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal, ReadOnlyMutable},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
};
use dominator::{clone, Dom};
use dominator_helpers::futures::AsyncLoader;
use std::{marker::PhantomData, rc::Rc};
use std::collections::HashSet;
use std::hash::Hash;
use super::super::{
    state::*,
    actions::HistoryStateImpl
};
use shared::domain::jig::{JigId, Jig, TextDirection, module::{ModuleId, body::{ThemeChoice, BodyExt, ModeExt, StepExt}}};
use utils::prelude::*;
use std::future::Future;
use uuid::Uuid;
use crate::audio_mixer::AudioMixer;

/// This is passed *to* the consumer in order to get a BaseInit
pub struct BaseInitFromRawArgs<RawData, Mode, Step>
where
    RawData: BodyExt<Mode, Step> + 'static, 
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    pub audio_mixer: AudioMixer, 
    pub step: Mutable<Step>,
    pub steps_completed: Mutable<HashSet<Step>>, 
    pub theme: Mutable<ThemeChoice>,
    pub jig_id: JigId, 
    pub module_id: ModuleId, 
    pub jig: Jig,
    pub raw: RawData, 
    pub source: InitSource,  
    pub history: Rc<HistoryStateImpl<RawData>>,
    phantom: PhantomData<Mode>
}

impl <RawData, Mode, Step> BaseInitFromRawArgs<RawData, Mode, Step> 
where
    RawData: BodyExt<Mode, Step> + 'static, 
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    pub fn new(
        audio_mixer: AudioMixer,
        jig_id: JigId,
        module_id: ModuleId,
        jig: Option<Jig>,
        raw: RawData,
        source: InitSource,
        history: Rc<HistoryStateImpl<RawData>>
    ) -> Self {
        let step = Mutable::new(raw.get_editor_state_step().unwrap_or_default());
        let steps_completed = Mutable::new(raw.get_editor_state_steps_completed().unwrap_or_default());

        let theme = Mutable::new(raw.get_theme().unwrap_or_default());

        let jig = match jig {
            Some(jig) => jig,
            None => {
                if source == InitSource::Load {
                    panic!("no jig but was supposed to load!");
                }
                 
                Jig {
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
                    theme: ThemeId::default()
                }
            }
        };

        Self {
            audio_mixer,
            step,
            steps_completed,
            theme,
            jig_id,
            module_id,
            jig,
            raw,
            source,
            history,
            phantom: PhantomData
        }
    }
}

/// this is held in this top level, created essentially from a BaseInit
/// By way of a BaseInit factory and BaseInitFromRawArgs
/// (it's done this way since args like step mutable need to be shared at both levels) 
pub struct AppBase <RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay> 
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
    pub preview_step_reactor: AsyncLoader,
    pub step: Mutable<Step>,
    pub theme: ReadOnlyMutable<ThemeChoice>,
    pub jig: Jig,
    pub base: Rc<Base>,
    pub main: Rc<Main>,
    pub sidebar: Rc<Sidebar>,
    pub header: Rc<Header>,
    pub footer: Rc<Footer>,
    pub overlay: Rc<Overlay>,
    pub steps_completed: Mutable<HashSet<Step>>,
    pub history: Rc<HistoryStateImpl<RawData>>,
    phantom: PhantomData<Mode>
}

impl <RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay> AppBase <RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay> 
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
    pub async fn new<BaseInitFromRawFn, BaseInitFromRawOutput>(
        app: Rc<GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>>, 
        init_from_raw: BaseInitFromRawFn,
        init_args: BaseInitFromRawArgs<RawData, Mode, Step>,
    ) -> Self 
    where
        BaseInitFromRawFn: Fn(BaseInitFromRawArgs<RawData, Mode, Step>) -> BaseInitFromRawOutput + Clone + 'static,
        BaseInitFromRawOutput: Future<Output = BaseInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
    {

        // extract the things from init args that need to be shared
        let step = init_args.step.clone();
        let steps_completed = init_args.steps_completed.clone();
        let theme = init_args.theme.clone();
        let jig = init_args.jig.clone();

        // get a BaseInit
        let init = init_from_raw(init_args).await;

        // apply debug overrides
        if let Some(force_step) = init.force_step {
            step.set_neq(force_step);
        }
        if let Some(force_theme) = init.force_theme {
            theme.set_neq(force_theme);
        }

        // setup a reactor on the step stuff, independent of the dom rendering
        let preview_step_reactor = AsyncLoader::new();

        preview_step_reactor.load(step.signal().for_each(clone!(app => move |step| {
            if step.is_preview() {
                app.is_preview.set_neq(true);
            } else {
                app.is_preview.set_neq(false);
            }
            async move {}
        })));

        Self {
            step, 
            theme: theme.read_only(), // we don't change the theme at this level
            jig,
            base: init.base,
            main: init.main,
            sidebar: init.sidebar,
            header: init.header,
            footer: init.footer,
            overlay: init.overlay,
            preview_step_reactor,
            steps_completed,
            history: app.history.borrow().as_ref().unwrap_ji().clone(),
            phantom: PhantomData,
        }
    }
}

pub struct BaseInit<Step, Base, Main, Sidebar, Header, Footer, Overlay> 
where
    Step: StepExt + 'static,
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
{
    pub force_step: Option<Step>,
    pub force_theme: Option<ThemeChoice>,
    pub base: Rc<Base>,
    pub main: Rc<Main>,
    pub sidebar: Rc<Sidebar>,
    pub header: Rc<Header>,
    pub footer: Rc<Footer>,
    pub overlay: Rc<Overlay>,
}

pub trait DomRenderable {
    fn render(state: Rc<Self>) -> Dom;
}

pub trait BaseExt<Step: StepExt> {
    // using these in practice will require
    // #![feature(min_type_alias_impl_trait)]
    // and the implementor will have
    // type FooSignal = impl Signal<Item = Foo>

    type ThemeIdSignal: Signal<Item = ThemeId>;
    type ThemeIdStrSignal: Signal<Item = &'static str>;
    type NextStepAllowedSignal: Signal<Item = bool>;

    fn allowed_step_change(&self, from:Step, to: Step) -> bool;

    fn next_step_allowed_signal(&self) -> Self::NextStepAllowedSignal;

    fn get_theme_id(&self) -> ThemeId;
    fn theme_id_signal(&self) -> Self::ThemeIdSignal; 
    fn theme_id_str_signal(&self) -> Self::ThemeIdStrSignal; 
    /* Todo - make implementing this a macro for every module */
    /*
    fn get_theme_id(&self) -> ThemeId {
        match self.theme.get_cloned() {
            ThemeChoice::Jig => self.jig.theme.clone(),
            ThemeChoice::Override(theme_id) => theme_id
        }
    }
    fn theme_id_signal(&self) -> impl Signal<Item = ThemeId> {
        let jig_theme_id = self.jig.theme.clone();

        self.theme.signal_cloned()
            .map(clone!(jig_theme_id => move |theme| {
                match theme { 
                    ThemeChoice::Jig => jig_theme_id,
                    ThemeChoice::Override(theme_id) => theme_id
                }
            }))
    }

    fn theme_id_str_signal(&self) -> impl Signal<Item = &'static str> {
        self.theme_id_signal().map(|id| id.as_str_id())
    }
    */
}

pub trait MainExt: DomRenderable {
}

pub trait SidebarExt: DomRenderable {
}

pub trait HeaderExt: DomRenderable {
}

pub trait FooterExt: DomRenderable {
}

pub trait OverlayExt: DomRenderable {
}

