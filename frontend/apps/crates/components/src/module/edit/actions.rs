use crate::module::history::state::HistoryState;
use super::state::*;
use std::rc::Rc;
use shared::{
    api::endpoints::{ApiEndpoint, self, jig::module::*}, 
    domain::{
        image::ImageId,
        audio::AudioId, 
        jig::{*, module::{*, body::BodyExt}}
    }, 
    error::{EmptyError, MetadataNotFound},
    media::MediaLibrary
};
use super::steps::state::*;
use super::choose::state::*;
use utils::prelude::*;
use dominator_helpers::futures::AsyncLoader;
use std::future::Future;
use dominator::clone;

impl <Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay> GenericState <Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay> 
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
{
    pub fn change_phase_choose<InitFromModeFn, InitFromModeOutput>(_self: Rc<Self>, init_from_mode: InitFromModeFn) 
    where
        InitFromModeFn: Fn(JigId, ModuleId, Option<Jig>, Mode, Rc<HistoryStateImpl<RawData>>) -> InitFromModeOutput + Clone + 'static,
        InitFromModeOutput: Future<Output = StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
    {
        _self.phase.set(Rc::new(Phase::Choose(Rc::new(Choose::new(
            _self.clone(),
            init_from_mode,
        )))));
    }
    pub fn change_phase_steps(_self: Rc<Self>, steps_init: StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>) -> Rc<Steps<Step, Base, Main, Sidebar, Header, Footer, Overlay>> {
        let steps = Rc::new(Steps::new(
            _self.clone(),
            steps_init 
        ));

        _self.phase.set(Rc::new(Phase::Steps(steps.clone())));

        steps
    }

    pub fn reset_from_history<InitFromRawFn, InitFromRawOutput, InitFromModeFn, InitFromModeOutput>(
        _self: Rc<Self>,
        init_from_raw: InitFromRawFn,
        init_from_mode: InitFromModeFn,
    ) -> Box<dyn Fn(RawData)> 
    where
        InitFromRawFn: Fn(JigId, ModuleId, Option<Jig>, RawData, IsHistory, Option<Rc<Steps<Step, Base, Main, Sidebar, Header, Footer, Overlay>>>, Rc<HistoryStateImpl<RawData>>) -> InitFromRawOutput + Clone + 'static,
        InitFromRawOutput: Future<Output = Option<StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>>>,
        InitFromModeFn: Fn(JigId, ModuleId, Option<Jig>, Mode, Rc<HistoryStateImpl<RawData>>) -> InitFromModeOutput + Clone + 'static,
        InitFromModeOutput: Future<Output = StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
    {
        Box::new(move |raw:RawData| {
            let curr_steps = match &*_self.phase.get_cloned() {
                Phase::Steps(curr_steps) => Some(curr_steps.clone()),
                _ => None
            };

            //History shouldn't affect current or completed steps
            //though this should arguably be configurable on the init object as a simple flag
            //i.e. it's up to the app to decice whether or not to preserve it
            //but the mechanism to do that is here
            let preserve_steps = curr_steps.as_ref().map(|curr| {
                (curr.step.get_cloned(), curr.steps_completed.get_cloned())
            });

            _self.reset_from_history_loader.load(clone!(_self, init_from_raw, init_from_mode => async move {

                let (jig_id, module_id, jig) = (
                    _self.opts.jig_id.clone(),
                    _self.opts.module_id.clone(),
                    _self.jig.borrow().clone()
                );
                if let Some(steps) = init_from_raw(jig_id, module_id, jig, raw, true, curr_steps, _self.history.borrow().as_ref().unwrap_ji().clone()).await {
                    let steps = Self::change_phase_steps(_self.clone(), steps);
                    if let Some((step, steps_completed)) = preserve_steps {
                        steps.step.set_neq(step);
                        steps.steps_completed.set(steps_completed);
                    }
                } else {
                    Self::change_phase_choose(_self.clone(), init_from_mode.clone());
                }
            }));
        })
    }
}

pub type HistoryStateImpl<RawData> = HistoryState<RawData, HistorySaveFn<RawData>, Box<dyn Fn(RawData)>>;
pub type HistorySaveFn<RawData> = impl Fn(RawData);

pub fn save_history<RawData>(skip_for_debug: bool, save_loader: Rc<AsyncLoader>, jig_id: JigId, module_id: ModuleId) -> HistorySaveFn<RawData> 
where
    RawData: BodyExt + 'static 
{
    move |raw_data:RawData| {
        if !skip_for_debug {
            save(raw_data, save_loader.clone(), jig_id, module_id);
        }
    }
}

pub fn save<RawData>(raw_data: RawData, save_loader: Rc<AsyncLoader>, jig_id: JigId, module_id: ModuleId)
where
    RawData: BodyExt + 'static 
{
    save_loader.load(async move {
        let body = raw_data.as_body(); 
        log::info!("SAVING...");
        let path = Update::PATH
            .replace("{id}",&jig_id.0.to_string())
            .replace("{module_id}",&module_id.0.to_string());

        let req = Some(ModuleUpdateRequest {
            is_complete: Some(raw_data.is_complete()),
            index: None,
            body: Some(body), 
        });
        api_with_auth_empty::<EmptyError, _>(&path, Update::METHOD, req).await; //.expect_ji("error saving module!");
        log::info!("SAVED!");
    });
}
//doesn't compile, gotta box for now: https://github.com/rust-lang/rust/issues/65442
//pub type HistoryUndoRedoFn<RawData> = impl Fn(Option<RawData>);
//pub fn history_on_undo_redo<Main, Mode, RawData>(state:Rc<State<Main, Mode, RawData>>) -> HistoryUndoRedoFn<RawData> 



