use std::cmp;

use serde::{Deserialize, Serialize};
use serde_json::{json, value::Value};
use shared::domain::{
    asset::DraftOrLive,
    audio::AudioId,
    course::{
        unit::{CourseUnitId, CourseUnitValue},
        CourseId,
    },
    image::ImageId,
    pdf::PdfId,
};
use sqlx::PgPool;
use url::Url;

use crate::error;

#[derive(Deserialize, Serialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[serde(rename_all = "camelCase")]
pub struct ResourceObject {
    content: serde_json::Value,
}

pub async fn create(
    pool: &PgPool,
    course_id: CourseId,
    display_name: String,
    description: String,
    unit_content: CourseUnitValue,
) -> anyhow::Result<CourseUnitId> {
    let unit: serde_json::Value = check_value(pool, unit_content).await?;

    let mut txn = pool.begin().await?;

    let draft_id = sqlx::query!(
        //language=SQL
        r#"
select draft_id from course where id = $1
"#,
        course_id.0,
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.draft_id);

    let res = sqlx::query!(
        r#"
insert into course_data_unit (course_data_id, display_name, description, value, index)
values ((select draft_id from course where id = $1), $2, $3, $4, (select count(*) from course_data_unit where course_data_id = $5))
returning unit_id as "unit_id!: CourseUnitId"
        "#,
        course_id.0,
        display_name,
        description,
        unit,
        draft_id
    )
    .fetch_one(pool)
    .await
    .map(|it| it.unit_id)
    .map_err(Into::into);

    txn.commit().await?;

    res
}

pub async fn get(
    pool: &PgPool,
    course_id: CourseId,
    draft_or_live: DraftOrLive,
    unit_id: CourseUnitId,
) -> anyhow::Result<(CourseUnitId, String, String, CourseUnitValue), error::NotFound> {
    let mut txn = pool.begin().await?;

    let (draft_id, live_id) = super::get_draft_and_live_ids(&mut txn, course_id)
        .await
        .ok_or(error::NotFound::ResourceNotFound)?;

    let course_data_id = match draft_or_live {
        DraftOrLive::Draft => draft_id,
        DraftOrLive::Live => live_id,
    };

    if !sqlx::query!(
        //language=SQL
        r#"
select exists(select 1 from course_data_unit "pddu" where course_data_id = $1
    and pddu.unit_id = $2) as "exists!"
    "#,
        course_data_id,
        unit_id.0,
    )
    .fetch_one(&mut txn)
    .await?
    .exists
    {
        return Err(error::NotFound::ResourceNotFound);
    }

    let res = sqlx::query!(
        r#"
select unit_id              as "id!: CourseUnitId",
       display_name         as "display_name!",
       description          as "description!",
       value                as "value!"
from course_data_unit "pddr"
where course_data_id = $1
  and pddr.unit_id = $2
        "#,
        course_data_id,
        unit_id.0,
    )
    .fetch_one(&mut txn)
    .await?;

    let content = serde_json::from_value::<CourseUnitValue>(res.value)?;

    txn.rollback().await?;

    Ok((res.id, res.display_name, res.description, content))
}

