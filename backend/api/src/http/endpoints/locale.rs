use actix_web::web::{Data, Json};
use paperclip::actix::{api_v2_operation, web::ServiceConfig};
use shared::{
    api::{endpoints::locale, ApiEndpoint},
    domain::locale::{Bundle, ItemKind, ListBundleResponse, ListItemKindResponse},
};
use sqlx::PgPool;

use crate::error;

#[api_v2_operation]
async fn list_bundles(db: Data<PgPool>) -> Result<Json<ListBundleResponse>, error::Server> {
    let bundles = sqlx::query_as!(
        Bundle,
        "select id, display_name as name from locale_bundle order by created_at"
    )
    .fetch_all(db.as_ref())
    .await?;

    Ok(Json(ListBundleResponse { bundles }))
}

#[api_v2_operation]
async fn list_item_kinds(db: Data<PgPool>) -> Result<Json<ListItemKindResponse>, error::Server> {
    let item_kinds = sqlx::query_as!(
        ItemKind,
        "select id, display_name as name from locale_item_kind order by created_at"
    )
    .fetch_all(db.as_ref())
    .await?;

    Ok(Json(ListItemKindResponse { item_kinds }))
}

mod entry {
    use actix_web::web::{Data, Json, Path};
    use paperclip::actix::{api_v2_operation, CreatedJson};
    use shared::domain::locale::{
        CreateEntryRequest, CreateEntryResponse, Entry, EntryStatus, GetEntryResponse,
    };
    use sqlx::PgPool;

    use crate::{
        error,
        extractor::{ScopeManageManageEntry, TokenUserWithScope},
    };

    #[api_v2_operation]
    pub async fn create(
        _user: TokenUserWithScope<ScopeManageManageEntry>,
        db: Data<PgPool>,
        req: Json<CreateEntryRequest>,
    ) -> Result<CreatedJson<CreateEntryResponse>, error::NotFound> {
        let req = req.into_inner();
        let entry = sqlx::query!(
            r#"
insert into locale_entry (bundle_id, section, item_kind_id, english, hebrew, status, zeplin_reference, comments, in_app, in_element, in_mock)
values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
returning id
"#, req.bundle_id, req.section, req.item_kind_id, req.english, req.hebrew, req.status as i16, req.zeplin_reference, req.comments, req.in_app, req.in_element, req.in_mock
        )
        .fetch_one(db.as_ref())
        .await?;

        Ok(CreatedJson(CreateEntryResponse {
            id: entry.id as u32,
        }))
    }

    #[api_v2_operation]
    pub async fn get(
        db: Data<PgPool>,
        id: Path<u32>,
    ) -> Result<Json<GetEntryResponse>, error::NotFound> {
        let entry = sqlx::query_as!(
            Entry,
            r#"
select
id as "id: u32",
bundle_id,
section,
item_kind_id,
english,
hebrew,
status as "status: EntryStatus",
zeplin_reference,
comments,
in_app, 
in_element, 
in_mock
from
locale_entry
where id = $1
"#,
            id.into_inner() as u32,
        )
        .fetch_one(db.as_ref())
        .await?;

        Ok(Json(GetEntryResponse { entry }))
    }
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(
        locale::bundle::List::PATH,
        locale::bundle::List::METHOD.route().to(list_bundles),
    )
    .route(
        locale::item_kind::List::PATH,
        locale::item_kind::List::METHOD.route().to(list_item_kinds),
    )
    .route(
        locale::entry::Get::PATH,
        locale::entry::Get::METHOD.route().to(entry::get),
    )
    .route(
        locale::entry::Create::PATH,
        locale::entry::Create::METHOD.route().to(entry::create),
        //     )
        //    .route(
        //         locale::entry::List::PATH,
        //         locale::entry::List::METHOD.route().to(entry::list),
        //     )
        //     .route(
        //         locale::entry::Update::PATH,
        //         locale::entry::Update::METHOD.route().to(entry::update),
        //     )
        //     .route(
        //         locale::entry::Delete::PATH,
        //         locale::entry::Delete::METHOD.route().to(entry::delete),
    );
}
