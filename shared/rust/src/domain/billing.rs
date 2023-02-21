//! Types for billing

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{api::endpoints::PathPart, domain::user::UserId};

/// Stripe customer ID
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerId(String);

/// Stripe payment method ID
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StripePaymentMethodId(String);

/// Last 4 digits of a card number
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Last4(String);

/// Payment network associated with a [Card]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentNetwork {
    /// Visa
    Visa,
    /// Mastercard
    Mastercard,
    /// Discover Global Network
    Discover,
    /// American Express
    AmericanExpress,
}

/// Status of the payment method
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentMethodStatus {
    /// Payment method is active
    Active,
    /// Payment method has expired
    Expired,
}

/// A display-only representation of a card
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    /// The last 4 digits of the card
    pub last_4: Last4,
    /// The cards payment network
    pub payment_network: PaymentNetwork,
    /// The cards current status
    pub status: PaymentMethodStatus,
}

// TODO what details do I need for the other types?
/// Type of payment method
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentMethodType {
    // ApplePay(..),
    // GooglePay(..),
    // Link(..),
    /// Card
    Card(Card),
}

wrap_uuid! {
    /// Local payment method ID
    pub struct PaymentMethodId
}

/// Payment method
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentMethod {
    /// Local payment method ID
    pub payment_method_id: PaymentMethodId,
    /// The Stripe payment method ID
    pub stripe_payment_method_id: StripePaymentMethodId, // Stripe payment method ID
    /// The type of payment method
    pub payment_method_type: PaymentMethodType,
}

/// The tier a subscription is on. This would apply to any [SubscriptionType]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SubscriptionTier {
    /// Basic
    Basic,
    /// Pro
    Pro,
}

/// Stripe subscription ID
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StripeSubscriptionId(String);

/// Stripe product ID
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StripeProductId(String);

/// Stripe price ID
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StripePriceId(String);

/// The subscriptions billing interval
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BillingInterval {
    /// Subscription is billed monthly
    Monthly,
    /// Subscription is billed yearly
    Annually,
}

/// Status of a subscription
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SubscriptionStatus {
    /// The subscription is active, i.e. not cancelled or expired.
    Active,
    /// The subscription is cancelled but still active, i.e. not expired.
    Cancelled,
    /// The subscription is expired.
    Expired,
}

wrap_uuid! {
    /// Local subscription ID
    pub struct SubscriptionId
}

/// An existing subscription for a customer
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subscription {
    /// The local subscription ID
    pub subscription_id: SubscriptionId,
    /// The Stripe subscription ID
    pub stripe_subscription_id: StripeSubscriptionId,
    /// The subscription plan ID
    pub subscription_plan_id: PlanId,
    /// The subscription tier
    pub tier: SubscriptionTier,
    /// Whether the subscription auto-renews
    pub auto_renew: bool,
    /// The subscription status
    pub status: SubscriptionStatus,
    /// When the subscriptions current period ends/expires
    pub current_period_end: DateTime<Utc>,
    /// The teacher who is the administrator of this subscription
    ///
    /// For [SubscriptionType::Individual] subscriptions, this would always be the user that created
    /// the subscription.
    ///
    /// For [SubscriptionType::School] subscriptions, this would be the user that initially created
    /// the subscription. It should be possible to transfer ownership of the subscription.
    pub admin: UserId,
    // TODO do we need other fields?
}

/// The limit of how many accounts can be associated with the subscription. [None] means unlimited.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountLimit(Option<usize>);

/// The type of subscription
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SubscriptionType {
    /// An individual subscription
    Individual,
    /// A school subscription
    School,
}

/// Stripe invoice number
pub struct InvoiceNumber(String);

/// Represents a value amount in cents
pub struct AmountInCents(usize);

wrap_uuid! {
    /// Local charge ID
    pub struct ChargeId
}

/// A charge to a customer for a subscription
// TODO this may need to be updated
pub struct Charge {
    /// Local ID of the charge
    pub charge_id: ChargeId,
    /// Timestamp of charge
    pub charged_at: DateTime<Utc>,
    /// Subscription tier at the time of charge
    pub subscription_tier: SubscriptionTier,
    /// Payment method used at the time of charge
    pub payment_method: PaymentMethod,
    /// Stripe invoice number
    pub invoice_number: InvoiceNumber,
    /// Amount charged in cents
    pub amount_in_cents: AmountInCents,
}

wrap_uuid! {
    /// Local subscription plan ID
    pub struct PlanId
}

/// A subscription plan
///
/// In Stripe this would correspond to a Price within a Product.
pub struct SubscriptionPlan {
    /// Local ID of the subscription plan
    pub plan_id: PlanId,
    /// Stripe product ID
    pub product_id: StripeProductId,
    /// Stripe price ID
    pub price_id: StripePriceId,
    /// Subscription tier
    pub subscription_tier: SubscriptionTier,
    /// Subscription plan type
    pub subscription_type: SubscriptionType,
    /// Billing interval
    pub billing_interval: BillingInterval,
    /// The account limit for this subscription
    ///
    /// For [SubscriptionType::Individual] subscriptions, this will _always_ be `Some(1)`.
    pub account_limit: AccountLimit,
    /// Current price of subscription in cents
    pub amount_in_cents: AmountInCents,
}

// TODO Add stripe billing fields to UserProfile
// /// The user's customer ID
// #[serde(default)]
// #[serde(skip_serializing_if = "Option::is_none")]
// pub stripe_customer_id: Option<CustomerId>,

// /// The user's current payment method
// #[serde(default)]
// #[serde(skip_serializing_if = "Option::is_none")]
// pub payment_method: Option<PaymentMethod>,

// /// The users current subscription
// #[serde(default)]
// #[serde(skip_serializing_if = "Option::is_none")]
// pub subscription: Option<Subscription>,