pub async fn update(
    pool: &PgPool,
    course_id: CourseId,
    unit_id: CourseUnitId,
    display_name: Option<String>,
    description: Option<String>,
    unit_value: Option<CourseUnitValue>,
    new_index: Option<u16>,
) -> anyhow::Result<bool, error::Auth> {
    let mut txn = pool.begin().await?;

    let draft_id = sqlx::query!(
        //language=SQL
        r#"
select draft_id from course where course.id = $1
"#,
        course_id.0,
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.draft_id);

    let index = sqlx::query!(
        //language=SQL
        r#"
select index from course_data_unit
where course_data_id = $1 and course_data_unit.unit_id is not distinct from $2
"#,
        draft_id,
        unit_id.0,
    )
    .fetch_optional(&mut txn)
    .await?;

    let index = match index {
        Some(it) => it.index,
        None => return Ok(false),
    };

    sqlx::query!(
        //language=SQL
        r#"
update course_data_unit
set display_name    = coalesce($3, display_name),
    description     = coalesce($4, description)
where course_data_id = $1
  and index = $2
"#,
        draft_id,
        index,
        display_name,
        description,
    )
    .execute(&mut txn)
    .await?;

    if let Some(unit_value) = unit_value {
        let unit: serde_json::Value = check_value(pool, unit_value).await?;
        sqlx::query!(
            //language=SQL
            r#"
    update course_data_unit
    set value = coalesce($3, value)
    where course_data_id = $1
      and index = $2
    "#,
            draft_id,
            index,
            json!(unit),
        )
        .execute(&mut txn)
        .await?;
    };

    if let Some(new_index) = new_index {
        let new_index = new_index as i16;

        let max_index = sqlx::query!(
            //language=SQL
            r#"select count(*) - 1 as "max_index!" from course_data_unit where course_data_id = $1"#,
            draft_id
        )
        .fetch_one(&mut txn)
        .await?
        .max_index;

        let new_index = cmp::min(new_index, max_index as i16);

        if new_index < index {
            sqlx::query!(
                //language=SQL
                r#"
update course_data_unit
set
    index = case when index = $2 then $3 else index + 1 end,
    updated_at = now()
where course_data_id = $1 and index between $3 and $2
"#,
                draft_id,
                index,
                new_index
            )
            .execute(&mut txn)
            .await?;
        } else if new_index > index {
            sqlx::query!(
                //language=SQL
                r#"
update course_data_unit
set
    index = case when index = $2 then $3 else index - 1 end,
    updated_at = now()
where course_data_id = $1 and index between $2 and $3
"#,
                draft_id,
                index,
                new_index
            )
            .execute(&mut txn)
            .await?;
        }
    }

    txn.commit().await?;

    Ok(true)
}

pub async fn delete(
    pool: &PgPool,
    course_id: CourseId,
    unit_id: CourseUnitId,
) -> anyhow::Result<()> {
    let mut txn = pool.begin().await?;

    let draft_id = sqlx::query!(
        //language=SQL
        r#"
select draft_id from course where course.id = $1
"#,
        course_id.0,
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.draft_id);

    let idx = sqlx::query!(
        //language=SQL
        r#"
delete
from course_data_unit
where course_data_id = $1 and course_data_unit.unit_id is not distinct from $2
returning index
"#,
        draft_id,
        unit_id.0,
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.index);

    if let Some(idx) = idx {
        sqlx::query!(
            //language=SQL
            r#"
update course_data_unit
set index = index - 1
where course_data_id = $1
  and index > $2
"#,
            draft_id,
            idx,
        )
        .execute(&mut txn)
        .await?;
    }

    txn.commit().await?;

    Ok(())
}

pub async fn check_value(db: &PgPool, unit_value: CourseUnitValue) -> anyhow::Result<Value> {
    let unit_value: serde_json::Value = match unit_value {
        CourseUnitValue::ImageId(data) => {
            sqlx::query!(
                r#"select id as "id: ImageId" from user_image_library where id = $1"#,
                data.0
            )
            .fetch_one(db)
            .await
            .map_err(|_| anyhow::anyhow!("Image Id does not exist"))?;

            json!(CourseUnitValue::ImageId(data))
        }
        CourseUnitValue::AudioId(data) => {
            sqlx::query!(
                r#"select id as "id: AudioId" from user_audio_library where id = $1"#,
                data.0
            )
            .fetch_one(db)
            .await
            .map_err(|_| anyhow::anyhow!("Audio Id does not exist"))?;

            json!(CourseUnitValue::AudioId(data))
        }
        CourseUnitValue::Link(data) => {
            let data = Url::parse(data.as_str())?;

            json!(CourseUnitValue::Link(data))
        }
        CourseUnitValue::Video(data) => {
            json!(CourseUnitValue::Video(data))
        }
        CourseUnitValue::PdfId(data) => {
            sqlx::query!(
                r#"select id as "id: PdfId" from user_pdf_library where id = $1"#,
                data.0
            )
            .fetch_one(db)
            .await
            .map_err(|_| anyhow::anyhow!("Pdf Id does not exist"))?;

            json!(CourseUnitValue::PdfId(data))
        }
    };

    Ok(unit_value)
}
