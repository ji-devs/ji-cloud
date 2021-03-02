use fixture::Fixture;
use http::StatusCode;

mod helpers;

mod fixture {
    #[derive(Debug, Copy, Clone)]
    pub enum Fixture {
        User,
        MetaKinds,
        CategoryOrdering,
        CategoryNesting,
        Image,
        UserNoPerms,
    }

    impl Fixture {
        pub const fn as_query(self) -> &'static str {
            match self {
                Self::User => include_str!("../../fixtures/1_user.sql"),
                Self::MetaKinds => include_str!("../../fixtures/2_meta_kinds.sql"),
                Self::CategoryOrdering => {
                    include_str!("../../fixtures/3_category_ordering.sql")
                }
                Self::CategoryNesting => include_str!("../../fixtures/4_category_nesting.sql"),
                Self::Image => include_str!("../../fixtures/5_image.sql"),
                Self::UserNoPerms => include_str!("../../fixtures/6_user_no_perms.sql"),
            }
        }
    }
}

#[actix_rt::test]
async fn pass() -> anyhow::Result<()> {
    let app = helpers::initialize_server(&[]).await;

    let port = app.port();

    let _ = tokio::spawn(app.run_until_stopped());

    let resp = reqwest::get(&format!("http://0.0.0.0:{}", port)).await?;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    Ok(())
}

#[actix_rt::test]
async fn login_missing_auth() -> anyhow::Result<()> {
    let app = helpers::initialize_server(&[]).await;

    let port = app.port();

    let _ = tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/session", port))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    Ok(())
}

#[actix_rt::test]
async fn login_user_basic() -> anyhow::Result<()> {
    let app = helpers::initialize_server(&[Fixture::User]).await;

    let port = app.port();

    let _ = tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/session", port))
        .basic_auth("test@test.test", Some("password1"))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;
    body.as_object()
        .expect("body wasn't a object")
        .contains_key("csrf");

    Ok(())
}
