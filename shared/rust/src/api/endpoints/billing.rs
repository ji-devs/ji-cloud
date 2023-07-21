use super::ApiEndpoint;
use crate::domain::billing::{CreateSetupIntentPath, CreateSetupIntentRequest, SubscriptionPath};
use crate::{
    api::Method,
    domain::billing::{
        CreateSubscriptionPath, CreateSubscriptionRequest, CreateSubscriptionResponse,
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

/// Cancel a subscription
pub struct CancelSubscription;
impl ApiEndpoint for CancelSubscription {
    type Path = SubscriptionPath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
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
