//! Types for billing

use chrono::{DateTime, Utc};
use macros::make_path_parts;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use strum_macros::{Display, EnumString};

#[cfg(feature = "backend")]
use anyhow::anyhow;
use serde_json::Value;

use crate::api::endpoints::PathPart;
use crate::domain::image::ImageId;
use crate::domain::user::UserProfile;

/// Stripe customer ID
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::Type), sqlx(transparent))]
pub struct CustomerId(String);

#[cfg(feature = "backend")]
impl From<stripe::CustomerId> for CustomerId {
    fn from(value: stripe::CustomerId) -> Self {
        Self(value.as_str().to_owned())
    }
}

#[cfg(feature = "backend")]
impl From<CustomerId> for stripe::CustomerId {
    fn from(value: CustomerId) -> Self {
        use std::str::FromStr;
        Self::from_str(&value.0).unwrap()
    }
}

impl CustomerId {
    /// Obtain a reference to the inner string
    #[cfg(feature = "backend")]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stripe payment method ID
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StripePaymentMethodId(String);

/// Last 4 digits of a card number
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Last4(String);

/// Payment network associated with a [Card]
#[derive(Debug, Serialize, Deserialize, Clone, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "lowercase")]
pub enum PaymentNetwork {
    /// Visa
    Visa,
    /// Mastercard
    Mastercard,
    /// Discover Global Network
    Discover,
    /// JCB Co
    JCB,
    /// American Express
    #[strum(serialize = "amex")]
    AmericanExpress,
    /// UnionPay
    UnionPay,
    /// Diners
    #[strum(serialize = "diners")]
    DinersClub,
    /// Unknown
    Unknown,
}

impl Default for PaymentNetwork {
    fn default() -> Self {
        Self::Unknown
    }
}

/// A display-only representation of a card
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    /// The last 4 digits of the card
    pub last4: Last4,
    /// The cards payment network
    pub payment_network: PaymentNetwork,
    /// The expiry month for this card
    pub exp_month: u8,
    /// The expiry year for this card
    pub exp_year: u16,
}

#[cfg(feature = "backend")]
impl From<stripe::CardDetails> for Card {
    fn from(value: stripe::CardDetails) -> Self {
        use std::str::FromStr;
        Self {
            last4: Last4(value.last4),
            payment_network: PaymentNetwork::from_str(&value.brand).unwrap_or_default(),
            exp_month: value.exp_month as u8,
            exp_year: value.exp_year as u16,
        }
    }
}

/// Type of payment method
///
/// Note: Only the [PaymentMethodType::Card] variant has any display details.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentMethodType {
    /// Apple Pay
    ApplePay,
    /// Google Pay
    GooglePay,
    /// [Link](https://stripe.com/docs/payments/link) one-click checkout
    Link,
    /// Card
    Card(Card),
    /// Other/unknown
    Other,
}

wrap_uuid! {
    /// Local payment method ID
    pub struct PaymentMethodId
}

/// Payment method
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentMethod {
    /// The Stripe payment method ID
    pub stripe_payment_method_id: StripePaymentMethodId, // Stripe payment method ID
    /// The type of payment method
    pub payment_method_type: PaymentMethodType,
}

#[cfg(feature = "backend")]
impl From<stripe::PaymentMethod> for PaymentMethod {
    fn from(value: stripe::PaymentMethod) -> Self {
        let payment_method_type = if value.link.is_some() {
            PaymentMethodType::Link
        } else if let Some(card) = value.card {
            if let Some(wallet) = card.wallet {
                if wallet.apple_pay.is_some() {
                    PaymentMethodType::ApplePay
                } else if wallet.google_pay.is_some() {
                    PaymentMethodType::GooglePay
                } else {
                    PaymentMethodType::Other
                }
            } else {
                PaymentMethodType::Card(Card::from(card))
            }
        } else {
            PaymentMethodType::Other
        };

        Self {
            stripe_payment_method_id: StripePaymentMethodId(value.id.as_str().to_string()),
            payment_method_type,
        }
    }
}

