use super::state::*;
use shared::{
    api::endpoints::{self, ApiEndpoint},
    domain::{
        jig::{
            module::{
                body::{BodyExt, ModeExt, StepExt},
                ModuleCreateRequest, ModuleId,
            },
            ModuleKind,
        },
        CreateResponse,
    },
    error::EmptyError,
};
use utils::prelude::*;

impl PostPreview {
    pub fn duplicate_module<RawData, Mode, Step>(&self, target_kind: ModuleKind, raw_data: RawData)
    where
        RawData: BodyExt<Mode, Step> + 'static,
        Mode: ModeExt + 'static,
        Step: StepExt + 'static,
    {
        let target_body = raw_data.convert_to_body(target_kind).unwrap_ji();

        let path = endpoints::jig::module::Create::PATH.replace("{id}", &self.jig_id.0.to_string());
        let req = ModuleCreateRequest { body: target_body };

        let jig_id = self.jig_id;

        self.loader.load(async move {
            let res = api_with_auth::<CreateResponse<ModuleId>, EmptyError, ModuleCreateRequest>(
                &path,
                endpoints::jig::module::Create::METHOD,
                Some(req),
            )
            .await;

            match res {
                Ok(res) => {
                    let route: String =
                        Route::Jig(JigRoute::Edit(jig_id, JigEditRoute::Module(res.id))).into();
                    dominator::routing::go_to_url(&route);
                }
                Err(_) => {
                    log::error!("request to create module failed!");
                }
            }
        });
    }
}
