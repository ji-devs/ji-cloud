use super::ApiEndpoint;
use crate::domain::billing::{
    AdminUpgradeSubscriptionPlanPath, AdminUpgradeSubscriptionPlanRequest,
    CreateCustomerPortalLinkPath, CreateSetupIntentPath, CreateSetupIntentRequest,
    SubscriptionCancellationStatusRequest, SubscriptionPauseRequest,
    UpdateSubscriptionCancellationPath, UpdateSubscriptionPausedPath, UpgradeSubscriptionPlanPath,
    UpgradeSubscriptionPlanRequest,
};
use crate::error::BillingError;
use crate::{
    api::Method,
    domain::billing::{
        CreateSubscriptionPath, CreateSubscriptionRequest, CreateSubscriptionResponse,
    },
};

/// Create a subscription and store payment info if provided
pub struct CreateSubscription;
impl ApiEndpoint for CreateSubscription {
    type Path = CreateSubscriptionPath;
    type Req = CreateSubscriptionRequest;
    type Res = Option<CreateSubscriptionResponse>;
    type Err = BillingError;
    const METHOD: Method = Method::Post;
}

/// Set the cancellation status of a subscription
pub struct UpdateSubscriptionCancellation;
impl ApiEndpoint for UpdateSubscriptionCancellation {
    type Path = UpdateSubscriptionCancellationPath;
    type Req = SubscriptionCancellationStatusRequest;
    type Res = ();
    type Err = BillingError;
    const METHOD: Method = Method::Patch;
}

/// Set whether the subscription is paused or not
pub struct UpdateSubscriptionPaused;
impl ApiEndpoint for UpdateSubscriptionPaused {
    type Path = UpdateSubscriptionPausedPath;
    type Req = SubscriptionPauseRequest;
    type Res = ();
    type Err = BillingError;
    const METHOD: Method = Method::Patch;
}

/// Set the cancellation status of a subscription
pub struct UpgradeSubscriptionPlan;
impl ApiEndpoint for UpgradeSubscriptionPlan {
    type Path = UpgradeSubscriptionPlanPath;
    type Req = UpgradeSubscriptionPlanRequest;
    type Res = ();
    type Err = BillingError;
    const METHOD: Method = Method::Post;
}

/// Set the cancellation status of a subscription
pub struct AdminUpgradeSubscriptionPlan;
impl ApiEndpoint for AdminUpgradeSubscriptionPlan {
    type Path = AdminUpgradeSubscriptionPlanPath;
    type Req = AdminUpgradeSubscriptionPlanRequest;
    type Res = ();
    type Err = BillingError;
    const METHOD: Method = Method::Post;
}

/// Create a setup intent so that a customer can add a payment method
pub struct CreateSetupIntent;
impl ApiEndpoint for CreateSetupIntent {
    type Path = CreateSetupIntentPath;
    type Req = CreateSetupIntentRequest;
    type Res = String;
    type Err = BillingError;
    const METHOD: Method = Method::Post;
}

/// Generate a link for a customer to view their Stripe customer portal
pub struct CreateCustomerPortalLink;
impl ApiEndpoint for CreateCustomerPortalLink {
    type Path = CreateCustomerPortalLinkPath;
    type Req = ();
    type Res = String;
    type Err = BillingError;
    const METHOD: Method = Method::Get;
}
