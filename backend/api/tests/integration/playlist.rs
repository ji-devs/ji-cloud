use http::StatusCode;
use macros::test_service;
use serde_json::json;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::MetaKinds",
        "Fixture::User",
        "Fixture::Jig",
        "Fixture::Playlist"
    )
)]
async fn get(port: u16) -> anyhow::Result<()> {
    let name = "get";

    let client = reqwest::Client::new();

    let playlist_id = "3a6a3660-f3ec-11ec-b8ef-071747fa2a0d".to_string();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/playlist/{}/draft",
            port, playlist_id
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-1",name),
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/playlist/{}/live",
            port, playlist_id
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-2",name),
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::MetaKinds",
        "Fixture::User",
        "Fixture::Jig",
        "Fixture::Playlist"
    )
)]
async fn update_and_publish_browse(port: u16) -> anyhow::Result<()> {
    let name = "update_and_publish_browse";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/playlist/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d/draft",
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
            "http://0.0.0.0:{}/v1/playlist/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d",
            port
        ))
        .json(&json!({
            "description": "asdasdasd",
            "language": "en-us",
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/playlist/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d/draft",
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
        }
    );

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/playlist/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d/live",
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
            "http://0.0.0.0:{}/v1/playlist/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d/draft/publish",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/playlist/browse?authorId=1f241e1b-b537-493f-a230-075cb16315be&draftOrLive=draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-4",name),
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

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::MetaKinds",
        "Fixture::User",
        "Fixture::Jig",
        "Fixture::Playlist"
    )
)]
async fn browse_simple(port: u16) -> anyhow::Result<()> {
    let name = "browse_simple";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/playlist/browse", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

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
    fixtures(
        "Fixture::MetaKinds",
        "Fixture::User",
        "Fixture::Jig",
        "Fixture::Playlist"
    )
)]
async fn playlist_jig_index(port: u16) -> anyhow::Result<()> {
    let name = "playlist_jig_index";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/playlist/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d/draft",
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
            "http://0.0.0.0:{}/v1/playlist/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d",
            port
        ))
        .json(&json!({
            "items": ["0cc084bc-7c83-11eb-9f77-e3218dffb008", "19becb2b-bff7-4c1b-bb2c-16f2e098d3d3", "3a71522a-cd77-11eb-8dc1-af3e35f7c743"]
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/playlist/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d/draft",
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
        }
    );

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/playlist/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d/live",
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
            "http://0.0.0.0:{}/v1/playlist/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d/draft/publish",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/playlist/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d/live",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-4",name),
        body, {
            ".**.id" => "[id]",
            ".**.stable_id" => "[stable_id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.publishedAt" => "[published_at]"
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::MetaKinds",
        "Fixture::User",
        "Fixture::Jig",
        "Fixture::Playlist"
    )
)]
async fn publish_modules(port: u16) -> anyhow::Result<()> {
    let name = "publish_modules";

    let client = reqwest::Client::new();

    let playlist_id = "3a6a3660-f3ec-11ec-b8ef-071747fa2a0d".to_string();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/playlist/{}/draft",
            port, playlist_id
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
        .get(&format!(
            "http://0.0.0.0:{}/v1/playlist/{}/live",
            port, playlist_id
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
        }
    );

    let _resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/playlist/{}/draft/publish",
            port, playlist_id
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/playlist/{}/live",
            port, playlist_id
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-3",name),
        body, {
            ".**.id" => "[id]",
            ".**.stable_id" => "[stable_id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.publishedAt" => "[published_at]"
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::MetaKinds",
        "Fixture::User",
        "Fixture::Jig",
        "Fixture::Playlist"
    )
)]
async fn live_up_to_date_flag(port: u16) -> anyhow::Result<()> {
    let name = "live_up_to_date_flag";

    let client = reqwest::Client::new();

    let playlist_id = "3a6a3660-f3ec-11ec-b8ef-071747fa2a0d".to_string();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/playlist/{playlist_id}/draft",
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

    let _resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/playlist/{playlist_id}/draft/publish",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/playlist/{playlist_id}/live",
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
            // Really just need to redact the module ID because it is recreated for the live data,
            // but I couldn't get a selector working correctly... So redacting all IDs.
            ".**.id" => "[id]",
            ".**.stable_id" => "[stable_id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.publishedAt" => "[published_at]",
        }
    );

    Ok(())
}
