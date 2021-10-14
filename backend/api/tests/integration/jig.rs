use http::StatusCode;
use serde_json::json;
use shared::domain::{jig::JigId, CreateResponse};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

mod additional_resource;
mod cover;
mod module;
mod player;

#[actix_rt::test]
async fn create_default() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User], &[]).await;

    let port = app.port();

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

    app.stop(false).await;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.id" => "[id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]"
        }
    );

    Ok(())
}

// requires algolia
// #[actix_rt::test]
// async fn delete() -> anyhow::Result<()> {
//     let app = initialize_server(&[Fixture::User, Fixture::Jig]).await;

//     let port = app.port();

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

#[actix_rt::test]
async fn create_with_params() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

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

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".id" => "[id]", ".last_edited" => "[last_edited]"});

    Ok(())
}

#[actix_rt::test]
async fn clone() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

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

    app.stop(false).await;

    Ok(())
}

#[actix_rt::test]
async fn get() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

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

    app.stop(false).await;

    Ok(())
}

// todo: test-exhaustiveness: create a `JigBrowse` Fixture, actually test the cases (paging, jig count, etc)
#[actix_rt::test]
async fn browse_simple() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/browse", port))
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

// todo: test-exhaustiveness: create a `JigBrowse` Fixture, actually test the cases (paging, jig count, etc)
#[actix_rt::test]
async fn browse_own_simple() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::UserDefaultPerms, Fixture::Jig], &[]).await;

    let port = app.port();

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
async fn count() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::UserDefaultPerms, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/count", port))
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
async fn update_and_publish() -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::User, Fixture::Jig, Fixture::CategoryOrdering],
        &[],
    )
    .await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/3a71522a-cd77-11eb-8dc1-af3e35f7c743/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/jig/3a71522a-cd77-11eb-8dc1-af3e35f7c743",
            port
        ))
        .json(&json!({
            "description": "asdasdasd",
            "language": "en-us",
            "categories":["7fe19326-e883-11ea-93f0-5343493c17c4", "81c4796a-e883-11ea-93f0-df2484ab6b11"]
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/3a71522a-cd77-11eb-8dc1-af3e35f7c743/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, {".**.lastEdited" => "[timestamp]"});

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/3a71522a-cd77-11eb-8dc1-af3e35f7c743/live",
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
            ".**.feedbackNegative" => "[audio]"
        }
    );

    let _resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/jig/3a71522a-cd77-11eb-8dc1-af3e35f7c743/draft/publish",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/3a71522a-cd77-11eb-8dc1-af3e35f7c743/live",
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
            ".**.publishedAt" => "[published_at]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]"
        }
    );

    app.stop(false).await;

    Ok(())
}
