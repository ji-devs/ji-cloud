use ji_core::settings::RuntimeSettings;
use shared::error::{ServiceError, ServiceKindError};
use stripe::Client;
use tracing::instrument;

#[instrument(skip_all)]
pub fn create_stripe_client(settings: &RuntimeSettings) -> Result<Client, ServiceError> {
    let secret = settings
        .stripe_secret_key
        .as_ref()
        .ok_or(ServiceError::DisabledService(ServiceKindError::Stripe))?;

    Ok(Client::new(secret))
}