/// The tier a subscription is on. This would apply to any [`SubscriptionType`]
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum SubscriptionTier {
    /// Basic
    Basic = 0,
    /// Pro
    Pro = 1,
}

/// Stripe subscription ID
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::Type), sqlx(transparent))]
pub struct StripeSubscriptionId(String);

#[cfg(feature = "backend")]
impl From<stripe::SubscriptionId> for StripeSubscriptionId {
    fn from(value: stripe::SubscriptionId) -> Self {
        Self(value.as_str().to_owned())
    }
}

/// Stripe invoice ID
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::Type), sqlx(transparent))]
pub struct StripeInvoiceId(String);

impl StripeInvoiceId {
    /// Returns a copy of the inner value
    pub fn inner(&self) -> String {
        self.0.clone()
    }
}

#[cfg(feature = "backend")]
impl From<&stripe::InvoiceId> for StripeInvoiceId {
    fn from(value: &stripe::InvoiceId) -> Self {
        Self(value.as_str().to_owned())
    }
}

/// Stripe product ID
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::Type), sqlx(transparent))]
pub struct StripeProductId(String);

/// Stripe price ID
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::Type), sqlx(transparent))]
pub struct StripePriceId(String);

impl From<StripePriceId> for String {
    fn from(value: StripePriceId) -> Self {
        value.0
    }
}

/// The subscriptions billing interval
#[derive(Debug, Display, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum BillingInterval {
    /// Subscription is billed monthly
    Monthly = 0,
    /// Subscription is billed yearly
    Annually = 1,
}

/// Status of a subscription
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum SubscriptionStatus {
    /// The subscription has been created, awaiting finalization from Stripe or paused.
    Inactive = 0,
    /// The subscription is active, i.e. not cancelled or expired.
    Active = 1,
    /// The subscription is cancelled but still active, i.e. not expired.
    Canceled = 2,
    /// The subscription is expired.
    Expired = 3,
}

impl SubscriptionStatus {
    /// Whether the subscription is still valid so that a teacher is able to make use of subscription
    /// features.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        matches!(self, Self::Active | Self::Canceled)
    }
}

#[cfg(feature = "backend")]
impl Default for SubscriptionStatus {
    fn default() -> Self {
        Self::Inactive
    }
}

#[cfg(feature = "backend")]
impl From<stripe::SubscriptionStatus> for SubscriptionStatus {
    fn from(value: stripe::SubscriptionStatus) -> Self {
        match value {
            stripe::SubscriptionStatus::Incomplete | stripe::SubscriptionStatus::Paused => {
                Self::Inactive
            }
            stripe::SubscriptionStatus::Active
            | stripe::SubscriptionStatus::PastDue
            | stripe::SubscriptionStatus::Trialing
            | stripe::SubscriptionStatus::Unpaid => Self::Active,
            stripe::SubscriptionStatus::Canceled => Self::Canceled,
            stripe::SubscriptionStatus::IncompleteExpired => Self::Expired,
        }
    }
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
    /// Account ID to associate this subscription with.
    pub account_id: AccountId,
    /// ID of the latest unpaid invoice generated for this subscription
    pub latest_invoice_id: Option<StripeInvoiceId>,
    /// Amount due if any
    pub amount_due_in_cents: Option<AmountInCents>,
    /// When the subscription was originally created.
    pub created_at: DateTime<Utc>,
    /// When the subscription was last updated.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Data used to create a new subscription record
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[cfg(feature = "backend")]
pub struct CreateSubscriptionRecord {
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
    /// Account ID to associate this subscription with
    /// User ID to associate this subscription with
    pub account_id: AccountId,
    /// ID of the latest unpaid invoice generated for this subscription
    pub latest_invoice_id: Option<StripeInvoiceId>,
    /// Amount due if any
    pub amount_due_in_cents: Option<AmountInCents>,
}

/// Data used to update a new subscription record
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[cfg(feature = "backend")]
pub struct UpdateSubscriptionRecord {
    /// The Stripe subscription ID
    pub stripe_subscription_id: StripeSubscriptionId,
    /// Whether the subscription auto-renews
    pub auto_renew: Option<bool>,
    /// The subscription status
    pub status: Option<SubscriptionStatus>,
    /// When the subscriptions current period ends/expires
    pub current_period_end: Option<DateTime<Utc>>,
    /// ID of the latest unpaid invoice generated for this subscription
    pub latest_invoice_id: Option<StripeInvoiceId>,
}

#[cfg(feature = "backend")]
impl UpdateSubscriptionRecord {
    /// Create a new instance with just the stripe subscription ID set
    pub fn new(stripe_subscription_id: StripeSubscriptionId) -> Self {
        Self {
            stripe_subscription_id,
            auto_renew: None,
            status: None,
            current_period_end: None,
            latest_invoice_id: None,
        }
    }
}

#[cfg(feature = "backend")]
impl TryFrom<stripe::Subscription> for UpdateSubscriptionRecord {
    type Error = anyhow::Error;

