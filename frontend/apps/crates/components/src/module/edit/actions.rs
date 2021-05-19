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


impl <Mode, Step, RawData, Sections, Main, Sidebar, Header, Footer, Overlay> GenericState <Mode, Step, RawData, Sections, Main, Sidebar, Header, Footer, Overlay> 
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
{
    pub fn change_phase_choose<InitFromModeFn>(_self: Rc<Self>, init_from_mode: InitFromModeFn) 
    where
        InitFromModeFn: Fn(Mode, Rc<HistoryStateImpl<RawData>>) -> StepsInit<Step, Sections, Main, Sidebar, Header, Footer, Overlay> + 'static,
    {
        _self.phase.set(Rc::new(Phase::Choose(Rc::new(Choose::new(
            _self.clone(),
            init_from_mode,
        )))));
    }
    pub fn change_phase_steps(_self: Rc<Self>, steps_init: StepsInit<Step, Sections, Main, Sidebar, Header, Footer, Overlay>) -> Rc<Steps<Step, Sections, Main, Sidebar, Header, Footer, Overlay>> {
        let steps = Rc::new(Steps::new(
            _self.clone(),
            steps_init 
        ));

        _self.phase.set(Rc::new(Phase::Steps(steps.clone())));

        steps
    }

    pub fn reset_from_history<InitFromRawFn, InitFromModeFn>(
        _self: Rc<Self>,
        init_from_raw: InitFromRawFn,
        init_from_mode: InitFromModeFn,
    ) -> Box<dyn Fn(Option<RawData>)> 
    where
        InitFromRawFn: Fn(RawData, IsHistory, Rc<HistoryStateImpl<RawData>>) -> Option<StepsInit<Step, Sections, Main, Sidebar, Header, Footer, Overlay>> + Clone + 'static,
        InitFromModeFn: Fn(Mode, Rc<HistoryStateImpl<RawData>>) -> StepsInit<Step, Sections, Main, Sidebar, Header, Footer, Overlay> + Clone + 'static,
    {
        Box::new(move |raw_data:Option<RawData>| {
            match raw_data {
                None => {
                    //
                    //allow going back to choose?
                    //Self::change_phase_choose(_self.clone(), on_mode_change);
                },
                Some(raw) => {
                    if let Some(steps) = init_from_raw(raw, true, _self.history.borrow().as_ref().unwrap_ji().clone()) {
                        Self::change_phase_steps(_self.clone(), steps);
                    } else {
                        Self::change_phase_choose(_self.clone(), init_from_mode.clone());
                    }
                }
            }
        })
    }
}

pub type HistoryStateImpl<RawData> = HistoryState<RawData, HistorySaveFn<RawData>, Box<dyn Fn(Option<RawData>)>>;
pub type HistorySaveFn<RawData> = impl Fn(Option<RawData>);

pub fn save_history<RawData>(skip_for_debug: bool, save_loader: Rc<AsyncLoader>, jig_id: JigId, module_id: ModuleId) -> HistorySaveFn<RawData> 
where
    RawData: BodyExt + 'static 
{
    move |raw_data:Option<RawData>| {
        if !skip_for_debug {
            if let Some(raw_data) = raw_data {
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
        }
    }
}

//doesn't compile, gotta box for now: https://github.com/rust-lang/rust/issues/65442
//pub type HistoryUndoRedoFn<RawData> = impl Fn(Option<RawData>);
//pub fn history_on_undo_redo<Main, Mode, RawData>(state:Rc<State<Main, Mode, RawData>>) -> HistoryUndoRedoFn<RawData> 



