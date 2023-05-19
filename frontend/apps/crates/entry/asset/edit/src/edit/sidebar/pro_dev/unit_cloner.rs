use shared::domain::pro_dev::unit::{
    CreateProDevUnitPath, ProDevUnit, ProDevUnitCreateRequest, ProDevUnitId,
};
use shared::{api::endpoints::pro_dev, domain::pro_dev::ProDevId};
use utils::prelude::ApiEndpointExt;

pub async fn clone_unit(
    orig_unit: &ProDevUnit,
    pro_dev_id: &ProDevId,
) -> anyhow::Result<ProDevUnit> {
    let id = create_unit(pro_dev_id, orig_unit.clone()).await?;
    Ok(ProDevUnit {
        id,
        display_name: orig_unit.display_name.clone(),
        description: orig_unit.description.clone(),
        value: orig_unit.value.clone(),
    })
}

async fn create_unit(
    pro_dev_id: &ProDevId,
    pro_dev_unit: ProDevUnit,
) -> anyhow::Result<ProDevUnitId> {
    let req = ProDevUnitCreateRequest {
        display_name: pro_dev_unit.display_name,
        description: pro_dev_unit.description,
        value: pro_dev_unit.value,
    };

    let res =
        pro_dev::unit::Create::api_with_auth(CreateProDevUnitPath(pro_dev_id.clone()), Some(req))
            .await?;
    Ok(res.id)
}
