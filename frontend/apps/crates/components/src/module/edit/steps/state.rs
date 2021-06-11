use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
};
use dominator::{clone, Dom};
use dominator_helpers::futures::AsyncLoader;
use std::rc::Rc;
use std::collections::HashSet;
use std::hash::Hash;
use super::super::state::{Phase, GenericState};
use shared::domain::jig::module::body::{ModeExt, BodyExt};
use utils::prelude::*;

pub struct Steps <Step, Base, Main, Sidebar, Header, Footer, Overlay> 
where
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
    pub step: Mutable<Step>,
    pub base: Rc<Base>,
    pub main: Rc<Main>,
    pub sidebar: Rc<Sidebar>,
    pub header: Rc<Header>,
    pub footer: Rc<Footer>,
    pub overlay: Rc<Overlay>,
}

pub trait StepExt : Copy + Default + PartialEq + Eq + Hash {
    fn next(&self) -> Option<Self>;
    fn as_number(&self) -> usize;
    fn label(&self) -> &'static str;
    fn get_list() -> Vec<Self>;
    fn get_preview() -> Self;
    fn is_preview(&self) -> bool {
        *self == Self::get_preview()
    }
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



impl <Step, Base, Main, Sidebar, Header, Footer, Overlay> Steps <Step, Base, Main, Sidebar, Header, Footer, Overlay> 
where
    Step: StepExt + 'static,
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
{
    pub fn new<Mode, RawData>(
        app: Rc<GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>>, 
        init: StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>
    ) -> Self 
    where
        Mode: ModeExt + 'static,
        RawData: BodyExt<Mode> + 'static, 
    {

        let preview_step_reactor = AsyncLoader::new();
        preview_step_reactor.load(init.step.signal().for_each(clone!(app => move |step| {
            if step.is_preview() {
                app.is_preview.set_neq(true);
            } else {
                app.is_preview.set_neq(false);
            }
            async move {}
        })));

        Self {
            step: init.step,
            base: init.base,
            main: init.main,
            sidebar: init.sidebar,
            header: init.header,
            footer: init.footer,
            overlay: init.overlay,
            preview_step_reactor,
            steps_completed: Mutable::new(HashSet::new())
        }
    }
}
