use dominator::clone;
use dominator_helpers::futures::AsyncLoader;

use super::super::{base::state::*, state::GenericState, state::*};
use std::future::Future;
use std::{marker::PhantomData, rc::Rc};

use shared::domain::jig::module::body::{BodyExt, ModeExt, StepExt};
use utils::prelude::*;

pub struct Choose<RawData, Mode, Step>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    //getting rid of this Box is probably more headache than it's worth
    pub on_mode_change: Box<dyn Fn(Mode)>,
    pub loader: Rc<AsyncLoader>,
    phantom: PhantomData<(RawData, Step)>, //TODO: might not need this once we derive the mode list from RawData
}

impl<RawData, Mode, Step> Choose<RawData, Mode, Step>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    pub fn new<
        BaseInitFromRawFn,
        BaseInitFromRawOutput,
        Base,
        Main,
        Sidebar,
        Header,
        Footer,
        Overlay,
    >(
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
        BaseInitFromRawFn:
            Fn(BaseInitFromRawArgs<RawData, Mode, Step>) -> BaseInitFromRawOutput + Clone + 'static,
        BaseInitFromRawOutput:
            Future<Output = BaseInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
    {
        let loader = Rc::new(AsyncLoader::new());

        Self {
            phantom: PhantomData,
            loader: loader.clone(),
            on_mode_change: Box::new(move |mode| {
                loader.load(clone!(init_from_raw, app => async move {

                    let (jig_id, module_id, jig) = (
                        app.opts.jig_id,
                        app.opts.module_id,
                        app.jig.borrow().clone().unwrap_ji()
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
