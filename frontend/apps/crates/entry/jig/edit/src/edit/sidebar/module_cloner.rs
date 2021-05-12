use futures_signals::signal::Mutable;
use shared::{api::endpoints::{ApiEndpoint, jig::module}, domain::{CreateResponse, jig::{JigId, module::{Module, ModuleBody, ModuleCreateRequest, ModuleId, ModuleResponse}}}, error::EmptyError};
use utils::{fetch::api_with_auth, unwrap::UnwrapJiExt};
use super::module::state::Module as BasicModule;

pub async fn clone_module(orig_jig_id: &JigId, orig_module_id: &ModuleId, new_jig_id: &JigId) -> Result<BasicModule, EmptyError> {
    let module = get_module(&orig_jig_id, &orig_module_id).await.unwrap_ji();

    let kind = match &module.body {
        None => None,
        Some(module_body) => Some(module_body.kind())
    };

    let id = create_module(new_jig_id, module.body.clone()).await?;
    Ok(BasicModule {
        id: id,
        kind: Mutable::new(kind),
    })
}

async fn get_module(jig_id: &JigId, module_id: &ModuleId) -> Result<Module, EmptyError> {
    let path = module::Get::PATH
        .replace("{id}", &jig_id.0.to_string())
        .replace("{module_id}", &module_id.0.to_string());
    let res =  api_with_auth::<ModuleResponse, EmptyError, ()>(&path, module::Get::METHOD, None).await?;
    Ok(res.module)
}

async fn create_module(jig_id: &JigId, module_body: Option<ModuleBody>) -> Result<ModuleId, EmptyError> {
    let path = module::Create::PATH
        .replace("{id}", &jig_id.0.to_string());
    let req = ModuleCreateRequest {
        body: module_body
    };
    let res = api_with_auth::<CreateResponse<ModuleId>, EmptyError, ModuleCreateRequest>(
        &path,
        module::Create::METHOD,
        Some(req)
    ).await?;
    Ok(res.id)
}
