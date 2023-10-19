use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};
use http::StatusCode;
use macros::test_service;

use shared::domain::{meta::AffiliationId, user::PatchProfileRequest};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

mod color;
mod font;
mod public_user;

#[test_service(setup = "setup_service", fixtures("Fixture::User"))]
async fn get_profile(port: u16) -> anyhow::Result<()> {
    let name = "get_profile";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/profile", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}", name), body, {".**.account_id" => "[account_id]"});

    Ok(())
}

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
async fn patch_profile(port: u16) -> anyhow::Result<()> {
    let name = "patch_profile";

    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!("http://0.0.0.0:{}/v1/user/me/profile", port))
        .json(&PatchProfileRequest {
            username: Some("test_user".to_owned()),
            given_name: Some("name".to_owned()),
            family_name: Some("nameson".to_owned()),
            profile_image: None, // FIXME
            bio: Some("a test user".to_owned()),
            languages_spoken_public: Some(true),
            organization_public: Some(true),
            persona_public: Some(true),
            location_public: Some(true),
            bio_public: Some(true),
            language_app: Some("en".to_owned()),
            language_emails: Some("en".to_owned()),
            languages_spoken: Some(vec!["en".to_owned(), "he".to_owned()]),
            timezone: None,
            opt_into_edu_resources: Some(false),
            organization: Some(None),
            persona: Some(vec!["persona".to_owned()]),
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

    insta::assert_json_snapshot!(
    format!("{}", name),
    body, { ".updated_at" => "[timestamptz]", ".**.account_id" => "[account_id]" });

    Ok(())
}

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
async fn browse_user_badges(port: u16) -> anyhow::Result<()> {
    let name = "browse_user_badges";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/browse", port))
        .query(&[("badge", "noBadge")])
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
    format!("{}-1", name),
    body, { ".updated_at" => "[timestamptz]" });

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/browse", port))
        .query(&[("badge", "jiTeam")])
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
    format!("{}-2", name),
    body, { ".updated_at" => "[timestamptz]" });

    Ok(())
}

// Ignored tests aren't captured. Will resolve later
//
// #[ignore]
// #[test_service(
//     setup = "setup_service",
//     fixtures("Fixture::User"),
//     services("Service::Email")
// )]
// async fn verify_email(port: u16) -> anyhow::Result<()> {
//     if !service::email_test_guard() {
//         return Ok(());
//     }

//     let client = reqwest::Client::new();

//     let resp = client
//         .post(&format!("http://0.0.0.0:{}/v1/user/verify-email", port))
//         .json(&json!({
//             "verify": {
//                 "token": "L6gfXvgZeUBt8pdmLBnsGPEWUe3qGCK2_DF"
//             }
//         }))
//         .send()
//         .await?
//         .error_for_status()?;

//     assert_eq!(resp.status(), StatusCode::NO_CONTENT);

//     Ok(())
// }
//
//
//#[ignore]
// #[test_service(
//     setup = "setup_service",
//     fixtures("Fixture::User"),
//     services("Service::Email")
// )]
// async fn post_profile(port: u16) -> anyhow::Result<()> {
//     let name = "post_profile";

//     if !service::email_test_guard() {
//         return Ok(());
//     }

//     const SUB: &str = "Sa84_qiKlh7WbOxeR9lofYJngysK_unF";
//     let csrf: &str = "FOYKzUtD7wCLb7JJ";
//     let key = &**super::helpers::PASETO_KEY;

//     // Generate auth token that the server will accept.
//     // On the front-end this below is handled by basic auth through `POST /v1/session`, which
//     // calls this same function on the backend and returns the csrf + cookie used to authenticate.
//     // See `basic_auth_flow`
//     let token = ji_cloud_api::token::create_auth_token_no_cookie(
//         key,
//         Duration::minutes(10),
//         SUB,
//         csrf.to_owned(),
//         Utc::now(),
//     )
//     .expect("failed to create auth token");

//     let client = reqwest::Client::new();

//     // create user profile
//     let resp = client
//         .post(&format!("http://0.0.0.0:{}/v1/user/me/profile", port))
//         .header("X-CSRF", csrf)
//         .header("Cookie", format!("X-AUTH={}", token))
//         .json(&CreateProfileRequest {
//             username: "test_user".to_owned(),
//             over_18: true,
//             given_name: "name".to_owned(),
//             family_name: "nameson".to_owned(),
//             profile_image_url: None,
//             language_app: "en".to_owned(),
//             language_emails: "en".to_owned(),
//             languages_spoken: vec!["en".to_owned(), "he".to_owned()],
//             timezone: chrono_tz::America::Los_Angeles,
//             opt_into_edu_resources: true,
//             organization: None,
//             persona: vec!["personesaa".to_owned(), "soma".to_owned()],
//             subjects: Vec::<SubjectId>::new(),
//             age_ranges: Vec::<AgeRangeId>::new(),
//             affiliations: Vec::<AffiliationId>::new(),
//             location: None,
//         })
//         .send()
//         .await?
//         .error_for_status()?;

//     assert_eq!(resp.status(), StatusCode::CREATED);

//     let body: serde_json::Value = resp.json().await?;

//     insta::assert_json_snapshot!(
//         format!("{}",name),
//         body, {".csrf" => "[csrf]"});

//     Ok(())
// }
