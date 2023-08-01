use actix_web::{
    web::{self, Data, Json, ServiceConfig},
    HttpRequest, HttpResponse,
};
use anyhow::anyhow;
use chrono::{TimeZone, Utc};
use ji_core::settings::RuntimeSettings;
use shared::api::endpoints::billing::{
    CreateSetupIntent, UpdateSubscriptionCancellation, UpgradeSubscriptionPlan,
};
use shared::domain::billing::{
    Account, AccountType, AmountInCents, CancellationStatus, CreateSubscriptionRecord,
    StripeInvoiceId, StripeSubscriptionId, SubscriptionStatus, SubscriptionType,
    UpdateSubscriptionRecord,
};
use shared::error::BillingError;

use shared::domain::UpdateNonNullable;
use shared::{
    api::{endpoints::billing::CreateSubscription, ApiEndpoint, Method, PathParts},
    domain::{
        billing::{CreateSubscriptionResponse, CustomerId, PaymentMethod, SubscriptionPlan},
        user::UserProfile,
    },
    error::{IntoAnyhow, ServiceError, ServiceKindError},
};
use sqlx::PgPool;
use std::borrow::Borrow;
use std::str::FromStr;
use stripe::{
    generated::billing::subscription::SubscriptionProrationBehavior, Client, CreateCustomer,
    CreateSubscription as CreateStripeSubscription, CreateSubscriptionItems, Customer,
    CustomerInvoiceSettings, EventObject, EventType, List, ListPromotionCodes, PromotionCode,
    PromotionCodeId, SetupIntent, SetupIntentId, UpdateCustomer, UpdateSubscription, Webhook,
};
use tracing::instrument;

use crate::domain::user_authorization;
use crate::stripe::create_stripe_client;
use crate::{db, extractor::TokenUser};

async fn retrieve_promotion_code_id(
    client: &Client,
    promotion_code: &Option<String>,
) -> Result<Option<PromotionCodeId>, BillingError> {
    Ok(if let Some(promotion_code) = &promotion_code {
        let list_params = ListPromotionCodes {
            active: Some(true),
            code: Some(promotion_code),
            ..Default::default()
        };

        let List {
            data: mut codes, ..
        } = PromotionCode::list(&client, &list_params).await?;

        if codes.is_empty() || codes.len() > 1 {
            return Err(BillingError::InvalidPromotionCode(
                promotion_code.to_string(),
            ));
        }

        Some(codes.pop().unwrap().id)
    } else {
        None
    })
}

