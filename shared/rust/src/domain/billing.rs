//! Types for billing

use chrono::{DateTime, Utc};
use macros::make_path_parts;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use strum_macros::{Display, EnumString};

use serde_json::Value;

use crate::api::endpoints::PathPart;
use crate::domain::image::ImageId;
use crate::domain::user::UserProfile;
use crate::domain::{UpdateNonNullable, UpdateNullable};

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
    pub struct PaymentMethodId("pm")
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
    pub struct SubscriptionId("sub")
}

/// An existing subscription for a customer
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subscription {
    /// The local subscription ID
    pub subscription_id: SubscriptionId,
    /// The Stripe subscription ID
    pub stripe_subscription_id: StripeSubscriptionId,
    /// The subscription type
    pub subscription_plan_type: PlanType,
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

/// Possible individual subscription plans
#[derive(
    Debug, Display, Serialize, Deserialize, Clone, Copy, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum PlanType {
    /// Basic level, monthly
    IndividualBasicMonthly = 0,
    /// Basic level, annually
    IndividualBasicAnnually = 1,
    /// Pro level, monthly
    IndividualProMonthly = 2,
    /// Pro level, annually
    IndividualProAnnually = 3,
    /// School Level 1
    SchoolLevel1 = 4,
    /// School Level 2
    SchoolLevel2 = 5,
    /// School Level 3
    SchoolLevel3 = 6,
    /// School Level 4
    SchoolLevel4 = 7,
    /// School Unlimited
    SchoolUnlimited = 8,
}

const ACCOUNT_LIMIT_L1: i64 = 4;
const ACCOUNT_LIMIT_L2: i64 = 10;
const ACCOUNT_LIMIT_L3: i64 = 20;
const ACCOUNT_LIMIT_L4: i64 = 30;

const INDIVIDUAL_TRIAL_PERIOD: i64 = 7;
const SCHOOL_TRIAL_PERIOD: i64 = 7;

impl PlanType {
    /// Represents the plan type as a `str`
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IndividualBasicMonthly => "individual-basic-monthly",
            Self::IndividualBasicAnnually => "individual-basic-annually",
            Self::IndividualProMonthly => "individual-pro-monthly",
            Self::IndividualProAnnually => "individual-pro-annually",
            Self::SchoolLevel1 => "school-level-1",
            Self::SchoolLevel2 => "school-level-2",
            Self::SchoolLevel3 => "school-level-3",
            Self::SchoolLevel4 => "school-level-4",
            Self::SchoolUnlimited => "school-unlimited",
        }
    }

    /// `SubscriptionTier` of the current plan
    #[must_use]
    pub const fn subscription_tier(&self) -> SubscriptionTier {
        match self {
            Self::IndividualBasicMonthly | Self::IndividualBasicAnnually => SubscriptionTier::Basic,
            _ => SubscriptionTier::Pro,
        }
    }

    /// Account limit of the current plan
    #[must_use]
    pub const fn account_limit(&self) -> Option<AccountLimit> {
        match self {
            Self::SchoolLevel1 => Some(AccountLimit(ACCOUNT_LIMIT_L1)),
            Self::SchoolLevel2 => Some(AccountLimit(ACCOUNT_LIMIT_L2)),
            Self::SchoolLevel3 => Some(AccountLimit(ACCOUNT_LIMIT_L3)),
            Self::SchoolLevel4 => Some(AccountLimit(ACCOUNT_LIMIT_L4)),
            Self::SchoolUnlimited => None,
            _ => Some(AccountLimit(1)),
        }
    }

    /// Subscription type of the current plant
    #[must_use]
    pub const fn subscription_type(&self) -> SubscriptionType {
        match self {
            Self::IndividualBasicMonthly
            | Self::IndividualBasicAnnually
            | Self::IndividualProMonthly
            | Self::IndividualProAnnually => SubscriptionType::Individual,
            _ => SubscriptionType::School,
        }
    }

    /// Trial period of the current plan
    #[must_use]
    pub const fn trial_period(&self) -> TrialPeriod {
        match self.subscription_type() {
            SubscriptionType::Individual => TrialPeriod(INDIVIDUAL_TRIAL_PERIOD),
            SubscriptionType::School => TrialPeriod(SCHOOL_TRIAL_PERIOD),
        }
    }

    /// Billing interval for the current plan
    #[must_use]
    pub const fn billing_interval(&self) -> BillingInterval {
        match self {
            Self::IndividualBasicMonthly | Self::IndividualProMonthly => BillingInterval::Monthly,
            _ => BillingInterval::Annually,
        }
    }
}

