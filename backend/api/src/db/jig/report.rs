use crate::error;
use shared::domain::jig::{
    report::{JigReport, JigReportEmail, JigReportType, ReportId},
    JigId,
};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

pub async fn create_report(
    pool: &PgPool,
    jig_id: JigId,
    report_type: JigReportType,
    user_id: Option<Uuid>,
) -> Result<ReportId, error::ReportError> {
    check_jig(pool, jig_id).await?;

    if let Some(user_id) = user_id {
        sqlx::query!(
            r#"
    insert into jig_report(jig_id, report_type, reporter_id)
    values ($1, $2, $3)
    returning id as "id!: ReportId"
            "#,
            jig_id.0,
            report_type as i16,
            user_id
        )
        .fetch_one(pool)
        .await
        .map(|it| it.id)
        .map_err(Into::into)
    } else {
        sqlx::query!(
            r#"
    insert into jig_report(jig_id, report_type)
    values ($1, $2)
    returning id as "id!: ReportId"
            "#,
            jig_id.0,
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
    jig_id: JigId,
    report_id: ReportId,
) -> Result<Option<JigReport>, error::ReportError> {
    let report = sqlx::query!(
        //language=SQL
        r#"
select id                                   as "id!: ReportId",
       jig_id                               as "jig_id!: JigId",    
       report_type                          as "report_type!: JigReportType",                  
       created_at,
       reporter_id                          as "reporter_id?: Uuid",
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
from jig_report
where id = $1 and jig_id = $2
"#,
        report_id.0,
        jig_id.0
    )
    .fetch_optional(pool)
    .await?
    .map(|row| JigReport {
        id: row.id,
        jig_id: row.jig_id,
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
    jig_id: JigId,
    report_id: ReportId,
) -> Result<Option<JigReportEmail>, error::ReportError> {
    let report = sqlx::query!(
        //language=SQL
        r#"
select display_name                               as "display_name!",    
       report_type                                as "report_type!: JigReportType",                  
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
from jig_report
    left join jig on jig.id = jig_report.jig_id
    left join jig_data on jig_data.id = jig.live_id
where jig_report.id = $1 and jig_report.jig_id = $2
"#,
        report_id.0,
        jig_id.0
    )
    .fetch_optional(&mut *conn)
    .await?
    .map(|row| JigReportEmail {
        display_name: row.display_name,
        report_type: row.report_type,
        reporter_name: row.name,
        reporter_email: row.email,
        creator_name: row.creator_name,
    });

    Ok(report)
}

pub async fn check_jig(db: &PgPool, jig_id: JigId) -> Result<(), error::ReportError> {
    let jig = sqlx::query!(
        //language=SQL
        r#"
select exists (
    select 1 from jig where id = $1
) as "authed!"
"#,
        jig_id.0
    )
    .fetch_one(db)
    .await?
    .authed;

    if !jig {
        return Err(error::ReportError::ResourceNotFound);
    }

    Ok(())
}
