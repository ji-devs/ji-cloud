use http::StatusCode;
use macros::test_service;
use serde_json::json;
use shared::domain::{jig::JigId, CreateResponse};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};

mod additional_resource;
mod cover;
mod module;
mod player;

#[test_service(setup = "setup_service", fixtures("Fixture::User"))]
async fn create_default(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<JigId> = resp.json().await?;

    insta::assert_json_snapshot!(body, {".id" => "[id]"});

    let jig_id = body.id.0;

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/{}/draft", port, jig_id))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.id" => "[id]",
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]"
        }
    );

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/{}/live", port, jig_id))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.id" => "[id]",
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]"
        }
    );

    Ok(())
}

// requires algolia
// #[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Jig"))]
// async fn delete(pool_opts: PoolOptions<Postgres>, conn_opts: PgConnectOptions<Postgres>) -> anyhow::Result<()> {
//     let app = initialize_server(&[Fixture::User, Fixture::Jig]).await;

//     let port = app.port();

//     tokio::spawn(app.run_until_stopped());

//     let client = reqwest::Client::new();

//     let resp = client
//         .delete(&format!(
//             "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008",
//             port
//         ))
//         .login()
//         .send()
//         .await?
//         .error_for_status()?;

//     assert_eq!(resp.status(), StatusCode::NO_CONTENT);

//     Ok(())
// }

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Jig")
)]
async fn create_with_params(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig", port))
        .json(&json!({
            "modules": ["0cbfdd82-7c83-11eb-9f77-d7d86264c3bc"],
            "display_name": "test jig",
            "description": "test description",
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<JigId> = resp.json().await?;

    insta::assert_json_snapshot!(body, {".id" => "[id]", ".last_edited" => "[last_edited]"});

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Jig")
)]
async fn clone(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/jig/3a71522a-cd77-11eb-8dc1-af3e35f7c743/clone",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let CreateResponse { id: JigId(id) } = resp.json().await?;

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/{}/draft", port, id))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.id" => "[id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.additionalResources" => "[ids]"
        }
    );

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/{}/live", port, id))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.id" => "[id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]",
            ".**.additionalResources" => "[ids]"
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Jig")
)]
async fn get(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]"
        }
    );

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/live",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]"
        }
    );

    Ok(())
}

// todo: test-exhaustiveness: create a `JigBrowse` Fixture, actually test the cases (paging, jig count, etc)
#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Jig")
)]
async fn browse_simple(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/browse", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]"
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Jig")
)]
async fn browse_order_by(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/browse", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]"
        }
    );

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/browse", port))
        .login()
        .json(&json!({
            "orderBy": "createdAt",
        }))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]"
        }
    );

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/browse", port))
        .login()
        .json(&json!({
            "orderBy": "publishedAt",
        }))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]"
        }
    );

    Ok(())
}

// todo: test-exhaustiveness: create a `JigBrowse` Fixture, actually test the cases (paging, jig count, etc)
#[ignore]
#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::UserDefaultPerms", "Fixture::Jig")
)]
async fn browse_own_simple(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/browse?authorId=me",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]"
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::UserDefaultPerms", "Fixture::Jig")
)]
async fn count(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/count", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::MetaKinds",
        "Fixture::User",
        "Fixture::Jig",
        "Fixture::CategoryOrdering"
    )
)]
async fn update_and_publish(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/19becb2b-bff7-4c1b-bb2c-16f2e098d3d3/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]",
        }
    );

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/jig/19becb2b-bff7-4c1b-bb2c-16f2e098d3d3",
            port
        ))
        .json(&json!({
            "description": "asdasdasd",
            "language": "en-us",
            "categories":["7fe19326-e883-11ea-93f0-5343493c17c4", "81c4796a-e883-11ea-93f0-df2484ab6b11"],
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/19becb2b-bff7-4c1b-bb2c-16f2e098d3d3/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]",
        }
    );

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/19becb2b-bff7-4c1b-bb2c-16f2e098d3d3/live",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]",
        }
    );

    let _resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/jig/19becb2b-bff7-4c1b-bb2c-16f2e098d3d3/draft/publish",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/19becb2b-bff7-4c1b-bb2c-16f2e098d3d3/live",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            // Really just need to redact the module ID because it is recreated for the live data,
            // but I couldn't get a selector working correctly... So redacting all IDs.
            ".**.id" => "[id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.publishedAt" => "[published_at]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]"
        }
    );

    Ok(())
}

#[ignore]
#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::MetaKinds",
        "Fixture::User",
        "Fixture::Jig",
        "Fixture::CategoryOrdering"
    )
)]
async fn update_and_publish_incomplete_modules(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    // Test no modules on JIG returns HTTP 400
    let resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/jig/3a71522a-cd77-11eb-8dc1-af3e35f7c743/draft/publish",
            port
        ))
        .login()
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    // Test no modules on JIG returns HTTP 400
    let resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft/publish",
            port
        ))
        .login()
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::MetaKinds",
        "Fixture::User",
        "Fixture::Jig",
        "Fixture::CategoryOrdering"
    )
)]
async fn live_up_to_date_flag(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/19becb2b-bff7-4c1b-bb2c-16f2e098d3d3/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]",
        }
    );

    let _resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/jig/19becb2b-bff7-4c1b-bb2c-16f2e098d3d3/draft/publish",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/19becb2b-bff7-4c1b-bb2c-16f2e098d3d3/live",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            // Really just need to redact the module ID because it is recreated for the live data,
            // but I couldn't get a selector working correctly... So redacting all IDs.
            ".**.id" => "[id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.publishedAt" => "[published_at]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]"
        }
    );

    Ok(())
}
