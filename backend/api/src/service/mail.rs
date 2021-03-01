use core::settings::EmailClientSettings;

use sendgrid::v3::{Email, Message, Personalization, SGMap, Sender};

use crate::error;

use super::Service;

pub struct Client {
    client: Sender,

    sender_email: Email,

    signup_verify_template: Option<String>,

    password_reset_template: Option<String>,
}

impl Client {
    pub fn new(settings: EmailClientSettings) -> Self {
        Client {
            client: Sender::new(settings.api_key),
            sender_email: Email::new(settings.sender_email),
            signup_verify_template: settings.signup_verify_template,
            password_reset_template: settings.password_reset_template,
        }
    }

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

    pub async fn send_password_reset(
        &self,
        template: PasswordResetTemplate<'_>,
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
