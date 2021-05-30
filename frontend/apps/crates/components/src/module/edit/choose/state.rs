use dominator_helpers::futures::AsyncLoader;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
};
use std::rc::Rc;
use std::future::Future;
use super::super::{
    steps::state::*,
    state::{Phase, GenericState},
    actions::*,
};
use shared::domain::jig::{JigId, module::{ModuleId, body::BodyExt}};
use utils::prelude::*;

pub trait ModeExt : Copy
where
    Self: Sized
{
    fn get_list() -> Vec<Self>;
    fn title() -> &'static str; 
    fn module() -> &'static str; 
    fn as_str_id(&self) -> &'static str;
    fn as_str_label(&self) -> &'static str;
}

pub struct Choose <Mode>
where
    Mode: ModeExt + 'static
{
    //getting rid of this Box is probably more headache than it's worth
    pub on_mode_change: Box<dyn Fn(Mode)>,
    pub loader: Rc<AsyncLoader>,
}


impl <Mode> Choose <Mode> 
where
    Mode: ModeExt + 'static,
{
    pub fn new<Step, RawData, InitFromModeFn, InitFromModeOutput, Base, Main, Sidebar, Header, Footer, Overlay>(
        app: Rc<GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>>, 
        init_from_mode: InitFromModeFn,
    ) -> Self 
    where
        Mode: ModeExt + 'static,
        Step: StepExt + 'static,
        RawData: BodyExt + 'static, 
        Base: BaseExt<Step> + 'static,
        Main: MainExt + 'static,
        Sidebar: SidebarExt + 'static,
        Header: HeaderExt + 'static,
        Footer: FooterExt + 'static,
        Overlay: OverlayExt + 'static,
        InitFromModeFn: Fn(JigId, ModuleId, Mode, Rc<HistoryStateImpl<RawData>>) -> InitFromModeOutput + Clone + 'static,
        InitFromModeOutput: Future<Output = StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>>,

    {

        let loader = Rc::new(AsyncLoader::new());

        Self {
            loader: loader.clone(),
            on_mode_change: Box::new(move |mode| {
                loader.load(clone!(init_from_mode, app => async move {
                    let steps_init = init_from_mode(app.opts.jig_id.clone(), app.opts.module_id.clone(), mode, app.history.borrow().as_ref().unwrap_ji().clone()).await;
                    GenericState::change_phase_steps(app.clone(), steps_init);
                }))
            }),
        }
    }
}