impl TryFrom<&str> for PlanType {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "individual-basic-monthly" => Ok(Self::IndividualBasicMonthly),
            "individual-basic-annually" => Ok(Self::IndividualBasicAnnually),
            "individual-pro-monthly" => Ok(Self::IndividualProMonthly),
            "individual-pro-annually" => Ok(Self::IndividualProAnnually),
            "school-level-1" => Ok(Self::SchoolLevel1),
            "school-level-2" => Ok(Self::SchoolLevel2),
            "school-level-3" => Ok(Self::SchoolLevel3),
            "school-level-4" => Ok(Self::SchoolLevel4),
            "school-unlimited" => Ok(Self::SchoolUnlimited),
            _ => Err(()),
        }
    }
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
    pub struct ChargeId("ch")
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
    pub struct PlanId("plan")
}

/// A subscription plan
///
/// In Stripe this would correspond to a Price within a Product.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct SubscriptionPlan {
    /// Local ID of the subscription plan
    pub plan_id: PlanId,
    /// Plan type
    pub plan_type: PlanType,
    /// Stripe price ID
    pub price_id: StripePriceId,
    /// When the plan was originally created.
    pub created_at: DateTime<Utc>,
    /// When the plan was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

make_path_parts!(SubscriptionPlanPath => "/v1/plans");

/// Request to create or update a subscription plans
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateSubscriptionPlansRequest {
    /// Map of price ids
    #[serde(flatten)]
    pub plans: HashMap<PlanType, StripePriceId>,
}

/// Request to create a subscription.
///
/// If no payment method information is passed with, then the system will attempt to use the
/// users existing payment method. Otherwise, a payment method will be saved.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSubscriptionRequest {
    /// Optional setup intent ID if a payment method was created prior to subscribing. Setting this
    /// mark the payment method as the default payment method.
    pub setup_intent_id: Option<String>,
    /// Plan to create the subscription for
    pub plan_type: PlanType,
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
    /// Plan to create the subscription for
    pub plan_type: PlanType,
}

make_path_parts!(CreateSetupIntentPath => "/v1/billing/payment-method");

wrap_uuid! {
    /// Account ID
    pub struct AccountId("acc")
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
    /// The type of plan the user's account is subscribed to
    pub plan_type: Option<PlanType>,
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
    pub struct SchoolId("sch")
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
    /// Whether this user is an admin. For non School accounts, this user will
    /// always be an admin
    pub is_admin: bool,
    /// Whether the user is verified for the account
    pub verified: bool,
}

wrap_uuid! {
    /// Wrapper type around [`Uuid`], represents the ID of a School Name.
    pub struct SchoolNameId("sn")
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

    /// The school's email address
    pub email: String,

    /// School location
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Value>,

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
    /// The school's email address
    #[serde(default, skip_serializing_if = "UpdateNonNullable::is_keep")]
    pub email: UpdateNonNullable<String>,

    /// The school's location
    #[serde(default, skip_serializing_if = "UpdateNullable::is_keep")]
    pub location: UpdateNullable<Value>,

    /// Description for school
    #[serde(default, skip_serializing_if = "UpdateNullable::is_keep")]
    pub description: UpdateNullable<String>,

    /// ID to the school's profile image in the user image library.
    #[serde(default, skip_serializing_if = "UpdateNullable::is_keep")]
    pub profile_image: UpdateNullable<ImageId>,

    /// Website for the school
    #[serde(default, skip_serializing_if = "UpdateNullable::is_keep")]
    pub website: UpdateNullable<String>,

    /// Organization type
    #[serde(default, skip_serializing_if = "UpdateNullable::is_keep")]
    pub organization_type: UpdateNullable<String>,
}

make_path_parts!(UpdateSchoolNamePath => "/v1/school/{}/school-name" => SchoolId);

make_path_parts!(IndividualAccountPath => "/v1/user/me/account");

/// Individual account response
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct IndividualAccountResponse {
    /// The users account, if any
    pub account: Option<Account>,
}
