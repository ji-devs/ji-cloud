use shared::config::RemoteTarget;

use core::{env::req_env, settings::EmailClientSettings};
use ji_cloud_api::service::{mail, s3, storage};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
    const TEST_EMAIL_INFO_ADDRESS: &'static str = "TEST_JIGZI_INFO_EMAIL";
    const TEST_SIGNUP_VERIFY_TEMPLATE: &'static str = "TEST_SIGNUP_VERIFY_TEMPLATE";
    const TEST_SIGNUP_PASSWORD_RESET_TEMPLATE: &'static str = "TEST_SIGNUP_PASSWORD_RESET_TEMPLATE";
    const TEST_EMAIL_RESET_TEMPLATE: &'static str = "TEST_EMAIL_RESET_TEMPLATE";
    const TEST_WELCOME_JIGZI_TEMPLATE: &'static str = "TEST_WELCOME_JIGZI_TEMPLATE";

    pub async fn new() -> anyhow::Result<Self> {
        let (token, project_id) = match req_env("TEST_SERVICE_ACCOUNT_JSON") {
            Ok(key_json) => {
                let credentials = yup_oauth2::read_service_account_key(&key_json)
                    .await
                    .map_err(|e| {
                        anyhow::anyhow!(
                            "Could not parse service account json key {:?}. Len: {:?}",
                            e,
                            &key_json.len()
                        )
                    })?;

                let project_id = credentials
                    .project_id
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("Couldn't find project_id"))?;

                let token = core::google::get_google_token_from_credentials(credentials).await?;

                (token.as_str().to_owned(), project_id)
            }
            _ => {
                log::info!("Falling back to json file for google cloud auth");
                let (token, project_id) = core::google::get_access_token_response_and_project_id(
                    RemoteTarget::Local.google_credentials_env_name(),
                )
                .await?;

                let token = token.access_token.unwrap();

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
        println!("init_services");
        let services: std::collections::HashSet<&Service> = services.into_iter().collect();

        let mail = match services.contains(&Service::Email) {
            true => self.create_test_mail_client().await,
            false => None,
        };

        let gcs = match services.contains(&Service::GoogleCloudStorage) {
            true => self.create_test_gcs_client(),
            false => None,
        };

        // todo: other clients

        (mail, None, gcs, None)
    }

    pub async fn create_test_mail_client(&self) -> Option<mail::Client> {
        println!("create_test_mail_client");
        let google_api_key = self
            .get_gcp_managed_secret(Self::TEST_SENDGRID_API_KEY)
            .await
            .ok();

        let env_api_key = req_env("TEST_SENDGRID_API_KEY").ok();

        let api_key = match google_api_key {
            Some(google_api_key) => Some(google_api_key),
            None => env_api_key,
        };

        let google_sender_email = self
            .get_gcp_managed_secret(Self::TEST_SENDER_EMAIL)
            .await
            .ok();

        let google_jigzi_info_email = self
            .get_gcp_managed_secret(Self::TEST_EMAIL_INFO_ADDRESS)
            .await
            .ok();

        let env_sender_email = req_env("TEST_SENDER_EMAIL").ok();

        let sender_email = match google_sender_email {
            Some(google_sender_email) => Some(google_sender_email),
            None => env_sender_email,
        };

        let env_jigzi_info_email = req_env("TEST_JIGZI_INFO_EMAIL").ok();

        let jigzi_info_email = match google_jigzi_info_email {
            Some(google_jigzi_info_email) => Some(google_jigzi_info_email),
            None => env_jigzi_info_email,
        };

        let google_signup_verify_template = self
            .get_gcp_managed_secret(Self::TEST_SIGNUP_VERIFY_TEMPLATE)
            .await
            .ok();

        let env_signup_verify_template = req_env("TEST_SIGNUP_VERIFY_TEMPLATE").ok();

        let signup_verify_template = match google_signup_verify_template {
            Some(google_signup_verify_template) => Some(google_signup_verify_template),
            None => env_signup_verify_template,
        };

        let google_password_reset_template = self
            .get_gcp_managed_secret(Self::TEST_SIGNUP_PASSWORD_RESET_TEMPLATE)
            .await
            .ok();

        let env_password_reset_template = req_env("TEST_PASSWORD_RESET_TEMPLATE").ok();

        let password_reset_template = match google_password_reset_template {
            Some(google_password_reset_template) => Some(google_password_reset_template),
            None => env_password_reset_template,
        };

        let google_email_reset_template = self
            .get_gcp_managed_secret(Self::TEST_EMAIL_RESET_TEMPLATE)
            .await
            .ok();

        let env_email_reset_template = req_env("TEST_EMAIL_RESET_TEMPLATE").ok();

        let google_welcome_jigzi_template = self
            .get_gcp_managed_secret(Self::TEST_WELCOME_JIGZI_TEMPLATE)
            .await
            .ok();

        let email_reset_template = match google_email_reset_template {
            Some(google_email_reset_template) => Some(google_email_reset_template),
            None => env_email_reset_template,
        };

        let env_welcome_jigzi_template = req_env("TEST_WELCOME_JIGZI_TEMPLATE").ok();

        let welcome_jigzi_template = match google_welcome_jigzi_template {
            Some(google_welcome_jigzi_template) => Some(google_welcome_jigzi_template),
            None => env_welcome_jigzi_template,
        };

        println!(
            "end keys {:?} {:?} {:?} {:?} {:?}",
            api_key,
            sender_email,
            signup_verify_template,
            password_reset_template,
            welcome_jigzi_template
        );

        let (api_key, sender_email, jigzi_info_email) =
            match (api_key, sender_email, jigzi_info_email) {
                (Some(api_key), Some(sender_email), Some(jigzi_info_email)) => {
                    (api_key, sender_email, jigzi_info_email)
                }
                _ => return None,
            };

        let settings = EmailClientSettings {
            api_key,
            sender_email,
            jigzi_info_email,
            signup_verify_template,
            password_reset_template,
            email_reset_template,
            welcome_jigzi_template,
        };

        let client = mail::Client::new(settings);

        log::info!("Test mail client created successfully");

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

    async fn get_gcp_managed_secret(&self, secret_name: &str) -> anyhow::Result<String> {
        core::google::get_secret(
            self.oauth2_token.as_ref().unwrap(),
            &*self.project_id,
            secret_name,
        )
        .await
    }

    fn create_test_gcs_client(&self) -> Option<storage::Client> {
        return None;

        // let settings = GoogleCloudStorageSettings {
        //     oauth2_token: "redacted".to_owned(),
        //     processing_bucket: RemoteTarget::Local
        //         .s3_processing_bucket()
        //         .unwrap()
        //         .to_owned(),
        //     media_bucket: RemoteTarget::Local.s3_bucket().unwrap().to_owned(),
        // };
        //
        // let client = storage::Client::new(settings);
        //
        // log::info!("Test GCS client created successfully");
        //
        // client.ok()
    }
}
