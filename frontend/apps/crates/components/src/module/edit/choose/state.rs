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
    pub fn new<Step, RawData, InitFromRawFn, InitFromRawOutput, Base, Main, Sidebar, Header, Footer, Overlay>(
        app: Rc<GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>>, 
        init_from_raw: InitFromRawFn,
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
        InitFromRawFn: Fn(AudioMixer, ReadOnlyStepMutables<Step>, JigId, ModuleId, Option<Jig>, RawData, InitSource, Rc<HistoryStateImpl<RawData>>) -> InitFromRawOutput + Clone + 'static,
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
                    
                    let raw = RawData::new_mode(mode);
                    let history = app.history.borrow().as_ref().unwrap_ji().clone();
                    history.push_modify(clone!(raw => |init| {
                        *init = raw;
                    }));

                    let step_mutables = get_step_mutables(&raw);
                    let read_only_step_mutables = (step_mutables.0.read_only(), step_mutables.1.read_only());

                    let steps_init = init_from_raw(app.get_audio_mixer(), read_only_step_mutables, jig_id, module_id, jig, raw, InitSource::ChooseMode, history).await;
                    GenericState::change_phase_steps(app.clone(), steps_init, step_mutables);

                }))
            }),
        }
    }
}
