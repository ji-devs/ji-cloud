use super::ApiEndpoint;
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
    type Req = CreateSubscriptionRequest;
    type Res = Option<CreateSubscriptionResponse>;
    type Path = CreateSubscriptionPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}
