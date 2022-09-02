use super::state::*;
use crate::module::_common::edit::history::state::HistoryState;
use shared::{
    api::endpoints::module::*,
    domain::{
        asset::{AssetId, DraftOrLive},
        module::{
            body::{BodyExt, ModeExt, StepExt},
            *,
        },
    },
};
use std::rc::Rc;

use super::base::state::*;
use super::choose::state::*;
use dominator::clone;
use dominator_helpers::futures::AsyncLoader;
use std::future::Future;
use utils::{prelude::*, screenshot::call_screenshot_service};

impl<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>
    GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>
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
{
    pub fn change_phase_choose<BaseInitFromRawFn, BaseInitFromRawOutput>(
        _self: Rc<Self>,
        init_from_raw: BaseInitFromRawFn,
    ) where
        BaseInitFromRawFn:
            Fn(BaseInitFromRawArgs<RawData, Mode, Step>) -> BaseInitFromRawOutput + Clone + 'static,
        BaseInitFromRawOutput:
            Future<Output = BaseInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
    {
        _self.phase.set(Rc::new(Phase::Choose(Rc::new(Choose::new(
            _self.clone(),
            init_from_raw,
        )))));
    }
    pub async fn change_phase_base<BaseInitFromRawFn, BaseInitFromRawOutput>(
        _self: Rc<Self>,
        init_from_raw: BaseInitFromRawFn,
        init_args: BaseInitFromRawArgs<RawData, Mode, Step>,
    ) -> Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>
    where
        BaseInitFromRawFn:
            Fn(BaseInitFromRawArgs<RawData, Mode, Step>) -> BaseInitFromRawOutput + Clone + 'static,
        BaseInitFromRawOutput:
            Future<Output = BaseInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
    {
        let app_base = Rc::new(AppBase::new(_self.clone(), init_from_raw, init_args).await);

        _self.phase.set(Rc::new(Phase::Base(app_base.clone())));

        app_base
    }

    pub fn reset_from_history<BaseInitFromRawFn, BaseInitFromRawOutput>(
        _self: Rc<Self>,
        init_from_raw: BaseInitFromRawFn,
    ) -> Box<dyn Fn(RawData)>
    where
        BaseInitFromRawFn:
            Fn(BaseInitFromRawArgs<RawData, Mode, Step>) -> BaseInitFromRawOutput + Clone + 'static,
        BaseInitFromRawOutput:
            Future<Output = BaseInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
    {
        Box::new(move |raw: RawData| {
            _self
                .reset_from_history_loader
                .load(clone!(_self, init_from_raw => async move {

                    let (asset_id, module_id, asset) = (
                        _self.opts.asset_id,
                        _self.opts.module_id,
                        _self.asset.borrow().clone().unwrap_ji()
                    );

                    if raw.requires_choose_mode() {
                        Self::change_phase_choose(_self.clone(), init_from_raw.clone());
                    } else {
                        Self::change_phase_base(
                            _self.clone(),
                            init_from_raw.clone(),
                            BaseInitFromRawArgs::new(
                                asset_id,
                                module_id,
                                asset,
                                raw,
                                InitSource::History,
                                _self.history.borrow().as_ref().unwrap_ji().clone()
                            )
                        ).await;

                    }
                }));
        })
    }
}

pub type HistoryStateImpl<RawData> =
    HistoryState<RawData, Box<dyn Fn(RawData)>, Box<dyn Fn(RawData)>>;
//pub type HistorySaveFn<RawData> = impl Fn(RawData);

pub fn save_history<RawData, Mode, Step>(
    skip_for_debug: bool,
    screenshot_loader: Rc<AsyncLoader>,
    save_loader: Rc<AsyncLoader>,
    asset_id: AssetId,
    module_id: ModuleId,
) -> Box<dyn Fn(RawData)>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    Box::new(move |raw_data: RawData| {
        if !skip_for_debug {
            save(
                raw_data,
                screenshot_loader.clone(),
                save_loader.clone(),
                asset_id,
                module_id,
            );
        }
    })
}

pub fn save<RawData, Mode, Step>(
    raw_data: RawData,
    screenshot_loader: Rc<AsyncLoader>,
    save_loader: Rc<AsyncLoader>,
    asset_id: AssetId,
    module_id: ModuleId,
) where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    save_loader.load(async move {
        let body = raw_data.as_body();

        let is_complete = raw_data.is_complete();

        let req = Some(ModuleUpdateRequest {
            // id: StableOrUniqueId::Unique(module_id),
            is_complete: Some(is_complete),
            index: None,
            body: Some(body),
            parent_id: asset_id,
        });
        let _ = Update::api_with_auth_empty(ModuleUploadPath(module_id.clone()), req).await;

        // Update the sidebar with this modules completion status
        let _ = IframeAction::new(ModuleToJigEditorMessage::Complete(module_id, is_complete))
            .try_post_message_to_editor();

        if is_complete {
            // Only generate a screenshot if the module has the minimum required content.
            screenshot_loader.load(async move {
                call_screenshot_service(asset_id, module_id, RawData::kind(), DraftOrLive::Draft)
                    .await;
            });
        }
    });
}

//doesn't compile, gotta box for now: https://github.com/rust-lang/rust/issues/65442
//pub type HistoryUndoRedoFn<RawData> = impl Fn(Option<RawData>);
//pub fn history_on_undo_redo<Main, Mode, RawData>(state:Rc<State<Main, Mode, RawData>>) -> HistoryUndoRedoFn<RawData>