/// Create a new subscription for an authenticated user.
///
/// - Checks whether the user already has an _active_ subscription. If it does, it will return a Bad Request error.
/// - If the user has no Stripe customer associated with them, then a Stripe customer will be created.
/// - If a user has had a subscription in the past, then the user will not receive any trial days.
///   - If a user has _not_ had a previous subscription, they will receive trial days.
///   - If a user does not receive trial days then a Payment Intent will be the response so that the client can confirm it.
/// - Trial subscriptions will be canceled if payment could not be collected. I.e. if they don't have a payment method.
#[instrument(skip_all)]
async fn create_subscription(
    auth: TokenUser,
    db: Data<PgPool>,
    settings: Data<RuntimeSettings>,
    req: Json<<CreateSubscription as ApiEndpoint>::Req>,
) -> Result<
    (
        Json<<CreateSubscription as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    <CreateSubscription as ApiEndpoint>::Err,
> {
    let user_id = auth.user_id();

    // Fetch the subscription plan details
    let plan: SubscriptionPlan = db::billing::get_subscription_plan_by_type(&db, req.plan_type)
        .await
        .into_anyhow()?
        .ok_or(BillingError::NotFound(format!("Plan {}", req.plan_type)))?;

    // Fetch the profile for the user that's creating the subscription
    let user_profile: UserProfile = db::user::get_profile(db.as_ref(), &user_id)
        .await?
        .ok_or(BillingError::NotFound(format!("User {user_id}")))?;

    let client = create_stripe_client(&settings)?;

    let account = get_or_create_customer(db.as_ref(), &client, &user_profile, &plan).await?;

    let stripe_customer_id = stripe::CustomerId::from(account.stripe_customer_id.unwrap().clone());

    if let Some(setup_intent_id) = &req.setup_intent_id {
        let setup_intent_id = SetupIntentId::from_str(setup_intent_id)
            .map_err(|_| BillingError::InvalidSetupIntentId)?;
        let setup_intent = SetupIntent::retrieve(&client, &setup_intent_id, &[])
            .await
            .map_err(|_| BillingError::InvalidSetupIntentId)?;

        let payment_method_id = setup_intent
            .payment_method
            .ok_or(BillingError::InternalServerError(
                anyhow!("Missing payment_method from SetupIntent").into(),
            ))?
            .id()
            .to_string();

        let customer_invoice_settings = CustomerInvoiceSettings {
            default_payment_method: Some(payment_method_id),
            ..Default::default()
        };

        let update_customer = UpdateCustomer {
            invoice_settings: Some(customer_invoice_settings),
            ..Default::default()
        };

        Customer::update(&client, &stripe_customer_id, update_customer).await?;
    }

    // Create a Stripe subscription
    let stripe_subscription = {
        let mut params = CreateStripeSubscription::new(stripe_customer_id);
        params.items = Some(vec![CreateSubscriptionItems {
            price: Some(plan.price_id.into()),
            ..Default::default()
        }]);

        params.promotion_code = retrieve_promotion_code_id(&client, &req.promotion_code).await?;

        // This will mark the subscription as incomplete until the payment intent has been
        // confirmed.
        params.payment_behavior = Some(stripe::SubscriptionPaymentBehavior::AllowIncomplete);
        params.expand = &["latest_invoice.payment_intent"];

        // If the user hasn't previously had a subscription, then we can set their trial period.
        if account.subscription.is_none() {
            params.trial_period_days = Some(plan.plan_type.trial_period().inner() as u32);
            params.trial_settings = Some(stripe::CreateSubscriptionTrialSettings {
                end_behavior: stripe::CreateSubscriptionTrialSettingsEndBehavior {
                    missing_payment_method: stripe::CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Cancel,
                },
            });
        }

        stripe::Subscription::create(&client, params).await?
    };

    let stripe_subscription_id: StripeSubscriptionId = stripe_subscription.id.into();

    let latest_invoice_id = stripe_subscription
        .latest_invoice
        .as_ref()
        .map(|invoice| StripeInvoiceId::from(&invoice.id()));

    let amount_due_in_cents = match stripe_subscription.latest_invoice.as_ref() {
        Some(invoice) => invoice
            .as_object()
            .unwrap()
            .amount_remaining
            .map(AmountInCents::new),
        None => None,
    };

    // Fetch the latest invoice so that we can retrieve the client secret. This is useful if the
    // user doesn't get a trial, and needs to add a payment method so that the subscription can be
    // completed.
    let create_response = match stripe_subscription.latest_invoice {
        Some(invoice) => {
            invoice
                .as_object()
                .unwrap()
                .payment_intent
                .as_ref()
                .map(|payment_intent| CreateSubscriptionResponse {
                    subscription_id: stripe_subscription_id.clone(),
                    client_secret: payment_intent
                        .as_object()
                        .unwrap()
                        .client_secret
                        .as_ref()
                        .unwrap()
                        .clone(),
                })
        }
        None => None,
    };

    // Create subscription in database
    let subscription = CreateSubscriptionRecord {
        stripe_subscription_id,
        subscription_plan_id: plan.plan_id,
        status: Default::default(), // This will be updated in the webhook
        current_period_end: Utc
            .timestamp_opt(stripe_subscription.current_period_end, 0)
            .latest()
            .ok_or(anyhow::anyhow!("Invalid timestamp"))?,
        account_id: account.account_id,
        latest_invoice_id,
        amount_due_in_cents,
    };

    db::billing::create_subscription(db.as_ref(), subscription)
        .await
        .into_anyhow()?;

    Ok((Json(create_response), http::StatusCode::CREATED))
}

#[instrument(skip_all)]
async fn update_subscription_cancel_status(
    auth: TokenUser,
    db: Data<PgPool>,
    settings: Data<RuntimeSettings>,
    req: Json<<UpdateSubscriptionCancellation as ApiEndpoint>::Req>,
) -> Result<HttpResponse, <UpdateSubscriptionCancellation as ApiEndpoint>::Err> {
    let user_id = auth.user_id();
    let account = db::account::get_account_by_user_id(db.as_ref(), &user_id)
        .await?
        .ok_or_else(|| BillingError::NotFound("User does not have an account".into()))?;

    user_authorization(db.as_ref(), &user_id, &account.account_id)
        .await
        .map_err(Into::<BillingError>::into)?
        .test_authorized(true)?;

    let subscription =
        db::billing::get_latest_subscription_by_account_id(db.as_ref(), account.account_id)
            .await
            .into_anyhow()?
            .ok_or_else(|| {
                BillingError::NotFound("User does not have an existing subscription".into())
            })?;

    let client = create_stripe_client(&settings)?;

    let stripe_id: Result<stripe::SubscriptionId, anyhow::Error> =
        subscription.stripe_subscription_id.try_into();

    let stripe_id: stripe::SubscriptionId = stripe_id?;

    let update_subscription = match &req.status {
        CancellationStatus::CancelAtPeriodEnd => {
            if !subscription.status.is_active() {
                return Err(BillingError::NoActiveSubscription);
            }

            UpdateSubscription {
                cancel_at_period_end: Some(true),
                ..Default::default()
            }
        }
        CancellationStatus::RemoveCancellation => {
            if !subscription.status.is_canceled() {
                return Err(BillingError::NoCanceledSubscription);
            }

            UpdateSubscription {
                cancel_at_period_end: Some(false),
                ..Default::default()
            }
        }
    };

    let updated_subscription =
        stripe::Subscription::update(&client, &stripe_id, update_subscription).await?;

    db::billing::save_subscription(
        db.as_ref(),
        UpdateSubscriptionRecord::try_from(updated_subscription)?,
    )
    .await
    .into_anyhow()?;

    Ok(HttpResponse::NoContent().finish())
}

#[instrument(skip_all)]
async fn upgrade_subscription_plan(
    auth: TokenUser,
    db: Data<PgPool>,
    settings: Data<RuntimeSettings>,
    req: Json<<UpgradeSubscriptionPlan as ApiEndpoint>::Req>,
) -> Result<HttpResponse, <UpgradeSubscriptionPlan as ApiEndpoint>::Err> {
    let user_id = auth.user_id();
    let account = db::account::get_account_by_user_id(db.as_ref(), &user_id)
        .await?
        .ok_or_else(|| BillingError::NotFound("User does not have an account".into()))?;

    user_authorization(db.as_ref(), &user_id, &account.account_id)
        .await
        .map_err(Into::<BillingError>::into)?
        .test_authorized(true)?;

    let subscription =
        db::billing::get_latest_subscription_by_account_id(db.as_ref(), account.account_id)
            .await
            .into_anyhow()?
            .ok_or_else(|| {
                BillingError::NotFound("User does not have an existing subscription".into())
            })?;

    // Fetch the subscription plan details
    let plan: SubscriptionPlan = db::billing::get_subscription_plan_by_type(&db, req.plan_type)
        .await
        .into_anyhow()?
        .ok_or(BillingError::NotFound(format!("Plan {}", req.plan_type)))?;

    if !plan
        .plan_type
        .can_upgrade_from(subscription.subscription_plan_type)
    {
        return Err(BillingError::InvalidUpgradePlanType {
            upgrade_to: plan.plan_type,
            upgrade_from: subscription.subscription_plan_type,
        });
    }

    let client = create_stripe_client(&settings)?;

    let stripe_id: Result<stripe::SubscriptionId, anyhow::Error> =
        subscription.stripe_subscription_id.try_into();

    let stripe_id: stripe::SubscriptionId = stripe_id?;

    let stripe_subscription = stripe::Subscription::retrieve(&client, &stripe_id, &[]).await?;

    let price_item = stripe_subscription.items.data.first().ok_or(anyhow!(
        "Expected exactly one subscription item for subscription {stripe_id}"
    ))?;

    let update_subscription = UpdateSubscription {
        items: Some(vec![stripe::UpdateSubscriptionItems {
            id: Some(price_item.id.to_string()),
            price: Some(plan.price_id.into()),
            ..Default::default()
        }]),
        promotion_code: retrieve_promotion_code_id(&client, &req.promotion_code).await?,
        proration_behavior: Some(SubscriptionProrationBehavior::AlwaysInvoice),
        ..Default::default()
    };

    let updated_subscription =
        stripe::Subscription::update(&client, &stripe_id, update_subscription).await?;

    let mut update_record = UpdateSubscriptionRecord::try_from(updated_subscription)?;
    update_record.subscription_plan_id = UpdateNonNullable::Change(plan.plan_id);

    db::billing::save_subscription(db.as_ref(), update_record)
        .await
        .into_anyhow()?;

    Ok(HttpResponse::NoContent().finish())
}

#[instrument(skip_all)]
async fn create_setup_intent(
    auth: TokenUser,
    db: Data<PgPool>,
    settings: Data<RuntimeSettings>,
    req: Json<<CreateSetupIntent as ApiEndpoint>::Req>,
) -> Result<
    (
        Json<<CreateSetupIntent as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    <CreateSetupIntent as ApiEndpoint>::Err,
> {
    let user_id = auth.user_id();

    // Fetch the subscription plan details
    let plan: SubscriptionPlan = db::billing::get_subscription_plan_by_type(&db, req.plan_type)
        .await
        .into_anyhow()?
        .ok_or(BillingError::NotFound(format!("Plan {}", req.plan_type)))?;

    // Fetch the profile for the user that wants to subscribe
    let user_profile: UserProfile = db::user::get_profile(db.as_ref(), &user_id)
        .await?
        .ok_or(BillingError::NotFound(format!("User {user_id}")))?;

    let client = create_stripe_client(&settings)?;

    let account = get_or_create_customer(db.as_ref(), &client, &user_profile, &plan).await?;
    let customer_id = account.stripe_customer_id.unwrap(); // get_or_create_customer guarantees that this is `Some`

    let create_setup_intent = stripe::CreateSetupIntent {
        customer: Some(customer_id.into()),
        payment_method_types: Some(vec!["card".into(), "link".into()]),
        // TODO need to set `automatic_payment_methods` but it isn't available in async-stripe?
        ..Default::default()
    };
    let setup_intent = SetupIntent::create(&client, create_setup_intent).await?;

    Ok((
        Json(
            setup_intent
                .client_secret
                .ok_or(anyhow!("Missing client secret"))?,
        ),
        http::StatusCode::CREATED,
    ))
}

/// Get the user accounts customer ID. If they don't have one yet, then we create one here.
#[instrument(skip_all)]
async fn get_or_create_customer(
    db: &PgPool,
    client: &Client,
    user_profile: &UserProfile,
    plan: &SubscriptionPlan,
) -> Result<Account, BillingError> {
    let mut account =
        if let Some(account) = db::account::get_account_by_user_id(db, &user_profile.id).await? {
            if let Some(subscription) = &account.subscription {
                // Check whether they have an _active_ subscription
                if !matches!(subscription.status, SubscriptionStatus::Expired) {
                    // If a subscription exists, we don't want to create a new subscription
                    return Err(BillingError::SubscriptionExists)?;
                }
            }

            if !account
                .account_type
                .matches_subscription_type(&plan.plan_type.subscription_type())
            {
                return Err(BillingError::IncorrectPlanType {
                    expected: account.account_type,
                    found: plan.plan_type.subscription_type(),
                });
            }

            account
        } else {
            if !matches!(
                plan.plan_type.subscription_type(),
                SubscriptionType::Individual
            ) {
                return Err(BillingError::IncorrectPlanType {
                    expected: AccountType::Individual,
                    found: SubscriptionType::School,
                });
            }

            db::account::create_default_individual_account(db, &user_profile.id)
                .await
                .into_anyhow()?;

            db::account::get_account_by_user_id(db, &user_profile.id)
                .await?
                .ok_or(BillingError::from(anyhow!("Missing account")))?
        };

    if account.stripe_customer_id.is_none() {
        let customer_id = match account.account_type {
            AccountType::School => {
                let school = db::account::get_school_account_by_account_id(db, &account.account_id)
                    .await
                    .into_anyhow()?
                    .ok_or(BillingError::SchoolNotFound)?;

                create_stripe_customer(
                    client,
                    CreateCustomer {
                        email: Some(school.email.as_str()),
                        name: Some(school.school_name.as_str()),
                        ..Default::default()
                    },
                )
                .await?
            }
            AccountType::Individual => {
                create_stripe_customer(
                    client,
                    CreateCustomer {
                        email: Some(user_profile.email.as_str()),
                        name: Some(&format!(
                            "{} {}",
                            user_profile.given_name, user_profile.family_name
                        )),
                        ..Default::default()
                    },
                )
                .await?
            }
        };

        db::account::save_customer_id(db, &account.account_id, &customer_id).await?;

        account.stripe_customer_id = Some(customer_id);
    }

    Ok(account)
}

#[instrument(skip(client))]
async fn create_stripe_customer(
    client: &Client,
    create_customer: CreateCustomer<'_>,
) -> Result<CustomerId, BillingError> {
    Ok(Customer::create(client, create_customer).await?.id.into())
}

#[instrument(skip_all)]
async fn webhook(
    db: Data<PgPool>,
    req: HttpRequest,
    settings: Data<RuntimeSettings>,
    payload: web::Bytes,
) -> Result<HttpResponse, BillingError> {
    let secret = settings
        .stripe_webhook_secret
        .as_ref()
        .ok_or(ServiceError::DisabledService(ServiceKindError::Stripe))?;

    let payload_str = std::str::from_utf8(payload.borrow()).unwrap();

    let stripe_signature = req
        .headers()
        .get("Stripe-Signature")
        .ok_or(BillingError::MissingStripeSignature)?
        .to_str()
        .ok()
        .unwrap_or_default();

    match Webhook::construct_event(payload_str, stripe_signature, secret) {
        Ok(event) => match event.type_ {
            EventType::PaymentMethodAttached => {
                save_payment_method(db, event.data.object, EventType::PaymentMethodAttached)
                    .await?;
            }
            EventType::PaymentMethodUpdated => {
                save_payment_method(db, event.data.object, EventType::PaymentMethodUpdated).await?;
            }
            EventType::PaymentMethodDetached => {
                save_payment_method(db, event.data.object, EventType::PaymentMethodDetached)
                    .await?;
            }
            _ => {
                match event.data.object {
                    EventObject::Subscription(subscription) => {
                        let _span = tracing::info_span!("subscription event");

                        // Save a subscription from a subscription event
                        // Note: this will handle invoice changes on subscriptions as well since a
                        // subscription is updated when an invoice is paid/unpaid/etc.
                        let update_subscription = UpdateSubscriptionRecord::try_from(subscription)?;

                        db::billing::save_subscription(db.as_ref(), update_subscription)
                            .await
                            .into_anyhow()?;
                    }
                    EventObject::Invoice(invoice) => {
                        let _span = tracing::info_span!("invoice event");

                        let invoice_id = StripeInvoiceId::from(&invoice.id);

                        if let Some(subscription_id) =
                            db::billing::get_stripe_subscription_id_by_invoice_id(
                                db.as_ref(),
                                &invoice_id,
                            )
                            .await
                            .into_anyhow()?
                        {
                            db::billing::set_subscription_amount_due(
                                db.as_ref(),
                                subscription_id,
                                AmountInCents::new(invoice.amount_remaining.unwrap_or_default()),
                            )
                            .await
                            .into_anyhow()?;
                        }
                    }
                    _ => {
                        log::trace!("Unknown event encountered in webhook: {:?}", event.type_);
                    }
                }
            }
        },
        Err(error) => {
            log::warn!("Failed to construct webhook event: {error:#?}");
        }
    }

    Ok(HttpResponse::Ok().finish())
}

/// Save a payment method for a customer. This will overwrite the existing payment method
/// if there is one. If `payment_method` is `None`, then the customer's payment method
/// will be removed.
#[instrument(skip(db, event_object))]
async fn save_payment_method(
    db: Data<PgPool>,
    event_object: EventObject,
    event_type: EventType,
) -> anyhow::Result<()> {
    let (customer_id, payment_method) =
        if let EventObject::PaymentMethod(payment_method) = event_object {
            if let Some(customer) = &payment_method.customer {
                let customer_id = CustomerId::from(customer.id());

                let payment_method = if let EventType::PaymentMethodDetached = event_type {
                    None
                } else {
                    Some(payment_method)
                };
                (customer_id, payment_method)
            } else {
                log::warn!("No customer associated with payment method event");
                return Ok(());
            }
        } else {
            log::warn!("EventObject was not `PaymentMethod`");
            return Ok(());
        };

    match db::account::get_account_id_by_customer_id(db.as_ref(), &customer_id).await? {
        Some(account_id) => {
            db::account::save_payment_method(
                db.as_ref(),
                &account_id,
                payment_method.map(PaymentMethod::from),
            )
            .await?;
        }
        None => {
            log::warn!("Customer ID {customer_id:?} not found");
        }
    }

    Ok(())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <CreateSubscription as ApiEndpoint>::Path::PATH,
        CreateSubscription::METHOD.route().to(create_subscription),
    )
    .route(
        <UpdateSubscriptionCancellation as ApiEndpoint>::Path::PATH,
        UpdateSubscriptionCancellation::METHOD
            .route()
            .to(update_subscription_cancel_status),
    )
    .route(
        <UpgradeSubscriptionPlan as ApiEndpoint>::Path::PATH,
        UpgradeSubscriptionPlan::METHOD
            .route()
            .to(upgrade_subscription_plan),
    )
    .route(
        <CreateSetupIntent as ApiEndpoint>::Path::PATH,
        CreateSetupIntent::METHOD.route().to(create_setup_intent),
    )
    .route("/v1/stripe-webhook", Method::Post.route().to(webhook));
}
