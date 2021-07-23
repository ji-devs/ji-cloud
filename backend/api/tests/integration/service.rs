use config::RemoteTarget;
use core::env::req_env;
use core::settings::EmailClientSettings;
use ji_cloud_api::s3;
use ji_cloud_api::service::{mail, storage};

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)] // temp
pub enum Service {
    S3,
    GoogleCloudStorage,
    Email,
    Algolia,
}

// FIXME: make this more generic for all services, once GCS migration and test coverage is included
// if the given key is false, then bypass the test so CI can
pub fn email_test_guard() -> bool {
    let _ = dotenv::dotenv().ok();
    !core::env::env_bool("TEST_SENDGRID_DISABLE")
}

/// Holds settings related to external services, in test context only
pub struct TestServicesSettings {
    oauth2_token: Option<String>,
    project_id: String,
}

impl TestServicesSettings {
    const TEST_SENDGRID_API_KEY: &'static str = "TEST_SENDGRID_API_KEY";
    const TEST_SENDER_EMAIL: &'static str = "TEST_SENDER_EMAIL";
    const TEST_SIGNUP_VERIFY_TEMPLATE: &'static str = "TEST_SIGNUP_VERIFY_TEMPLATE";
    const TEST_SIGNUP_PASSWORD_RESET_TEMPLATE: &'static str = "TEST_SIGNUP_PASSWORD_RESET_TEMPLATE";

    pub async fn new() -> anyhow::Result<Self> {
        let (token, project_id) = match req_env("GOOGLE_CLOUD_SERVICE_ACCOUNT_JSON_KEY_SANDBOX") {
            Ok(key_json) => {
                let credentials =
                    match serde_json::from_str::<yup_oauth2::ServiceAccountKey>(&key_json) {
                        Ok(v) => v,
                        Err(e) => {
                            return Err(anyhow::anyhow!(
                                "Could not parse service account json key {:?}",
                                e
                            ));
                        }
                    };

                let project_id = credentials
                    .project_id
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("Couldn't find project_id"))?;

                let token = core::google::get_google_token_from_credentials(credentials).await?;

                (token.as_str().to_owned(), project_id)
            }
            _ => {
                log::info!("Falling back to json file for google cloud auth");
                let (token, project_id) = core::google::get_access_token_and_project_id(
                    RemoteTarget::Local.google_credentials_env_name(),
                )
                .await?;

                (token, project_id)
            }
        };

        Ok(TestServicesSettings {
            oauth2_token: Some(token),
            project_id,
        })
    }

    // TODO use a hashset for the services. Short array + algebraic type
    pub async fn init_services(
        &self,
        services: &[Service],
    ) -> (
        Option<mail::Client>,
        Option<s3::Client>,
        Option<storage::Client>,
        Option<ji_cloud_api::algolia::Client>,
    ) {
        let mail = match services.iter().any(|&it| it == Service::Email) {
            true => {
                log::info!("adrenochrome");
                self.create_test_mail_client().await
            }
            false => None,
        };

        // todo once s3, algolia, storage tests are ready

        (mail, None, None, None)
    }

    pub async fn create_test_mail_client(&self) -> Option<mail::Client> {
        log::info!("Starting email client");

        let api_key = self.get_gcp_secret(Self::TEST_SENDGRID_API_KEY).await.ok();

        let sender_email = self.get_gcp_secret(Self::TEST_SENDER_EMAIL).await.ok();

        let signup_verify_template = self
            .get_gcp_secret(Self::TEST_SIGNUP_VERIFY_TEMPLATE)
            .await
            .ok();

        let password_reset_template = self
            .get_gcp_secret(Self::TEST_SIGNUP_PASSWORD_RESET_TEMPLATE)
            .await
            .ok();

        let (api_key, sender_email) = match (api_key, sender_email) {
            (Some(api_key), Some(sender_email)) => (api_key, sender_email),
            _ => return None,
        };

        let settings = EmailClientSettings {
            api_key,
            sender_email,
            signup_verify_template,
            password_reset_template,
        };

        let client = mail::Client::new(settings);

        log::info!("Test mail client created");

        Some(client)
    }

    #[deprecated]
    #[allow(dead_code)]
    fn read_test_secret_from_env(&self, secret: &str) -> Option<String> {
        match std::env::var(secret) {
            Ok(secret) => Some(secret),
            Err(_) => None,
        }
    }

    async fn get_gcp_secret(&self, secret_name: &str) -> anyhow::Result<String> {
        core::google::get_secret(
            self.oauth2_token.as_ref().unwrap(),
            &*self.project_id,
            secret_name,
        )
        .await
    }

    // TODO: this
    // fn create_test_gcs_client(&self) -> Option<ji_cloud_api::google::storage::Client> {
    //
    // }
}
