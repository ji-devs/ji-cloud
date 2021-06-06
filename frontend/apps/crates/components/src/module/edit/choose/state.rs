use dominator_helpers::futures::AsyncLoader;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
};
use std::{marker::PhantomData, rc::Rc};
use std::future::Future;
use super::super::{
    state::*,
    steps::state::*,
    state::{Phase, GenericState},
    actions::*,
};
use shared::domain::jig::{JigId, Jig, module::{ModuleId, body::BodyExt}};
use utils::prelude::*;

pub trait ModeExt<RawMode> : Copy
where
    Self: Sized
{
    fn get_list() -> Vec<Self>;
    fn title() -> &'static str; 
    fn module() -> &'static str; 
    fn as_str_id(&self) -> &'static str;
    fn as_str_label(&self) -> &'static str;
    fn to_raw(&self) -> RawMode;
}

pub struct Choose <Mode, RawMode>
where
    Mode: ModeExt<RawMode> + 'static,
    RawMode: 'static
{
    //getting rid of this Box is probably more headache than it's worth
    pub on_mode_change: Box<dyn Fn(Mode)>,
    pub loader: Rc<AsyncLoader>,
    pub phantom: PhantomData<RawMode> 
}



impl <Mode, RawMode> Choose <Mode, RawMode> 
where
    Mode: ModeExt<RawMode> + 'static,
    RawMode: 'static
{
    pub fn new<Step, RawData, InitFromRawFn, InitFromRawOutput, Base, Main, Sidebar, Header, Footer, Overlay>(
        app: Rc<GenericState<Mode, Step, RawData, RawMode, Base, Main, Sidebar, Header, Footer, Overlay>>, 
        init_from_raw: InitFromRawFn,
    ) -> Self 
    where
        Mode: ModeExt<RawMode> + 'static,
        Step: StepExt + 'static,
        RawData: BodyExt<RawMode> + 'static, 
        Base: BaseExt<Step> + 'static,
        Main: MainExt + 'static,
        Sidebar: SidebarExt + 'static,
        Header: HeaderExt + 'static,
        Footer: FooterExt + 'static,
        Overlay: OverlayExt + 'static,
        InitFromRawFn: Fn(JigId, ModuleId, Option<Jig>, RawData, InitSource, Option<Rc<Steps<Step, Base, Main, Sidebar, Header, Footer, Overlay>>>, Rc<HistoryStateImpl<RawData>>) -> InitFromRawOutput + Clone + 'static,
        InitFromRawOutput: Future<Output = StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>>,

    {

        let loader = Rc::new(AsyncLoader::new());

        Self {
            loader: loader.clone(),
            on_mode_change: Box::new(move |mode| {
                loader.load(clone!(init_from_raw, app => async move {

                    let (jig_id, module_id, jig) = (
                        app.opts.jig_id.clone(),
                        app.opts.module_id.clone(),
                        app.jig.borrow().clone()
                    );
                    
                    let raw = RawData::new_mode(mode.to_raw());
                    let history = app.history.borrow().as_ref().unwrap_ji().clone();
                    history.push_modify(clone!(raw => |init| {
                        *init = raw;
                    }));

                    let steps_init = init_from_raw(jig_id, module_id, jig, raw, InitSource::ChooseMode, None,history).await;
                    GenericState::change_phase_steps(app.clone(), steps_init);

                }))
            }),
            phantom: PhantomData
        }
    }
}