    fn try_from(value: stripe::Subscription) -> Result<Self, Self::Error> {
        use chrono::TimeZone;

        let latest_invoice_id = value
            .latest_invoice
            .as_ref()
            .map(|invoice| StripeInvoiceId::from(&invoice.id()));

        Ok(Self {
            stripe_subscription_id: value.id.into(),
            auto_renew: None, // TODO need to impl this
            // This is weird.
            status: Some(if value.ended_at.is_some() {
                SubscriptionStatus::Expired
            } else if value.canceled_at.is_some() {
                SubscriptionStatus::Canceled
            } else {
                SubscriptionStatus::from(value.status)
            }),
            current_period_end: Some(
                Utc.timestamp_opt(value.current_period_end, 0)
                    .latest()
                    .ok_or(anyhow::anyhow!("Invalid timestamp"))?,
            ),
            latest_invoice_id,
        })
    }
}

/// The limit of how many accounts can be associated with the subscription. [None] means unlimited.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "backend", derive(sqlx::Type), sqlx(transparent))]
pub struct AccountLimit(i64);

impl From<i64> for AccountLimit {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

/// The type of subscription
#[derive(Debug, Display, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum SubscriptionType {
    /// An individual subscription
    Individual = 0,
    /// A school subscription
    School = 1,
}

/// The type of account
#[derive(Debug, Display, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum AccountType {
    /// An individual account
    Individual = 0,
    /// A school account
    School = 1,
}

impl AccountType {
    /// Whether this account type has a dedicated admin user
    pub fn has_admin(&self) -> bool {
        match self {
            Self::School => true,
            _ => false,
        }
    }

    /// Test whether this variant matches the variant in SubscriptionType
    pub fn matches_subscription_type(&self, subscription_type: &SubscriptionType) -> bool {
        match self {
            Self::Individual => matches!(subscription_type, SubscriptionType::Individual),
            Self::School => matches!(subscription_type, SubscriptionType::School),
        }
    }
}

impl From<SubscriptionType> for AccountType {
    fn from(value: SubscriptionType) -> Self {
        match value {
            SubscriptionType::Individual => Self::Individual,
            SubscriptionType::School => Self::School,
        }
    }
}

/// Stripe invoice number
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::Type), sqlx(transparent))]
pub struct InvoiceNumber(String);

/// Represents an amount in cents
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::Type), sqlx(transparent))]
pub struct AmountInCents(i64);

impl AmountInCents {
    /// Create a new instance
    pub fn new(amount: i64) -> Self {
        Self(amount)
    }

    /// Returns a copy of the inner value
    pub fn inner(&self) -> i64 {
        self.0
    }
}

/// Represents a trial period length
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::Type), sqlx(transparent))]
pub struct TrialPeriod(i64);

impl TrialPeriod {
    /// Create a new instance
    pub fn new(length: i64) -> Self {
        Self(length)
    }

    /// Returns a copy of the inner value
    pub fn inner(&self) -> i64 {
        self.0
    }
}

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
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
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
    pub account_limit: Option<AccountLimit>,
    /// Current price of subscription in cents
    pub amount_in_cents: AmountInCents,
    /// Trial period, if any
    pub trial_period: Option<TrialPeriod>,
    /// When the plan was originally created.
    pub created_at: DateTime<Utc>,
    /// When the plan was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Request to create or update a subscription plan
