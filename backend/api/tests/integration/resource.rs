use http::StatusCode;
use serde_json::json;
use shared::domain::{resource::ResourceId, CreateResponse};
use sqlx::PgPool;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

mod cover;
mod curation;

#[sqlx::test]
async fn create_default(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User], &[], pool).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/resource", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    println!("resp: {:?}", resp);

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<ResourceId> = resp.json().await?;

    insta::assert_json_snapshot!(body, {".id" => "[id]"});

    let resource_id = body.id.0;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/{}/draft",
            port, resource_id
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.id" => "[id]",
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]"
        }
    );

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/{}/live",
            port, resource_id
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.id" => "[id]",
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]"
        }
    );

    Ok(())
}

#[sqlx::test]
async fn create_with_params(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::MetaKinds, Fixture::User, Fixture::Resource],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/resource", port))
        .json(&json!({
            "modules": ["a6b248f8-1dd7-11ec-8426-975953035335"],
            "display_name": "test resource",
            "description": "test description",
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<ResourceId> = resp.json().await?;

    insta::assert_json_snapshot!(body, {".id" => "[id]", ".last_edited" => "[last_edited]"});

    Ok(())
}

#[sqlx::test]
async fn clone(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::MetaKinds, Fixture::User, Fixture::Resource],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resource_id = "d8067526-1518-11ed-87fa-ebaf880b6d9c".to_string();

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/resource/{}/clone",
            port, resource_id
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let CreateResponse { id: ResourceId(id) } = resp.json().await?;

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/resource/{}/draft", port, id))
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
        .get(&format!("http://0.0.0.0:{}/v1/resource/{}/live", port, id))
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

    Ok(())
}

#[sqlx::test]
async fn get(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::MetaKinds, Fixture::User, Fixture::Resource],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resource_id = "d8067526-1518-11ed-87fa-ebaf880b6d9c".to_string();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}/draft",
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
            ".**.lastEdited" => "[last_edited]"
        }
    );

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}/live",
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
            ".**.lastEdited" => "[last_edited]"
        }
    );

    Ok(())
}

// todo: test-exhaustiveness: create a `ResourceBrowse` Fixture, actually test the cases (paging, resource count, etc)
#[sqlx::test]
async fn browse_simple(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::MetaKinds, Fixture::User, Fixture::Resource],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/resource/browse", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]"
        }
    );

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/browse?draftOrLive=draft",
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
            ".**.lastEdited" => "[last_edited]"
        }
    );

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/browse?draftOrLive=live",
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
            ".**.lastEdited" => "[last_edited]"
        }
    );

    Ok(())
}

#[sqlx::test]
async fn browse_order_by(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::MetaKinds, Fixture::User, Fixture::Resource],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/resource/browse", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/browse?orderBy=createdAt",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/browse?orderBy=publishedAt",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

// todo: test-exhaustiveness: create a `ResourceBrowse` Fixture, actually test the cases (paging, resource count, etc)
#[sqlx::test]
async fn browse_own_simple(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::MetaKinds, Fixture::User, Fixture::Resource],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/browse?authorId=me",
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
            ".**.lastEdited" => "[last_edited]"
        }
    );

    Ok(())
}

#[sqlx::test]
async fn count(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::MetaKinds,
            Fixture::UserDefaultPerms,
            Fixture::Resource,
        ],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/resource/count", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[sqlx::test]
async fn update_and_publish(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::MetaKinds,
            Fixture::User,
            Fixture::Resource,
            Fixture::CategoryOrdering,
        ],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resource_id = "d8067526-1518-11ed-87fa-ebaf880b6d9c".to_string();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}/draft",
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
        }
    );

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}",
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
            "http://0.0.0.0:{}/v1/resource/{resource_id}/draft",
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
        }
    );

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}/live",
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
        }
    );

    let _resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}/draft/publish",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}/live",
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
            ".**.publishedAt" => "[published_at]"
        }
    );

    Ok(())
}

#[ignore]
#[sqlx::test]
async fn update_and_publish_incomplete_modules(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::MetaKinds,
            Fixture::User,
            Fixture::Resource,
            Fixture::CategoryOrdering,
        ],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    // Test no modules on JIG returns HTTP 400
    let resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/resource/3a71522a-cd77-11eb-8dc1-af3e35f7c743/draft/publish",
            port
        ))
        .login()
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    // Test no modules on JIG returns HTTP 400
    let resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/resource/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft/publish",
            port
        ))
        .login()
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    Ok(())
}

#[actix_rt::test]
async fn live_up_to_date_flag() -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::MetaKinds,
            Fixture::User,
            Fixture::Resource,
            Fixture::CategoryOrdering,
        ],
        &[],
    )
    .await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resource_id = "d8067526-1518-11ed-87fa-ebaf880b6d9c".to_string();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}/draft",
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
        }
    );

    let _resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}/draft/publish",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}/live",
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
        }
    );

    app.stop(false).await;

    Ok(())
}
