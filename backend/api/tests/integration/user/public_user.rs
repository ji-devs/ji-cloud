use http::StatusCode;
use macros::test_service;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::User",
        "Fixture::Image",
        "Fixture::PublicUser",
        "Fixture::Circle"
    )
)]
async fn browse_public_user(port: u16) -> anyhow::Result<()> {
    let name = "browse_public_user";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/public/browse", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}", name), body);

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::User",
        "Fixture::Image",
        "Fixture::PublicUser",
        "Fixture::Circle"
    )
)]
async fn browse_users_with_circles(port: u16) -> anyhow::Result<()> {
    let name = "browse_users_with_circles";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/user/public/browse?circles=829606d0-f185-11ec-b9e4-5fadfd7252f6",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}", name), body);

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::User",
        "Fixture::MetaKinds",
        "Fixture::Jig",
        "Fixture::PublicUser"
    )
)]
async fn browse_user_jigs(port: u16) -> anyhow::Result<()> {
    let name = "browse_user_jigs";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/user/{}/public/jig/browse",
            port, "1f241e1b-b537-493f-a230-075cb16315be"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}", name),
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
    fixtures(
        "Fixture::User",
        "Fixture::MetaKinds",
        "Fixture::Jig",
        "Fixture::PublicUser"
    )
)]
async fn browse_user_resources(port: u16) -> anyhow::Result<()> {
    let name = "browse_user_resources";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/user/{}/public/resource/browse",
            port, "1f241e1b-b537-493f-a230-075cb16315be"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}", name), body);

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::User",
        "Fixture::MetaKinds",
        "Fixture::Jig",
        "Fixture::PublicUser",
        "Fixture::Playlist"
    )
)]
async fn browse_user_playlists(port: u16) -> anyhow::Result<()> {
    let name = "browse_user_playlists";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/user/{}/public/playlist/browse",
            port, "1f241e1b-b537-493f-a230-075cb16315be"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-1", name),
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/user/{}/public/playlist/browse",
            port, "7b96a41c-e406-11eb-8176-efd86dd7f444"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-2", name),
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::User",
        "Fixture::MetaKinds",
        "Fixture::Jig",
        "Fixture::PublicUser",
        "Fixture::Playlist"
    )
)]
async fn browse_follower_and_unfollow(port: u16) -> anyhow::Result<()> {
    let name = "browse_follower_and_unfollow";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/user/{}/public/follower/browse",
            port, "7b96a41c-e406-11eb-8176-efd86dd7f444"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-1", name), body);

    let resp = client
        .delete(&format!(
            "http://0.0.0.0:{}/v1/user/{}/unfollow",
            port, "7b96a41c-e406-11eb-8176-efd86dd7f444"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/user/{}/public/follower/browse",
            port, "7b96a41c-e406-11eb-8176-efd86dd7f444"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-2", name), body);

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::PublicUser")
)]
async fn browse_follower_and_follow(port: u16) -> anyhow::Result<()> {
    let name = "browse_follower_and_follow";

    let client = reqwest::Client::new();

    let resp = client
        .delete(&format!(
            "http://0.0.0.0:{}/v1/user/{}/unfollow",
            port, "7b96a41c-e406-11eb-8176-efd86dd7f444"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/user/{}/public/follower/browse",
            port, "7b96a41c-e406-11eb-8176-efd86dd7f444"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-1", name), body);

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/user/{}/follow",
            port, "7b96a41c-e406-11eb-8176-efd86dd7f444"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/user/{}/public/follower/browse",
            port, "7b96a41c-e406-11eb-8176-efd86dd7f444"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-2", name), body);

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::User",
        "Fixture::MetaKinds",
        "Fixture::Jig",
        "Fixture::PublicUser",
        "Fixture::Playlist"
    )
)]
async fn browse_following_and_unfollow(port: u16) -> anyhow::Result<()> {
    let name = "browse_following_and_unfollow";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/user/{}/public/following/browse",
            port, "1f241e1b-b537-493f-a230-075cb16315be"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-1", name), body);

    let resp = client
        .delete(&format!(
            "http://0.0.0.0:{}/v1/user/{}/unfollow",
            port, "7b96a41c-e406-11eb-8176-efd86dd7f444"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/user/{}/public/following/browse",
            port, "1f241e1b-b537-493f-a230-075cb16315be"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-2", name), body);

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::MetaKinds", "Fixture::PublicUser")
)]
async fn follow_self_error(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    // Also, the current user logged in
    let follower_id = "1f241e1b-b537-493f-a230-075cb16315be".to_string();

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/user/{follower_id}/follow",
            port
        ))
        .login()
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    Ok(())
}
