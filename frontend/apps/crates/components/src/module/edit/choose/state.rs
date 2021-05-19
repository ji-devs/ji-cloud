use dominator_helpers::futures::AsyncLoader;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
};
use std::rc::Rc;
use super::super::{
    steps::state::*,
    state::{Phase, GenericState},
    actions::*,
};
use shared::domain::jig::module::body::BodyExt;
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
}


impl <Mode> Choose <Mode> 
where
    Mode: ModeExt + 'static,
{
    pub fn new<Step, RawData, InitFromModeFn, Sections, Main, Sidebar, Header, Footer, Overlay>(
        app: Rc<GenericState<Mode, Step, RawData, Sections, Main, Sidebar, Header, Footer, Overlay>>, 
        init_from_mode: InitFromModeFn,
    ) -> Self 
    where
        Mode: ModeExt + 'static,
        Step: StepExt + 'static,
        RawData: BodyExt + 'static, 
        Sections: SectionsExt<Step> + 'static,
        Main: MainExt + 'static,
        Sidebar: SidebarExt + 'static,
        Header: HeaderExt + 'static,
        Footer: FooterExt + 'static,
        Overlay: OverlayExt + 'static,
        InitFromModeFn: Fn(Mode, Rc<HistoryStateImpl<RawData>>) -> StepsInit<Step, Sections, Main, Sidebar, Header, Footer, Overlay> + 'static,

    {
        Self {
            on_mode_change: Box::new(move |mode| {
                let steps_init = init_from_mode(mode, app.history.borrow().as_ref().unwrap_ji().clone());
                GenericState::change_phase_steps(app.clone(), steps_init);
            }),
        }
    }
}
