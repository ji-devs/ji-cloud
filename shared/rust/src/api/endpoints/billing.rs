use super::ApiEndpoint;
use crate::domain::billing::{CreateSetupIntentPath, SubscriptionPlansResponse};
use crate::{
    api::Method,
    domain::billing::{
        CreateSubscriptionPath, CreateSubscriptionRequest, CreateSubscriptionResponse,
        SubscriptionPlanPath,
    },
    error::EmptyError,
};

/// Create a subscription and store payment info if provided
pub struct CreateSubscription;
impl ApiEndpoint for CreateSubscription {
    type Req = CreateSubscriptionRequest;
    type Res = Option<CreateSubscriptionResponse>;
    type Path = CreateSubscriptionPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Create a setup intent so that a customer can add a payment method
pub struct CreateSetupIntent;
impl ApiEndpoint for CreateSetupIntent {
    type Req = ();
    type Res = String;
    type Path = CreateSetupIntentPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Get available subscription plans
pub struct GetSubscriptionPlans;
impl ApiEndpoint for GetSubscriptionPlans {
    type Req = ();
    type Res = SubscriptionPlansResponse;
    type Path = SubscriptionPlanPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}
