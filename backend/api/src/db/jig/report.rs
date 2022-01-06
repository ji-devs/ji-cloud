use crate::error;
use crate::extractor::TokenUser;
use shared::domain::jig::{
    report::{JigReport, JigReportType, ReportId},
    JigId,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_report(
    pool: &PgPool,
    jig_id: JigId,
    report_type: JigReportType,
    claims: Option<TokenUser>,
) -> Result<ReportId, error::ReportError> {
    check_jig(pool, jig_id).await?;

    if let Some(reporter_id) = claims {
        sqlx::query!(
            r#"
    insert into jig_report(jig_id, report_type, reporter_id)
    values ($1, $2, $3)
    returning id as "id!: ReportId"
            "#,
            jig_id.0,
            report_type as i16,
            reporter_id.0.user_id
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
