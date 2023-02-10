use dominator::Dom;
use futures_signals::signal::{Mutable, Signal};
use shared::domain::asset::{Asset, AssetId};
use std::collections::HashSet;
use std::{marker::PhantomData, rc::Rc};

use super::super::{actions::HistoryStateImpl, state::*};
use shared::domain::module::{
    body::{BodyExt, ModeExt, StepExt, ThemeId},
    ModuleId, ModuleKind,
};
use std::future::Future;
use utils::prelude::*;

use crate::tabs::MenuTabKind;

/// This is passed *to* the consumer in order to get a BaseInit
pub struct BaseInitFromRawArgs<RawData, Mode, Step>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    pub step: Mutable<Step>, //not intended to be changed lower down, just for passing back really
    pub steps_completed: Mutable<HashSet<Step>>,
    pub theme_id: Mutable<ThemeId>,
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    pub asset: Asset,
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
        asset_id: AssetId,
        module_id: ModuleId,
        asset: Asset,
        raw: RawData,
        source: InitSource,
        history: Rc<HistoryStateImpl<RawData>>,
    ) -> Self {
        let step = Mutable::new(raw.get_editor_state_step().unwrap_or_default());
        let steps_completed =
            Mutable::new(raw.get_editor_state_steps_completed().unwrap_or_default());

        let theme_id = Mutable::new(raw.get_theme().unwrap_or_default());

        Self {
            step,
            steps_completed,
            theme_id,
            asset_id,
            module_id,
            asset,
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
    // pub preview_step_reactor: AsyncLoader,
    pub step: Mutable<Step>,
    pub asset: Asset,
    pub base: Rc<Base>,
    pub main: Rc<Main>,
    pub sidebar: Rc<Sidebar>,
    pub header: Rc<Header>,
    pub footer: Rc<Footer>,
    pub overlay: Rc<Overlay>,
    pub steps_completed: Mutable<HashSet<Step>>,
    pub history: Rc<HistoryStateImpl<RawData>>,
    // Only for Jigs
    pub jig_is_post_preview: Mutable<bool>,
    pub mode: Option<Mode>,
    phantom: PhantomData<Mode>,
}

#[derive(Clone)]
pub enum PreviewMode {
    Preview,
    PostPreview,
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
        let theme_id = init_args.theme_id.clone();
        let steps_completed = init_args.steps_completed.clone();
        let asset = init_args.asset.clone();
        let mode = init_args.raw.mode();

        // get a BaseInit
        let init = init_from_raw(init_args).await;

        // apply debug overrides
        if let Some(force_step) = init.force_step {
            step.set_neq(force_step);
        }
        if let Some(force_theme) = init.force_theme {
            theme_id.set_neq(force_theme);
        }

        Self {
            step,
            asset,
            base: init.base,
            main: init.main,
            sidebar: init.sidebar,
            header: init.header,
            footer: init.footer,
            overlay: init.overlay,
            steps_completed,
            history: app.history.borrow().as_ref().unwrap_ji().clone(),
            jig_is_post_preview: Mutable::new(false),
            mode,
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
    pub force_theme: Option<ThemeId>,
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

/// Convenience alias for commonly used custom continue functions
pub type ContinueNextFn = Mutable<Option<Rc<dyn Fn() -> bool>>>;

pub trait BaseExt<Step: StepExt> {
    type CanContinueSignal: Signal<Item = bool>;
    /// Whether the step in the modules sidebar can be changed
    fn allowed_step_change(&self, from: Step, to: Step) -> bool;

    /// A signal used to determine whether the module navigation can proceed to the next tab/step
    fn can_continue_next(&self) -> Self::CanContinueSignal;

    /// Custom module-level navigation
    ///
    /// Returns `false` if the module implementing this didn't handle the navigation.
    fn continue_next(&self) -> bool;

    /// Current JIG's ID
    fn get_asset_id(&self) -> AssetId;

    /// Current module's ID
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
    type TabKindSignal: Signal<Item = Option<MenuTabKind>>;
    fn tab_kind(&self) -> Self::TabKindSignal;
}

pub trait HeaderExt: DomRenderable {}

pub trait FooterExt: DomRenderable {}

pub trait OverlayExt: DomRenderable {}