///
/// In Stripe this would correspond to a Price within a Product.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct CreateUpdateSubscriptionPlanRequest {
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
    pub account_limit: Option<AccountLimit>,
    /// Current price of subscription in cents
    pub amount_in_cents: AmountInCents,
    /// Trial period, if any
    pub trial_period: Option<TrialPeriod>,
}

make_path_parts!(SubscriptionPlanPath => "/v1/plans");

/// Mapped into plan details
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscriptionPlanDetailsResponse {
    /// Local subscription plan ID
    pub plan_id: PlanId,
    /// Amount in cents to subscribe to this plan
    pub amount_in_cents: AmountInCents,
    /// Trial period
    pub trial_period: Option<TrialPeriod>,
    /// Billing interval
    pub billing_interval: BillingInterval,
    /// Subscription type
    pub subscription_type: SubscriptionType,
    /// Subscription tier
    pub subscription_tier: SubscriptionTier,
}

impl From<SubscriptionPlan> for SubscriptionPlanDetailsResponse {
    fn from(plan: SubscriptionPlan) -> Self {
        Self {
            plan_id: plan.plan_id,
            amount_in_cents: plan.amount_in_cents,
            trial_period: plan.trial_period,
            billing_interval: plan.billing_interval,
            subscription_type: plan.subscription_type,
            subscription_tier: plan.subscription_tier,
        }
    }
}

/// Mapped individual plans
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndividualPlanResponse {
    /// Basic plan monthly
    pub basic_monthly: PlanId,
    /// Basic plan annual
    pub basic_annual: PlanId,
    /// Pro plan monthly
    pub pro_monthly: PlanId,
    /// Pro plan annual
    pub pro_annual: PlanId,
}

/// Mapped school plans
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchoolPlanResponse {
    /// Annual school plans with an account limit
    pub limited_annual: HashMap<AccountLimit, PlanId>,
    /// Annual school plan without a limit
    pub unlimited_annual: PlanId,
}

/// Subscription plans mapped into a response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscriptionPlansResponse {
    /// Plans
    pub plans: HashMap<PlanId, SubscriptionPlanDetailsResponse>,
    /// Individual plan lookups
    pub individual: IndividualPlanResponse,
    /// School plan lookups
    pub school: SchoolPlanResponse,
}

#[cfg(feature = "backend")]
impl TryFrom<Vec<SubscriptionPlan>> for SubscriptionPlansResponse {
    type Error = anyhow::Error;

    fn try_from(plans: Vec<SubscriptionPlan>) -> Result<Self, Self::Error> {
        let mut builder = SubscriptionPlansResponseBuilder::default();
        for plan in plans {
            builder.set_from_plan(plan);
        }

        builder.build()
    }
}

/// Allows to easily build a subscription plans response from database records
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[cfg(feature = "backend")]
struct SubscriptionPlansResponseBuilder {
    /// Plans map
    plans: HashMap<PlanId, SubscriptionPlanDetailsResponse>,
    /// Individual basic monthly
    individual_basic_monthly: Option<PlanId>,
    /// Individual basic annual
    individual_basic_annual: Option<PlanId>,
    /// Individual pro monthly
    individual_pro_monthly: Option<PlanId>,
    /// Individual pro annual
    individual_pro_annual: Option<PlanId>,
    /// Annual school plans with account limits
    school_limited_annual: HashMap<AccountLimit, PlanId>,
    /// Annual school plan without an account limit
    school_unlimited_annual: Option<PlanId>,
}

