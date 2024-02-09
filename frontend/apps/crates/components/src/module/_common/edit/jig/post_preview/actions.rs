use super::state::*;
use shared::{
    api::endpoints::{self},
    domain::module::{
        body::{BodyExt, ModeExt, StepExt},
        ModuleCreatePath, ModuleCreateRequest, ModuleKind,
    },
};
use utils::{
    iframe::{IframeAction, IframeMessageExt, ModuleToJigEditorMessage},
    prelude::*,
};

impl PostPreview {
    pub fn next(&self) {
        let msg = IframeAction::new(ModuleToJigEditorMessage::Next);

        if msg.try_post_message_to_editor().is_err() {
            log::info!("Couldn't post message to top... redirect!");

            let route: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                self.jig_id,
                JigEditRoute::Landing,
            )))
            .into();
            dominator::routing::go_to_url(&route);
        }
    }

    pub fn publish(&self) {
        let msg = IframeAction::new(ModuleToAssetEditorMessage::Publish);

        if msg.try_post_message_to_editor().is_err() {
            log::info!("Couldn't post message to top... redirect!");

            let route: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                self.jig_id,
                JigEditRoute::Landing,
            )))
            .into();
            dominator::routing::go_to_url(&route);
        }
    }

    pub fn duplicate_module<RawData, Mode, Step>(&self, target_kind: ModuleKind, raw_data: RawData)
    where
        RawData: BodyExt<Mode, Step> + 'static,
        Mode: ModeExt + 'static,
        Step: StepExt + 'static,
    {
        let target_body = raw_data.convert_to_body(target_kind).unwrap_ji();

        let req = ModuleCreateRequest {
            body: target_body,
            parent_id: self.jig_id.into(),
        };

        let jig_id = self.jig_id;

        self.loader.load(async move {
            let res = endpoints::module::Create::api_with_auth(ModuleCreatePath(), Some(req)).await;

            match res {
                Ok(module) => {
                    let module_id = module.id;

                    let msg = IframeAction::new(ModuleToJigEditorMessage::AppendModule(module));

                    if msg.try_post_message_to_editor().is_err() {
                        log::info!("Couldn't post message to parent... redirect!");
                        let route: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                            jig_id,
                            JigEditRoute::Module(module_id),
                        )))
                        .into();
                        dominator::routing::go_to_url(&route);
                    }
                }
                Err(_) => {
                    log::error!("request to create module failed!");
                }
            }
        });
    }

    pub fn print_cards<RawData, Mode, Step>(&self, raw_data: &RawData) -> anyhow::Result<()>
    where
        RawData: BodyExt<Mode, Step> + 'static,
        Mode: ModeExt + 'static,
        Step: StepExt + 'static,
    {
        super::print::cards::print(raw_data)
    }

    pub fn print_design(&self) {
        super::print::screenshot::print(self.jig_id.into(), self.module_id);
    }
}
