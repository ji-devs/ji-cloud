use super::ApiEndpoint;
use crate::domain::billing::{
    CreateSetupIntentPath, CreateSetupIntentRequest, SubscriptionPlansResponse,
};
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
    type Path = CreateSubscriptionPath;
    type Req = CreateSubscriptionRequest;
    type Res = Option<CreateSubscriptionResponse>;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Create a setup intent so that a customer can add a payment method
pub struct CreateSetupIntent;
impl ApiEndpoint for CreateSetupIntent {
    type Path = CreateSetupIntentPath;
    type Req = CreateSetupIntentRequest;
    type Res = String;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Get available subscription plans
pub struct GetSubscriptionPlans;
impl ApiEndpoint for GetSubscriptionPlans {
    type Path = SubscriptionPlanPath;
    type Req = ();
    type Res = SubscriptionPlansResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}