#[cfg(feature = "backend")]
impl SubscriptionPlansResponseBuilder {
    /// Set the appropriate plan from a subscription plan record
    fn set_from_plan(&mut self, plan: SubscriptionPlan) {
        let subscription_type = plan.subscription_type;
        let subscription_tier = plan.subscription_tier;
        let billing_interval = plan.billing_interval;
        let account_limit = plan.account_limit;

        let response = SubscriptionPlanDetailsResponse::from(plan);
        let plan_id = response.plan_id;
        self.plans.insert(plan_id, response);

        match subscription_type {
            SubscriptionType::Individual => match (subscription_tier, billing_interval) {
                (SubscriptionTier::Basic, BillingInterval::Monthly) => {
                    self.individual_basic_monthly = Some(plan_id);
                }
                (SubscriptionTier::Basic, BillingInterval::Annually) => {
                    self.individual_basic_annual = Some(plan_id);
                }
                (SubscriptionTier::Pro, BillingInterval::Monthly) => {
                    self.individual_pro_monthly = Some(plan_id);
                }
                (SubscriptionTier::Pro, BillingInterval::Annually) => {
                    self.individual_pro_annual = Some(plan_id);
                }
            },
            SubscriptionType::School => match (account_limit, billing_interval) {
                (None, BillingInterval::Annually) => {
                    self.school_unlimited_annual = Some(plan_id);
                }
                (Some(account_limit), BillingInterval::Annually) => {
                    self.school_limited_annual.insert(account_limit, plan_id);
                }
                _ => {
                    // There are no monthly plans for schools. If the data exists, we can safely
                    // ignore it.
                }
            },
        }
    }

    /// Build the subscription plans response.
    ///
    /// Will return an error if:
    /// - Either or both of the individual plans are missing;
    /// - The unlimited school plan is missing;
    /// - Or, there are no limited school plans.
    fn build(self) -> anyhow::Result<SubscriptionPlansResponse> {
        if self.school_limited_annual.is_empty() {
            return Err(anyhow!("Missing limited school plans"));
        }

        Ok(SubscriptionPlansResponse {
            plans: self.plans,
            individual: IndividualPlanResponse {
                basic_monthly: self
                    .individual_basic_monthly
                    .ok_or(anyhow!("Missing monthly Individual Basic plan"))?,
                basic_annual: self
                    .individual_basic_annual
                    .ok_or(anyhow!("Missing annual Individual Basic plan"))?,
                pro_monthly: self
                    .individual_pro_monthly
                    .ok_or(anyhow!("Missing monthly Individual Pro plan"))?,
                pro_annual: self
                    .individual_pro_annual
                    .ok_or(anyhow!("Missing annual Individual Pro plan"))?,
            },
            school: SchoolPlanResponse {
                limited_annual: self.school_limited_annual,
                unlimited_annual: self
                    .school_unlimited_annual
                    .ok_or(anyhow!("Missing annual School Unlimited plan"))?,
            },
        })
    }
}

/// Request to create a subscription.
///
/// If no payment method information is passed with, then the system will attempt to use the
/// users existing payment method. Otherwise, a payment method will be saved.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSubscriptionRequest {
    /// Optional setup intent ID if a payment method was created prior to subscribing.
    pub setup_intent_id: Option<String>,
    /// Plan ID to create the subscription for
    pub plan_id: PlanId,
    /// Promotion code
    pub promotion_code: Option<String>,
}

make_path_parts!(CreateSubscriptionPath => "/v1/subscribe");

/// Create subscription response.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSubscriptionResponse {
    /// The *Stripe* subscription ID
    pub subscription_id: StripeSubscriptionId,
    /// The client secret from Stripe for reference when adding a payment method
    ///
    /// `None` indicates that the subscription was created without requiring a new payment method to
    /// be added.
    pub client_secret: String,
}

/// Request to create a subscription.
///
/// If no payment method information is passed with, then the system will attempt to use the
/// users existing payment method. Otherwise, a payment method will be saved.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSetupIntentRequest {
    /// Plan ID to create the subscription for
    pub plan_id: PlanId,
}

make_path_parts!(CreateSetupIntentPath => "/v1/billing/payment-method");

wrap_uuid! {
    /// Account ID
    pub struct AccountId
}

/// A billing account
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    /// Account ID
    pub account_id: AccountId,
    /// The type of account
    pub account_type: AccountType,
    /// The customer ID on stripe
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_customer_id: Option<CustomerId>,
    /// Stripe payment method, if any
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethod>,
    /// _Current_ subscription if any
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Subscription>,
    /// When the account was created.
    pub created_at: DateTime<Utc>,
    /// When the account was last updated.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Summary of the user's account. This could be a school account that a user is a member of.
