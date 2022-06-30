use http::StatusCode;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[actix_rt::test]
async fn browse_public_user() -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::Image,
            Fixture::PublicUser,
            Fixture::Circle,
        ],
        &[],
    )
    .await;

    let port = app.port();

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

    app.stop(false).await;

    Ok(())
}

#[actix_rt::test]
async fn browse_user_jigs() -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::MetaKinds,
            Fixture::Jig,
            Fixture::PublicUser,
        ],
        &[],
    )
    .await;

    let port = app.port();

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

    app.stop(false).await;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]"
        }
    );

    Ok(())
}

#[actix_rt::test]
async fn browse_user_resources() -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::MetaKinds,
            Fixture::Jig,
            Fixture::PublicUser,
        ],
        &[],
    )
    .await;

    let port = app.port();

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

    app.stop(false).await;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[actix_rt::test]
async fn browse_user_courses() -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::MetaKinds,
            Fixture::Jig,
            Fixture::PublicUser,
            Fixture::Course,
        ],
        &[],
    )
    .await;

    let port = app.port();

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

    app.stop(false).await;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}

#[actix_rt::test]
async fn browse_follower_and_unfollow() -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::MetaKinds,
            Fixture::Jig,
            Fixture::PublicUser,
            Fixture::Course,
        ],
        &[],
    )
    .await;

    let port = app.port();

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

    app.stop(false).await;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[actix_rt::test]
async fn browse_follower_and_follow() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::PublicUser], &[]).await;

    let port = app.port();

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

    app.stop(false).await;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[actix_rt::test]
async fn browse_following_and_unfollow() -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::MetaKinds,
            Fixture::Jig,
            Fixture::PublicUser,
            Fixture::Course,
        ],
        &[],
    )
    .await;

    let port = app.port();

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

    app.stop(false).await;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}
