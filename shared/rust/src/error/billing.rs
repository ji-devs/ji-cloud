use crate::domain::billing::{AccountType, PlanType, SubscriptionType};
use crate::error::service::ServiceError;
use crate::error::TransientError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "backend")]
use actix_web::{body::BoxBody, HttpResponse, ResponseError};
#[cfg(feature = "backend")]
use stripe::StripeError;

#[allow(missing_docs)]
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum BillingError {
    #[cfg_attr(feature = "backend", error(transparent))]
    #[cfg_attr(not(feature = "backend"), error("Internal server error"))]
    InternalServerError(
        #[serde(skip)]
        #[from]
        TransientError<anyhow::Error>,
    ),
    #[error(transparent)]
    Service(ServiceError),
    #[cfg_attr(feature = "backend", error(transparent))]
    #[cfg_attr(not(feature = "backend"), error("Stripe error"))]
    Stripe(
        #[cfg(feature = "backend")]
        #[serde(skip)]
        #[from]
        TransientError<StripeError>,
    ),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Missing Stripe signature")]
    MissingStripeSignature,
    #[error("Invalid Setup Intent ID")]
    InvalidSetupIntentId,
    #[error("No active subscription for user")]
    NoActiveSubscription,
    #[error("No canceled subscription for user")]
    NoCanceledSubscription,
    #[error("Account has an existing subscription")]
    SubscriptionExists,
    #[error("School not found")]
    SchoolNotFound,
    #[error("Incorrect plan type. Expected {expected}, found {found}.")]
    IncorrectPlanType {
        expected: AccountType,
        found: SubscriptionType,
    },
    #[error("Invalid promotion code {0}")]
    InvalidPromotionCode(String),
    #[error("Forbidden")]
    Forbidden,
    #[error("Cannot upgrade to {upgrade_to} from {upgrade_from}")]
    InvalidUpgradePlanType {
        upgrade_to: PlanType,
        upgrade_from: PlanType,
    },
}

#[cfg(feature = "backend")]
impl From<StripeError> for BillingError {
    fn from(value: StripeError) -> Self {
        Self::from(TransientError::from(value))
    }
}

impl From<ServiceError> for BillingError {
    fn from(err: ServiceError) -> Self {
        Self::Service(err)
    }
}

impl From<anyhow::Error> for BillingError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e.into())
    }
}

#[cfg(feature = "backend")]
impl ResponseError for BillingError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            Self::InternalServerError { .. } | Self::Stripe { .. } => {
                http::StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Service(service) => service.status_code(),
            Self::NotFound(_) | Self::SchoolNotFound => http::StatusCode::NOT_FOUND,
            Self::Forbidden => http::StatusCode::FORBIDDEN,
            _ => http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }
}