///
/// In the case that the user is a member of a school account, the subscription tier would be
/// `None` for a free account, or `Pro`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAccountSummary {
    /// ID of the school if this is a School account
    pub school_id: Option<SchoolId>,
    /// The subscription tier the user is on
    pub subscription_tier: Option<SubscriptionTier>,
    /// Status of the accounts subscription, if any
    pub subscription_status: Option<SubscriptionStatus>,
    /// Whether this user is an admin. For non School accounts, this user will
    /// always be an admin
    pub is_admin: bool,
    /// Whether the account is overdue
    pub overdue: bool,
    /// Whether the user is verified for the account
    pub verified: bool,
}

wrap_uuid! {
    /// Wrapper type around [`Uuid`], represents the ID of a School.
    pub struct SchoolId
}

/// A school profile.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct School {
    /// The school's id.
    pub id: SchoolId,

    /// Name of the school
    pub school_name: SchoolName,

    /// The school's location
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Value>,

    /// The school's email address
    pub email: String,

    /// Description for school
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// ID to the school's profile image in the user image library.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<ImageId>,

    /// Website for the school
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,

    /// Organization type
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_type: Option<String>,

    /// The school's account ID
    pub account_id: AccountId,

    /// When the school was created.
    pub created_at: DateTime<Utc>,

    /// When the school was last updated.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// A user associated with an account
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountUser {
    /// The associated user
    pub user: UserProfile,
    /// The subscription tier the user is on
    pub subscription_tier: Option<SubscriptionTier>,
    /// Whether this user is an admin. For non School accounts, this user will
    /// always be an admin
    pub is_admin: bool,
    /// Whether the user is verified for the account
    pub verified: bool,
}

wrap_uuid! {
    /// Wrapper type around [`Uuid`], represents the ID of a School Name.
    pub struct SchoolNameId
}

make_path_parts!(SchoolNamePath => "/v1/school-name");

/// A known school name
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchoolName {
    /// The id of a school name
    pub id: SchoolNameId,
    /// The school name
    pub name: String,
    /// Whether the school name has been verified
    #[serde(default)]
    pub verified: bool,
}

/// Representation of a school name value
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct SchoolNameValue(String);

impl std::fmt::Display for SchoolNameValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<SchoolNameValue> for String {
    fn from(value: SchoolNameValue) -> Self {
        value.0
    }
}

impl From<String> for SchoolNameValue {
    fn from(value: String) -> Self {
        SchoolNameValue(value)
    }
}

impl AsRef<str> for SchoolNameValue {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Whether the user is creating a new school name or chosen an existing name that we know about
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SchoolNameRequest {
    /// Attempt to create a new name
    Value(SchoolNameValue),
    /// Use an existing name
    Id(SchoolNameId),
}

make_path_parts!(CreateSchoolAccountPath => "/v1/school");

/// Request to create a new school account
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSchoolAccountRequest {
    /// School name
    pub name: SchoolNameRequest,
    /// School location
    pub location: Value,
}

make_path_parts!(SchoolAccountPath => "/v1/school/{}" => SchoolId);

/// Request to create a new school account
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetSchoolAccountResponse {
    /// School name
    pub school: School,
    /// Account associated with the school
    pub account: AccountIfAuthorized,
    /// School location
    pub users: Vec<AccountUser>,
}

/// A school account only if the user requesting the account is a system admin or an account admin.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::large_enum_variant)]
#[serde(untagged)]
pub enum AccountIfAuthorized {
    /// The user is authorized
    Authorized(Account),
    /// The user is not authorized
    Unauthorized,
}

/// Request to update a school profile.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct UpdateSchoolAccountRequest {
    /// The school's location
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Value>,

    /// The school's email address
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Description for school
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// ID to the school's profile image in the user image library.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<ImageId>,

    /// Website for the school
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,

    /// Organization type
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_type: Option<String>,
}

make_path_parts!(UpdateSchoolNamePath => "/v1/school/{}/school-name" => SchoolId);
