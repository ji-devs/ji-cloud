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
    use std::collections::BTreeMap;

    use paperclip::actix::{
        api_v2_operation,
        web::{Data, Json, Path, Query},
        CreatedJson, NoContent,
    };
    use shared::domain::locale::{
        CreateEntryRequest, CreateEntryResponse, Entry, EntryStatus, GetEntryResponse,
        ListEntryGroupBy, ListEntryQuery, ListEntryResponse, UpdateEntryRequest,
    };
    use sqlx::PgPool;

    use crate::{
        db, error,
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
    pub async fn list(
        db: Data<PgPool>,
        query: Option<Query<ListEntryQuery>>,
    ) -> Result<Json<ListEntryResponse>, error::Server> {
        let query = query.map_or_else(ListEntryQuery::default, Query::into_inner);

        let entries = sqlx::query_as!(
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
from locale_entry
where $2 or bundle_id = any($1)
order by id
"#,
            &query.bundles,
            query.bundles.is_empty(),
        )
        .fetch_all(db.as_ref())
        .await?;

        match query.group_by {
            ListEntryGroupBy::None => Ok(Json(ListEntryResponse::List(entries))),
            ListEntryGroupBy::Bundle => {
                let mut map = BTreeMap::new();
                for entry in entries {
                    map.entry(entry.bundle_id)
                        .or_insert_with(Vec::new)
                        .push(entry);
                }

                Ok(Json(ListEntryResponse::Bundles(map)))
            }

            it => Err(anyhow::anyhow!("Unknown groupBy kind: {:?}", it).into()),
        }
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
from locale_entry
where id = $1
"#,
            id.into_inner() as u32,
        )
        .fetch_optional(db.as_ref())
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

        Ok(Json(GetEntryResponse { entry }))
    }

    #[api_v2_operation]
    pub async fn update(
        _user: TokenUserWithScope<ScopeManageManageEntry>,
        db: Data<PgPool>,
        id: Path<u32>,
        req: Json<UpdateEntryRequest>,
    ) -> Result<NoContent, error::NotFound> {
        // todo: use a more descriptive error.
        let id = id.into_inner() as i32;
        let req = req.into_inner();

        let mut txn = db.begin().await?;
        db::locale::update_entry(&mut txn, id, req).await?;
        txn.commit().await?;

        Ok(NoContent)
    }

    #[api_v2_operation]
    pub async fn delete(
        _user: TokenUserWithScope<ScopeManageManageEntry>,
        db: Data<PgPool>,
        id: Path<u32>,
    ) -> Result<NoContent, error::Delete> {
        sqlx::query!(
            "delete from locale_entry where id = $1",
            id.into_inner() as i32
        )
        .execute(db.as_ref())
        .await?;

        Ok(NoContent)
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
    )
    .route(
        locale::entry::List::PATH,
        locale::entry::List::METHOD.route().to(entry::list),
    )
    .route(
        locale::entry::Update::PATH,
        locale::entry::Update::METHOD.route().to(entry::update),
    )
    .route(
        locale::entry::Delete::PATH,
        locale::entry::Delete::METHOD.route().to(entry::delete),
    );
}
