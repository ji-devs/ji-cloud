use shared::{
    api::endpoints::{module, ApiEndpoint},
    domain::{
        asset::AssetType,
        jig::JigId,
        module::{LiteModule, Module, ModuleBody, ModuleCreateRequest, ModuleId, ModuleResponse},
        CreateResponse,
    },
    error::EmptyError,
};
use utils::{fetch::api_with_auth, unwrap::UnwrapJiExt};

pub async fn clone_module(
    orig_module_id: &ModuleId,
    new_jig_id: &JigId,
) -> Result<LiteModule, EmptyError> {
    let module = get_module(orig_module_id).await.unwrap_ji();

    let id = create_module(new_jig_id, module.body.clone()).await?;
    Ok(LiteModule {
        id,
        kind: module.body.kind(),
        is_complete: module.is_complete,
    })
}

async fn get_module(module_id: &ModuleId) -> Result<Module, EmptyError> {
    let path = module::GetDraft::PATH
        .replace("{asset_type}", AssetType::Jig.as_str())
        .replace("{module_id}", &module_id.0.to_string());

    let res =
        api_with_auth::<ModuleResponse, EmptyError, ()>(&path, module::GetDraft::METHOD, None)
            .await?;
    Ok(res.module)
}

async fn create_module(jig_id: &JigId, module_body: ModuleBody) -> Result<ModuleId, EmptyError> {
    let req = ModuleCreateRequest {
        body: module_body,
        parent_id: (*jig_id).into(),
    };
    let res = api_with_auth::<CreateResponse<ModuleId>, EmptyError, ModuleCreateRequest>(
        module::Create::PATH,
        module::Create::METHOD,
        Some(req),
    )
    .await?;
    Ok(res.id)
}
