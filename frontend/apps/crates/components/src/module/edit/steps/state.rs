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
use super::super::{
    choose::state::ModeExt,
    state::{Phase, GenericState}
};
use shared::domain::jig::module::body::BodyExt;

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
    pub main: Main,
    pub sidebar: Sidebar,
    pub header: Header,
    pub footer: Footer,
    pub overlay: Overlay,
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
    fn allowed_step_change(&self, from:Step, to: Step) -> bool;
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
        RawData: BodyExt + 'static, 
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
            main: Rc::new(init.main),
            sidebar: Rc::new(init.sidebar),
            header: Rc::new(init.header),
            footer: Rc::new(init.footer),
            overlay: Rc::new(init.overlay),
            preview_step_reactor,
            steps_completed: Mutable::new(HashSet::new())
        }
    }
}
