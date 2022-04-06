use core::settings::EmailClientSettings;
use sendgrid::v3::{Content, Email, Message, Personalization, SGMap, Sender};
use shared::domain::{jig::report::JigReportEmail, session::OAuthProvider};
use tracing::instrument;

use crate::error;

use super::Service;

const SENDER_NAME: &str = "Jigzi";

pub struct Client {
    client: Sender,

    sender_email: Email,

    signup_verify_template: Option<String>,

    password_reset_template: Option<String>,

    email_reset_template: Option<String>,
}

impl Client {
    pub fn new(settings: EmailClientSettings) -> Self {
        Client {
            client: Sender::new(settings.api_key),
            sender_email: Email::new(settings.sender_email).set_name(SENDER_NAME),
            signup_verify_template: settings.signup_verify_template,
            password_reset_template: settings.password_reset_template,
            email_reset_template: settings.email_reset_template,
        }
    }

    #[instrument(skip_all)]
    pub async fn send_signup_verify(
        &self,
        template: SignupVerifyTemplate<'_>,
        to: Email,
        link: String,
    ) -> anyhow::Result<()> {
        let mut template_data = SGMap::new();
        template_data.insert("url".to_string(), link);

        let message = Message::new(self.sender_email.clone())
            .set_template_id(&template.0)
            .add_personalization(Personalization::new(to).add_dynamic_template_data(template_data));

        self.client.send(&message).await?;

        Ok(())
    }

    #[instrument(skip_all)]
    pub async fn send_password_reset(
        &self,
        template: PasswordResetTemplate<'_>,
        to: Email,
        link: String,
        first_name: String,
    ) -> anyhow::Result<()> {
        let mut template_data = SGMap::new();
        template_data.insert("url".to_string(), link);
        template_data.insert("firstname".to_string(), first_name);

        let message = Message::new(self.sender_email.clone())
            .set_template_id(&template.0)
            .add_personalization(Personalization::new(to).add_dynamic_template_data(template_data));

        self.client.send(&message).await?;

        Ok(())
    }

    #[instrument(skip_all)]
    pub async fn send_oauth_password_reset(
        &self,
        to: Email,
        oauth_provider: OAuthProvider,
    ) -> anyhow::Result<()> {
        let subject = format!("Reset your password");

        let value = format!(
            r#" 
Looks like you requested a reset password link but you didn't sign up with a password, you signed up with a {} account. 
Please try logging in with your {} account.
            "#,
            oauth_provider.as_str(),
            oauth_provider.as_str()
        );
        let message = Message::new(self.sender_email.clone())
            .add_personalization(Personalization::new(to))
            .set_subject(&subject)
            .add_content(
                Content::new()
                    .set_content_type("text/plain")
                    .set_value(value),
            );

        self.client.send(&message).await?;

        Ok(())
    }

    pub async fn send_email_reset(
        &self,
        template: EmailResetTemplate<'_>,
        to: Email,
        link: String,
        first_name: String,
    ) -> anyhow::Result<()> {
        let mut template_data = SGMap::new();
        template_data.insert("url".to_string(), link);
        template_data.insert("firstname".to_string(), first_name);

        let message = Message::new(self.sender_email.clone())
            .set_template_id(&template.0)
            .add_personalization(Personalization::new(to).add_dynamic_template_data(template_data));

        self.client.send(&message).await?;

        Ok(())
    }

    pub async fn send_report_email(
        &self,
        to: Email,
        report: JigReportEmail,
        link: String,
    ) -> anyhow::Result<()> {
        let subject = format!("URGENT: JIG Report '{}'", report.report_type.as_str());

        let (reporter_email, reporter_name): (String, String) =
            if let (Some(email), Some(name)) = (report.reporter_email, report.reporter_name) {
                (email, name)
            } else {
                ("Unknown".to_string(), "Unknown".to_string())
            };

        let value = format!(
            r#"{} with email {} has reported "{}" for the following reason: "{}".

            URL: {}
            Created by: {}
               "#,
            reporter_name,
            reporter_email,
            report.display_name,
            report.report_type.as_str(),
            link,
            report.creator_name,
        );

        let content = Content::new();

        let message = Message::new(self.sender_email.clone())
            .add_personalization(Personalization::new(to))
            .set_subject(&subject)
            .add_content(content.set_content_type("text/plain").set_value(value));

        self.client.send(&message).await?;

        Ok(())
    }

    pub fn signup_verify_template(&self) -> Result<SignupVerifyTemplate<'_>, error::ServiceKind> {
        // todo: make the error more specific?
        self.signup_verify_template
            .as_deref()
            .map(SignupVerifyTemplate)
            .ok_or(error::ServiceKind::Mail)
    }

    pub fn password_reset_template(&self) -> Result<PasswordResetTemplate<'_>, error::ServiceKind> {
        // todo: make the error more specific?
        self.password_reset_template
            .as_deref()
            .map(PasswordResetTemplate)
            .ok_or(error::ServiceKind::Mail)
    }

    pub fn email_reset_template(&self) -> Result<EmailResetTemplate<'_>, error::ServiceKind> {
        // todo: make the error more specific?
        self.email_reset_template
            .as_deref()
            .map(EmailResetTemplate)
            .ok_or(error::ServiceKind::Mail)
    }
}

impl Service for Client {
    const DISABLED_ERROR: error::ServiceKind = error::ServiceKind::Mail;
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct SignupVerifyTemplate<'a>(&'a str);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PasswordResetTemplate<'a>(&'a str);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct EmailResetTemplate<'a>(&'a str);
