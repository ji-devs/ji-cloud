use shared::{
    api::endpoints::{module, ApiEndpoint},
    domain::{
        module::{LiteModule, Module, ModuleBody, ModuleCreateRequest, ModuleId, ModuleResponse},
        jig::JigId,
        CreateResponse,
    },
    error::EmptyError,
};
use utils::{fetch::api_with_auth, unwrap::UnwrapJiExt};

pub async fn clone_module(
    orig_jig_id: &JigId,
    orig_module_id: &ModuleId,
    new_jig_id: &JigId,
) -> Result<LiteModule, EmptyError> {
    let module = get_module(orig_jig_id, orig_module_id).await.unwrap_ji();

    let id = create_module(new_jig_id, module.body.clone()).await?;
    Ok(LiteModule {
        id,
        kind: module.body.kind(),
        is_complete: module.is_complete,
    })
}

async fn get_module(jig_id: &JigId, module_id: &ModuleId) -> Result<Module, EmptyError> {
    let path = module::GetDraft::PATH
        .replace("{id}", &jig_id.0.to_string())
        .replace("{module_id}", &module_id.0.to_string());

    let res =
        api_with_auth::<ModuleResponse, EmptyError, ()>(&path, module::GetDraft::METHOD, None)
            .await?;
    Ok(res.module)
}

async fn create_module(jig_id: &JigId, module_body: ModuleBody) -> Result<ModuleId, EmptyError> {
    let req = ModuleCreateRequest {
        body: module_body,
        parent_id: jig_id.clone().into()
    };
    let res = api_with_auth::<CreateResponse<ModuleId>, EmptyError, ModuleCreateRequest>(
        module::Create::PATH,
        module::Create::METHOD,
        Some(req),
    )
    .await?;
    Ok(res.id)
}
