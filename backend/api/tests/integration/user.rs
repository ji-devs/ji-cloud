use crate::{
    fixture::Fixture,
    helpers::{initialize_server, initialize_server_and_get_db, LoginExt},
    service::{self, Service},
};
use chrono::{Duration, Utc};
use http::StatusCode;
use ji_cloud_api::http::Application;
use serde_json::json;
use shared::domain::session::NewSessionResponse;
use shared::domain::user::PatchProfileRequest;
use shared::domain::{
    meta::{AffiliationId, AgeRangeId, SubjectId},
    session::CreateSessionResponse,
    user::PutProfileRequest,
};
use sqlx::PgPool;

mod color;
mod font;

#[actix_rt::test]
async fn get_profile() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/profile", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[actix_rt::test]
async fn put_profile() -> anyhow::Result<()> {
    if !service::email_test_guard() {
        return Ok(());
    }

    const SUB: &str = "Sa84_qiKlh7WbOxeR9lofYJngysK_unF";
    let csrf: &str = "FOYKzUtD7wCLb7JJ";
    let key = &**super::helpers::PASETO_KEY;

    // Generate auth token that the server will accept.
    // On the front-end this below is handled by basic auth through `POST /v1/session`, which
    // calls this same function on the backend and returns the csrf + cookie used to authenticate.
    // See `basic_auth_flow`
    let token = ji_cloud_api::token::create_auth_token_no_cookie(
        key,
        Duration::minutes(10),
        SUB,
        csrf.to_owned(),
        Utc::now(),
    )
    .expect("failed to create auth token");

    // test server application
    let app = initialize_server(&[Fixture::User], &[Service::Email]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    // put profile
    let resp = client
        .put(&format!("http://0.0.0.0:{}/v1/user/me/profile", port))
        .header("X-CSRF", csrf)
        .header("Cookie", format!("X-AUTH={}", token))
        .json(&PutProfileRequest {
            username: "test_user".to_owned(),
            over_18: true,
            given_name: "name".to_owned(),
            family_name: "nameson".to_owned(),
            language: "en_US".to_owned(),
            locale: "en_US".to_owned(),
            timezone: chrono_tz::America::Los_Angeles,
            opt_into_edu_resources: true,
            organization: None,
            subjects: Vec::<SubjectId>::new(),
            age_ranges: Vec::<AgeRangeId>::new(),
            affiliations: Vec::<AffiliationId>::new(),
            location: None,
        })
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".csrf" => "[csrf]"});

    Ok(())
}

#[actix_rt::test]
async fn patch_profile() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!("http://0.0.0.0:{}/v1/user/me/profile", port))
        .json(&PatchProfileRequest {
            username: Some("test_user".to_owned()),
            over_18: Some(false),
            given_name: Some("name".to_owned()),
            family_name: Some("nameson".to_owned()),
            language: Some("en_US".to_owned()),
            locale: Some("en_US".to_owned()),
            timezone: None,
            opt_into_edu_resources: Some(false),
            organization: Some(None),
            subjects: None,
            age_ranges: None,
            affiliations: Some(Vec::<AffiliationId>::new()),
            location: None,
        })
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/profile", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body, { ".updated_at" => "[timestamptz]" });

    Ok(())
}

