use http::StatusCode;
use macros::test_service;
use serde_json::json;
use shared::domain::{resource::ResourceId, CreateResponse};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};

mod cover;
mod curation;

#[test_service(setup = "setup_service", fixtures("Fixture::User"))]
async fn create_default(port: u16) -> anyhow::Result<()> {
    let name = "create_default";

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/resource", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let status = resp.status();

    assert_eq!(status, StatusCode::CREATED);

    let body: CreateResponse<ResourceId> = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-1", name), body, {".**.id" => "[id]"});

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
        format!("{}-2",name), 
        body, {
            ".**.id" => "[id]",
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]"});

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
        format!("{}-3",name), 
        body, {
            ".**.id" => "[id]",
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]"});

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Resource")
)]
async fn create_with_params(port: u16) -> anyhow::Result<()> {
    let name = "create_with_params";

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
    let status = resp.status();

    assert_eq!(status, StatusCode::CREATED);

    let body: CreateResponse<ResourceId> = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}",name), 
        body, {
            ".**.id" => "[id]",
            ".**.lastEdited" => "[last_edited]"});

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Resource")
)]
async fn clone(port: u16) -> anyhow::Result<()> {
    let name = "clone";

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

    let status = resp.status();

    assert_eq!(status, StatusCode::CREATED);

    let CreateResponse { id: ResourceId(id) } = resp.json().await?;

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/resource/{}/draft", port, id))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let status = resp.status();

    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-1",name),
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
    let status = resp.status();
    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-2",name),
        body, {
            ".**.id" => "[id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.additionalResources" => "[ids]"
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Resource")
)]
async fn get(port: u16) -> anyhow::Result<()> {
    let name = "get";

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
    let status = resp.status();

    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-1",name),
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
    let status = resp.status();

    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-2",name),
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}

// todo: test-exhaustiveness: create a `ResourceBrowse` Fixture, actually test the cases (paging, resource count, etc)
#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Resource")
)]
async fn browse_simple(port: u16) -> anyhow::Result<()> {
    let name = "browse_simple";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/resource/browse", port))
        .login()
        .send()
        .await?
        .error_for_status()?;
    let status = resp.status();

    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-1",name),
        body, {
            ".**.lastEdited" => "[last_edited]",
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
    let status = resp.status();

    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-2",name),
        body, {
            ".**.lastEdited" => "[last_edited]",
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
    let status = resp.status();

    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-3",name),
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Resource")
)]
async fn browse_order_by(port: u16) -> anyhow::Result<()> {
    let name = "browse_order_by";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/resource/browse", port))
        .login()
        .send()
        .await?
        .error_for_status()?;
    let status = resp.status();

    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-1", name), body);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/browse?orderBy=createdAt",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;
    let status = resp.status();

    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-2", name), body);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/browse?orderBy=publishedAt",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;
    let status = resp.status();

    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-3", name), body);

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Resource")
)]
async fn browse_own_simple(port: u16) -> anyhow::Result<()> {
    let name = "browse_own_simple";

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

    let status = resp.status();

    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}",name),
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::UserDefaultPerms", "Fixture::Resource")
)]
async fn count(port: u16) -> anyhow::Result<()> {
    let name = "count";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/resource/count", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let status = resp.status();

    assert_eq!(status, StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}", name), body);

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::MetaKinds",
        "Fixture::UserDefaultPerms",
        "Fixture::Resource",
        "Fixture::CategoryOrdering"
    )
)]
async fn update_and_publish(port: u16) -> anyhow::Result<()> {
    let name = "update_and_publish";

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
        format!("{}-1",name),
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
    let status = resp.status();

    assert_eq!(status, StatusCode::NO_CONTENT);

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
        format!("{}-2",name),
        body, {
            ".**.lastEdited" => "[last_edited]",
            ".**.publishedAt" => "[published_at]",
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
        format!("{}-3",name),
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
        format!("{}-4",name),
        body, {
            ".**.id" => "[id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.publishedAt" => "[published_at]",
            ".**.additionalResources" => "[ids]",

        }
    );

    Ok(())
}

// #[ignore]
// #[test_service(
//     setup = "setup_service",
//     fixtures(
//         "Fixture::MetaKinds",
//         "Fixture::UserDefaultPerms",
//         "Fixture::Resource",
//         "Fixture::CategoryOrdering"
//     )
// )]
// async fn publish_incomplete_modules(port: u16) -> anyhow::Result<()> {
//     let client = reqwest::Client::new();

//     // Test no modules on JIG returns HTTP 400
//     let resp = client
//         .put(&format!(
//             "http://0.0.0.0:{}/v1/resource/{}/draft/publish",
//             port, "2f8d91d0-1519-11ed-87fa-eb1826fcf343"
//         ))
//         .login()
//         .send()
//         .await?;
//     let status = resp.status();

//     assert_eq!(status, StatusCode::BAD_REQUEST);

//     // Test no modules on JIG returns HTTP 400
//     let resp = client
//         .put(&format!(
//             "http://0.0.0.0:{}/v1/resource/{}/draft/publish",
//             port, "af827e00-1519-11ed-87fa-7b1aa26c85a8"
//         ))
//         .login()
//         .send()
//         .await?;
//     let status = resp.status();

//     assert_eq!(status, StatusCode::BAD_REQUEST);

//     Ok(())
// }

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::MetaKinds",
        "Fixture::UserDefaultPerms",
        "Fixture::Resource",
        "Fixture::CategoryOrdering"
    )
)]
async fn live_up_to_date_flag(port: u16) -> anyhow::Result<()> {
    let name = "live_up_to_date_flag";

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

    println!("First");

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-1",name),
        body, {
            ".**.publishedAt" => "[published_at]",
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
        format!("{}-2",name),
        body, {
            ".**.id" => "[id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.publishedAt" => "[published_at]",
            ".**.additionalResources" => "[ids]"
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::MetaKinds",
        "Fixture::UserDefaultPerms",
        "Fixture::Resource",
        "Fixture::CategoryOrdering"
    )
)]
async fn update_privacy_level(port: u16) -> anyhow::Result<()> {
    let name = "update_privacy_level";

    let client = reqwest::Client::new();

    let resource_id = "d8067526-1518-11ed-87fa-ebaf880b6d9c".to_string();

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}",
            port
        ))
        .json(&json!({
            "privacyLevel": "private",
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
        format!("{}-1",name),
        body, {
            ".**.id" => "[id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.publishedAt" => "[published_at]",
            ".**.additionalResources" => "[ids]"
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
        format!("{}-2",name),
        body, {
            ".**.id" => "[id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.publishedAt" => "[published_at]",
            ".**.additionalResources" => "[ids]"
        }
    );

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}",
            port
        ))
        .json(&json!({
            "privacyLevel": "unlisted",
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

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
        format!("{}-3",name),
        body, {
            ".**.id" => "[id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.publishedAt" => "[published_at]",
            ".**.additionalResources" => "[ids]"
        }
    );

    Ok(())
}
