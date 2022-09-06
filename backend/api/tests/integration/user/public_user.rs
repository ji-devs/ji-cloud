use http::StatusCode;
use sqlx::PgPool;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[sqlx::test]
async fn browse_public_user(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::Image,
            Fixture::PublicUser,
            Fixture::Circle,
        ],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/public/browse", port))
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
async fn browse_users_with_circles(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::Image,
            Fixture::PublicUser,
            Fixture::Circle,
        ],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[sqlx::test]
async fn browse_user_jigs(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::MetaKinds,
            Fixture::Jig,
            Fixture::PublicUser,
        ],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]"
        }
    );

    Ok(())
}

#[sqlx::test]
async fn browse_user_resources(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::MetaKinds,
            Fixture::Jig,
            Fixture::PublicUser,
        ],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[sqlx::test]
async fn browse_user_courses(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::MetaKinds,
            Fixture::Jig,
            Fixture::PublicUser,
            Fixture::Course,
        ],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/user/{}/public/course/browse",
            port, "1f241e1b-b537-493f-a230-075cb16315be"
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
        }
    );

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/user/{}/public/course/browse",
            port, "7b96a41c-e406-11eb-8176-efd86dd7f444"
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
        }
    );

    Ok(())
}

#[sqlx::test]
async fn browse_follower_and_unfollow(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::MetaKinds,
            Fixture::Jig,
            Fixture::PublicUser,
            Fixture::Course,
        ],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

    insta::assert_json_snapshot!(body);

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

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[sqlx::test]
async fn browse_follower_and_follow(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::PublicUser], &[], pool).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

    insta::assert_json_snapshot!(body);

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

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[sqlx::test]
async fn browse_following_and_unfollow(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::MetaKinds,
            Fixture::Jig,
            Fixture::PublicUser,
            Fixture::Course,
        ],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

    insta::assert_json_snapshot!(body);

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

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[sqlx::test]
async fn follow_self_error(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::User, Fixture::MetaKinds, Fixture::PublicUser],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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