#[ignore]
#[actix_rt::test]
async fn verify_email() -> anyhow::Result<()> {
    if !service::email_test_guard() {
        return Ok(());
    }

    let app = initialize_server(&[Fixture::User], &[Service::Email]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/user/verify-email", port))
        .json(&json!({
            "verify": {
                "token": "L6gfXvgZeUBt8pdmLBnsGPEWUe3qGCK2_DF"
            }
        }))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    Ok(())
}

#[ignore]
#[actix_rt::test]
async fn basic_auth_flow_no_login() -> anyhow::Result<()> {
    if !service::email_test_guard() {
        return Ok(());
    }

    const EMAIL: &str = "testfakeemailnotreal@iiiiiis.test";
    const PASSWORD: &str = "badpassword";

    let (app, db): (Application, PgPool) =
        initialize_server_and_get_db(&[], &[Service::Email]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    // 0. register basic auth user
    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/user", port))
        .json(&json!({
            "email": EMAIL,
            "password": PASSWORD,
        }))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // 1. request resend email
    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/user/verify-email", port))
        .json(&json!({
            "resend": {"email": "testfakeemailnotreal@iiiiiis.test" }
        }))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // 1.1. retrieve verification token through database directly
    let mut txn = db.begin().await?;
    let token = sqlx::query!(r#"select token from "session" order by created_at limit 1"#)
        .fetch_one(&mut txn)
        .await?
        .token;

    // 2. verify email
    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/user/verify-email", port))
        .json(&json!({
            "verify": { "token": token }
        }))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    // 3. Create user profile for the first time

    // 3.1. extract auth info from response
    let token = resp.cookies().next().unwrap().value().to_owned();
    log::info!("{:?}", &token);
    let body = resp.json::<NewSessionResponse>().await?;
    log::info!("{:?}", &body);

    // 3.2. put profile
    let resp = client
        .put(&format!("http://0.0.0.0:{}/v1/user/me/profile", port))
        .header("X-CSRF", body.csrf.as_str())
        .header("Cookie", format!("X-AUTH={}", token))
        .json(&PutProfileRequest {
            username: "test_user".to_owned(),
            over_18: true,
            given_name: "name".to_owned(),
            family_name: "nameson".to_owned(),
            language: "en_US".to_owned(),
            locale: "en_US".to_owned(),
            timezone: chrono_tz::America::Los_Angeles,
            opt_into_edu_resources: true,
            organization: None,
            subjects: Vec::<SubjectId>::new(),
            age_ranges: Vec::<AgeRangeId>::new(),
            affiliations: Vec::<AffiliationId>::new(),
            location: None,
        })
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;
    txn.commit().await?;

    insta::assert_json_snapshot!(body, {".csrf" => "[csrf]"});

    Ok(())
}

#[ignore]
#[actix_rt::test]
async fn basic_auth_flow() -> anyhow::Result<()> {
    if !service::email_test_guard() {
        return Ok(());
    }

    const EMAIL: &str = "testfakeemailnotreal@iiiiiis.test";
    const PASSWORD: &str = "badpassword";

    let (app, db): (Application, PgPool) =
        initialize_server_and_get_db(&[], &[Service::Email]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    // 0. register basic auth user
    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/user", port))
        .json(&json!({
            "email": EMAIL,
            "password": PASSWORD,
        }))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // 1. request resend email
    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/user/verify-email", port))
        .json(&json!({
            "resend": {"email": "testfakeemailnotreal@iiiiiis.test" }
        }))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // 1.1. retrieve verification token through database directly
    let mut txn = db.begin().await?;
    let token = sqlx::query!(r#"select token from "session" order by created_at limit 1"#)
        .fetch_one(&mut txn)
        .await?
        .token;

    // 2. verify email
    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/user/verify-email", port))
        .json(&json!({
            "verify": { "token": token }
        }))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    // 3. login via basic auth `POST /v1/session`

    // 3.1. bad password/username. rejected login
    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/session", port))
        .basic_auth(EMAIL, Some("aasdasda"))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // 3.2. basic auth login successful
    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/session", port))
        .basic_auth(EMAIL, Some(PASSWORD))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    // 4. Create user profile for the first time

    // 4.1. extract auth info from response
    let token = resp.cookies().next().unwrap().value().to_owned();
    let body = resp.json::<CreateSessionResponse>().await?;

    let csrf = match body {
        CreateSessionResponse::Register(resp) => resp.csrf,
        _ => {
            return Err(anyhow::anyhow!(
                "invalid session response to create a user profile!"
            ))
        }
    };

    // 4.2. put profile
    let resp = client
        .put(&format!("http://0.0.0.0:{}/v1/user/me/profile", port))
        .header("X-CSRF", csrf.as_str())
        .header("Cookie", format!("X-AUTH={}", token))
        .json(&PutProfileRequest {
            username: "test_user".to_owned(),
            over_18: true,
            given_name: "name".to_owned(),
            family_name: "nameson".to_owned(),
            language: "en_US".to_owned(),
            locale: "en_US".to_owned(),
            timezone: chrono_tz::America::Los_Angeles,
            opt_into_edu_resources: true,
            organization: None,
            subjects: Vec::<SubjectId>::new(),
            age_ranges: Vec::<AgeRangeId>::new(),
            affiliations: Vec::<AffiliationId>::new(),
            location: None,
        })
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;
    txn.commit().await?;

    insta::assert_json_snapshot!(body, {".csrf" => "[csrf]"});

    Ok(())
}
