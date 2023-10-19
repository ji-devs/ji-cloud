use crate::error;
use shared::domain::{
    resource::{
        report::{ReportId, ResourceReport, ResourceReportEmail, ResourceReportType},
        ResourceId,
    },
    user::UserId,
};
use sqlx::{PgConnection, PgPool};

pub async fn create_report(
    pool: &PgPool,
    resource_id: ResourceId,
    report_type: ResourceReportType,
    user_id: Option<UserId>,
) -> Result<ReportId, error::ReportError> {
    check_resource(pool, resource_id).await?;

    if let Some(user_id) = user_id {
        sqlx::query!(
            r#"
    insert into resource_report(resource_id, report_type, reporter_id)
    values ($1, $2, $3)
    returning id as "id!: ReportId"
            "#,
            resource_id.0,
            report_type as i16,
            user_id.0
        )
        .fetch_one(pool)
        .await
        .map(|it| it.id)
        .map_err(Into::into)
    } else {
        sqlx::query!(
            r#"
    insert into resource_report(resource_id, report_type)
    values ($1, $2)
    returning id as "id!: ReportId"
            "#,
            resource_id.0,
            report_type as i16,
        )
        .fetch_one(pool)
        .await
        .map(|it| it.id)
        .map_err(Into::into)
    }
}

pub async fn get_report(
    pool: &PgPool,
    resource_id: ResourceId,
    report_id: ReportId,
) -> Result<Option<ResourceReport>, error::ReportError> {
    let report = sqlx::query!(
        //language=SQL
        r#"
select id                                   as "id!: ReportId",
       resource_id                               as "resource_id!: ResourceId",    
       report_type                          as "report_type!: ResourceReportType",                  
       created_at,
       reporter_id                          as "reporter_id?: uuid::Uuid",
       (
            select given_name || ' '::text || family_name
            from user_profile
            where user_profile.user_id = reporter_id
        )                                       as "name?",
        (
            select email::text
            from user_email
            where user_email.user_id = reporter_id
        )                                       as "email?"
from resource_report
where id = $1 and resource_id = $2
"#,
        report_id.0,
        resource_id.0
    )
    .fetch_optional(pool)
    .await?
    .map(|row| ResourceReport {
        id: row.id,
        resource_id: row.resource_id,
        report_type: row.report_type,
        reporter_id: row.reporter_id,
        reporter_name: row.name,
        reporter_email: row.email,
        created_at: row.created_at,
    });

    Ok(report)
}

pub async fn get_report_email(
    conn: &mut PgConnection,
    resource_id: ResourceId,
    report_id: ReportId,
) -> Result<Option<ResourceReportEmail>, error::ReportError> {
    let report = sqlx::query!(
        //language=SQL
        r#"
select display_name                               as "display_name!",    
       report_type                                as "report_type!: ResourceReportType",                  
       (
            select given_name || ' '::text || family_name
            from user_profile
            where user_profile.user_id = reporter_id
        )                                       as "name?",
        (
            select email::text
            from user_email
            where user_email.user_id = reporter_id
        )                                       as "email?",
        (
            select given_name || ' '::text || family_name
            from user_profile
            where user_profile.user_id = creator_id
        )                                       as "creator_name!"
from resource_report
    left join resource on resource.id = resource_report.resource_id
    left join resource_data on resource_data.id = resource.live_id
where resource_report.id = $1 and resource_report.resource_id = $2
"#,
        report_id.0,
        resource_id.0
    )
    .fetch_optional(&mut *conn)
    .await?
    .map(|row| ResourceReportEmail {
        display_name: row.display_name,
        report_type: row.report_type,
        reporter_name: row.name,
        reporter_email: row.email,
        creator_name: row.creator_name,
    });

    Ok(report)
}

pub async fn check_resource(
    db: &PgPool,
    resource_id: ResourceId,
) -> Result<(), error::ReportError> {
    let resource = sqlx::query!(
        //language=SQL
        r#"
select exists (
    select 1 from resource where id = $1
) as "authed!"
"#,
        resource_id.0
    )
    .fetch_one(db)
    .await?
    .authed;

    if !resource {
        return Err(error::ReportError::ResourceNotFound);
    }

    Ok(())
}
