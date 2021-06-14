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
    state::{Phase, GenericState},
    actions::HistoryStateImpl
};
use shared::domain::jig::module::body::{BodyExt, ModeExt, StepExt};
use utils::prelude::*;

pub struct Steps <RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay> 
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

pub type StepMutables<Step> = (Mutable<Step>, Mutable<HashSet<Step>>);
pub type ReadOnlyStepMutables<Step> = (ReadOnlyMutable<Step>, ReadOnlyMutable<HashSet<Step>>);

pub fn get_step_mutables<RawData, Mode, Step>(raw: &RawData) -> StepMutables<Step> 
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    (
        Mutable::new(raw.get_editor_state_step().unwrap_or_default()),
        Mutable::new(raw.get_editor_state_steps_completed().unwrap_or_default()),
    )
}

impl <RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay> Steps <RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay> 
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
    pub fn new(
        app: Rc<GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>>, 
        init: StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>,
        step_mutables: StepMutables<Step>,
    ) -> Self 
    {

        let (step, steps_completed) = step_mutables;

        if let Some(init_step) = init.force_step {
            step.set_neq(init_step);
        }

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

pub struct StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay> 
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
    // using this in practice will likely require
    // #![feature(min_type_alias_impl_trait)]
    // and the implementor will have
    // type NextStepAllowedSignal = impl Signal<Item = bool>
    type NextStepAllowedSignal: Signal<Item = bool>;

    fn allowed_step_change(&self, from:Step, to: Step) -> bool;

    fn next_step_allowed_signal(&self) -> Self::NextStepAllowedSignal;

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

