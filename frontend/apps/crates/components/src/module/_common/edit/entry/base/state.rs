use dominator::{clone, Dom};
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable, Signal, SignalExt},
};
use std::collections::HashSet;
use std::{marker::PhantomData, rc::Rc};

use super::super::{actions::HistoryStateImpl, state::*};
use shared::domain::jig::{
    module::{
        body::{BodyExt, ModeExt, StepExt, ThemeChoice},
        ModuleId,
    },
    JigData, JigId, ModuleKind,
};
use std::future::Future;
use utils::prelude::*;

use crate::module::_common::edit::post_preview::state::PostPreview;

use wasm_bindgen_futures::spawn_local;

/// This is passed *to* the consumer in order to get a BaseInit
pub struct BaseInitFromRawArgs<RawData, Mode, Step>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    pub step: Mutable<Step>, //not intended to be changed lower down, just for passing back really
    pub steps_completed: Mutable<HashSet<Step>>,
    pub theme_choice: Mutable<ThemeChoice>,
    pub theme_id: ReadOnlyMutable<ThemeId>, //derived from jig and module theme
    pub jig_theme_id: Mutable<ThemeId>,
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: JigData,
    pub raw: RawData,
    pub source: InitSource,
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub module_kind: ModuleKind,
    phantom: PhantomData<Mode>,
}

impl<RawData, Mode, Step> BaseInitFromRawArgs<RawData, Mode, Step>
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
        history: Rc<HistoryStateImpl<RawData>>,
    ) -> Self {
        let step = Mutable::new(raw.get_editor_state_step().unwrap_or_default());
        let steps_completed =
            Mutable::new(raw.get_editor_state_steps_completed().unwrap_or_default());

        let theme_choice = Mutable::new(raw.get_theme().unwrap_or_default());

        let jig_theme_id = Mutable::new(jig.theme);

        let theme_id_sig = {
            map_ref! {
                let jig_theme_id = jig_theme_id.signal(),
                let theme = theme_choice.signal()
                    => {
                    match *theme {
                        ThemeChoice::Jig => *jig_theme_id,
                        ThemeChoice::Override(theme_id) => theme_id
                    }
                }
            }
        };

        let theme_id = Mutable::new(match theme_choice.get() {
            ThemeChoice::Jig => jig.theme,
            ThemeChoice::Override(id) => id,
        });

        //TODO: - hold onto this somewhere?
        spawn_local(clone!(theme_id => async move {
            let _ = theme_id_sig.for_each(clone!(theme_id => move |id| {
                theme_id.set_neq(id);
                async {}
            })).await;
        }));

        Self {
            step,
            steps_completed,
            theme_choice,
            theme_id: theme_id.read_only(),
            jig_id,
            module_id,
            jig,
            jig_theme_id,
            raw,
            source,
            history,
            module_kind: RawData::kind(),
            phantom: PhantomData,
        }
    }
}

/// this is held in this top level, created essentially from a BaseInit
/// By way of a BaseInit factory and BaseInitFromRawArgs
/// (it's done this way since args like step mutable need to be shared at both levels)

pub struct AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>
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
    pub jig: JigData,
    pub base: Rc<Base>,
    pub main: Rc<Main>,
    pub sidebar: Rc<Sidebar>,
    pub header: Rc<Header>,
    pub footer: Rc<Footer>,
    pub overlay: Rc<Overlay>,
    pub steps_completed: Mutable<HashSet<Step>>,
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub preview_mode: Mutable<Option<PreviewMode>>,
    phantom: PhantomData<Mode>,
}

#[derive(Clone)]
pub enum PreviewMode {
    Preview,
    PostPreview(Rc<PostPreview>),
}

impl<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>
    AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>
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
        BaseInitFromRawFn:
            Fn(BaseInitFromRawArgs<RawData, Mode, Step>) -> BaseInitFromRawOutput + Clone + 'static,
        BaseInitFromRawOutput:
            Future<Output = BaseInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
    {
        // extract the things from init args that need to be shared
        // even if just for applying the debug override
        let step = init_args.step.clone();
        let theme_choice = init_args.theme_choice.clone();
        let steps_completed = init_args.steps_completed.clone();
        let jig = init_args.jig.clone();

        // get a BaseInit
        let init = init_from_raw(init_args).await;

        // apply debug overrides
        if let Some(force_step) = init.force_step {
            step.set_neq(force_step);
        }
        if let Some(force_theme) = init.force_theme {
            theme_choice.set_neq(force_theme);
        }

        // setup a reactor on the step stuff, independent of the dom rendering
        let preview_step_reactor = AsyncLoader::new();

        let preview_mode = Mutable::new(None);
        preview_step_reactor.load(step.signal().for_each(clone!(preview_mode => move |step| {
            if step.is_preview() {
                preview_mode.set(Some(PreviewMode::Preview));
            } else if preview_mode.lock_ref().is_some() {
                preview_mode.set(None);
            }
            async move {}
        })));

        Self {
            step,
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
            preview_mode,
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
    // #![feature(type_alias_impl_trait)]
    // and the implementor will have
    // type FooSignal = impl Signal<Item = Foo>
    type NextStepAllowedSignal: Signal<Item = bool>;

    fn allowed_step_change(&self, from: Step, to: Step) -> bool;

    fn next_step_allowed_signal(&self) -> Self::NextStepAllowedSignal;

    fn get_jig_id(&self) -> JigId;

    fn get_module_id(&self) -> ModuleId;
}

pub trait MainExt: MainDomRenderable {}

pub trait MainDomRenderable: DomRenderable {
    // This needs to be separate since we can have scrollbars
    // and the background should not count towards that
    fn render_bg(_state: Rc<Self>) -> Option<Dom> {
        None
    }
}

pub trait SidebarExt: DomRenderable {
    type TabIndexSignal: Signal<Item = Option<usize>>;

    fn tab_index(&self) -> Self::TabIndexSignal;
}

pub trait HeaderExt: DomRenderable {}

pub trait FooterExt: DomRenderable {}

pub trait OverlayExt: DomRenderable {}
