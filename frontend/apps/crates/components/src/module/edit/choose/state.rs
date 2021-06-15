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
    base::state::*,
    state::{Phase, GenericState},
    actions::*,
};
use crate::audio_mixer::AudioMixer;
use shared::domain::jig::{JigId, Jig, module::{ModuleId, body::{ModeExt, BodyExt, StepExt}}};
use utils::prelude::*;

pub struct Choose <Mode>
where
    Mode: ModeExt + 'static,
{
    //getting rid of this Box is probably more headache than it's worth
    pub on_mode_change: Box<dyn Fn(Mode)>,
    pub loader: Rc<AsyncLoader>,
}



impl <Mode> Choose <Mode> 
where
    Mode: ModeExt + 'static,
{
    pub fn new<Step, RawData, BaseInitFromRawFn, BaseInitFromRawOutput, Base, Main, Sidebar, Header, Footer, Overlay>(
        app: Rc<GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>>, 
        init_from_raw: BaseInitFromRawFn,
    ) -> Self 
    where
        Mode: ModeExt + 'static,
        Step: StepExt + 'static,
        RawData: BodyExt<Mode, Step> + 'static, 
        Base: BaseExt<Step> + 'static,
        Main: MainExt + 'static,
        Sidebar: SidebarExt + 'static,
        Header: HeaderExt + 'static,
        Footer: FooterExt + 'static,
        Overlay: OverlayExt + 'static,
        BaseInitFromRawFn: Fn(BaseInitFromRawArgs<RawData, Mode, Step>) -> BaseInitFromRawOutput + Clone + 'static,
        BaseInitFromRawOutput: Future<Output = BaseInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>>,

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
                    
                    let raw = RawData::new_mode(mode);
                    let history = app.history.borrow().as_ref().unwrap_ji().clone();
                    history.push_modify(clone!(raw => |init| {
                        *init = raw;
                    }));


                    GenericState::change_phase_base(
                        app.clone(),
                        init_from_raw.clone(),
                        BaseInitFromRawArgs::new(
                            app.get_audio_mixer(), 
                            jig_id, 
                            module_id, 
                            jig, 
                            raw, 
                            InitSource::ChooseMode, 
                            history
                        )
                    ).await;

                }))
            }),
        }
    }
}
