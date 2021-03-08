use shared::domain::locale::UpdateEntryRequest;
use sqlx::PgConnection;

use crate::error;

pub async fn update_entry(
    db: &mut PgConnection,
    id: i32,
    req: UpdateEntryRequest,
) -> Result<(), error::NotFound> {
    let entry_exists = sqlx::query!(
        r#"select exists(select 1 from locale_entry where id = $1 for update) as "exists!""#,
        id
    )
    .fetch_one(&mut *db)
    .await?;

    if !entry_exists.exists {
        return Err(error::NotFound::ResourceNotFound);
    }

    let section = (req.section.is_some(), req.section.flatten());

    let zeplin_reference = (
        req.zeplin_reference.is_some(),
        req.zeplin_reference.flatten(),
    );

    let comments = (req.comments.is_some(), req.comments.flatten());

    sqlx::query!(
        r#"
update locale_entry
set
    bundle_id = coalesce(bundle_id, $2),
    item_kind_id = coalesce(item_kind_id, $3),
    english = coalesce(english, $4),
    hebrew = coalesce(hebrew, $5),
    status = coalesce(status, $6),
    in_app = coalesce(in_app, $7),
    in_element = coalesce(in_element, $8),
    in_mock = coalesce(in_mock, $9),
    section = case when $10 then $11 else section end,
    zeplin_reference = case when $12 then $13 else zeplin_reference end,
    comments = case when $14 then $15 else comments end
where id = $1"#,
        id,
        req.bundle_id,
        req.item_kind_id,
        req.english,
        req.hebrew,
        req.status.map(|status| status as i16),
        req.in_app,
        req.in_element,
        req.in_mock,
        section.0,
        section.1.as_deref(),
        zeplin_reference.0,
        zeplin_reference.1.as_deref(),
        comments.0,
        comments.1.as_deref(),
    )
    .execute(db)
    .await?;

    Ok(())
}
