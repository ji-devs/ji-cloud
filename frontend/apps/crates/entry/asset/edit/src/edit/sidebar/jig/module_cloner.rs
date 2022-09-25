use shared::{
    api::endpoints::module,
    domain::{
        asset::AssetType,
        jig::JigId,
        module::{
            LiteModule, Module, ModuleBody, ModuleCreatePath, ModuleCreateRequest,
            ModuleGetDraftPath, ModuleId,
        },
    },
};
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};

pub async fn clone_module(
    orig_module_id: &ModuleId,
    new_jig_id: &JigId,
) -> anyhow::Result<LiteModule> {
    let module = get_module(orig_module_id).await.unwrap_ji();

    let id = create_module(new_jig_id, module.body.clone()).await?;
    Ok(LiteModule {
        id,
        kind: module.body.kind(),
        is_complete: module.is_complete,
    })
}

async fn get_module(module_id: &ModuleId) -> anyhow::Result<Module> {
    let res = module::GetDraft::api_with_auth(
        ModuleGetDraftPath(AssetType::Jig, module_id.clone()),
        None,
    )
    .await?;
    Ok(res.module)
}

async fn create_module(jig_id: &JigId, module_body: ModuleBody) -> anyhow::Result<ModuleId> {
    let req = ModuleCreateRequest {
        body: module_body,
        parent_id: (*jig_id).into(),
    };
    let res = module::Create::api_with_auth(ModuleCreatePath(), Some(req)).await?;
    Ok(res.id)
}
