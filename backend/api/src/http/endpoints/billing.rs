use actix_web::{
    web::{self, Data, Json, ServiceConfig},
    HttpRequest, HttpResponse,
};
use anyhow::anyhow;
use chrono::{TimeZone, Utc};
use core::settings::RuntimeSettings;
use shared::api::endpoints::billing::{CreateSetupIntent, GetSubscriptionPlans};
use shared::domain::billing::{
    AmountInCents, CreateSubscriptionRecord, StripeInvoiceId, StripeSubscriptionId,
    SubscriptionStatus, UpdateSubscriptionRecord,
};
use shared::{
    api::{endpoints::billing::CreateSubscription, ApiEndpoint, Method, PathParts},
    domain::{
        billing::{CreateSubscriptionResponse, CustomerId, PaymentMethod, SubscriptionPlan},
        user::UserProfile,
    },
};
use sqlx::PgPool;
use std::borrow::Borrow;
use stripe::{
    Client, CreateCustomer, CreateSubscription as CreateStripeSubscription,
    CreateSubscriptionItems, Customer, EventObject, EventType, Webhook,
};
use tracing::instrument;

use crate::{db, error, extractor::TokenUser};

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
    error::Billing,
> {
    let user_id = auth.user_id();

    // Fetch the profile for the user that wants to subscribe
    let user_profile: UserProfile = db::user::get_profile(db.as_ref(), &user_id)
        .await?
        .ok_or(error::Billing::NotFound)?;

    // So that we don't add a trial on if the teacher already had a subscription which may have been
    // cancelled or expired previously.
    let mut prev_subscription_exists = false;

    if let Some(subscription) = &user_profile.subscription {
        // They have at least had a subscription before
        prev_subscription_exists = true;

        // Check whether they have an _active_ subscription
        if !matches!(subscription.status, SubscriptionStatus::Expired) {
            // If a subscription exists, we don't want to create a new subscription
            return Err(error::Billing::SubscriptionExists)?;
        }
    }

    let client = create_stripe_client(&settings)?;

    let customer_id = get_or_create_customer(db.as_ref(), &client, &user_profile).await?;

    // Fetch the subscription plan details
    let plan: SubscriptionPlan = db::billing::get_subscription_plan_by_id(&db, req.plan_id)
        .await?
        .ok_or(error::Billing::NotFound)?;

    // Create a Stripe subscription
    let stripe_subscription = {
        let mut params = CreateStripeSubscription::new(stripe::CustomerId::from(customer_id));
        params.items = Some(vec![CreateSubscriptionItems {
            price: Some(plan.price_id.into()),
            ..Default::default()
        }]);

        // This will mark the subscription as incomplete until the payment intent has been
        // confirmed.
        params.payment_behavior = Some(stripe::SubscriptionPaymentBehavior::AllowIncomplete);
        params.expand = &["latest_invoice.payment_intent"];

        // If the user hasn't previously had a subscription, then we can set their trial period.
        if !prev_subscription_exists {
            if let Some(trial_period) = plan.trial_period {
                params.trial_period_days = Some(trial_period.inner() as u32);
                params.trial_settings = Some(stripe::CreateSubscriptionTrialSettings {
                    end_behavior: stripe::CreateSubscriptionTrialSettingsEndBehavior {
                        missing_payment_method: stripe::CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Cancel,
                    },
                });
            }
        }

        stripe::Subscription::create(&client, params)
            .await
            .map_err(error::Billing::Stripe)?
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
        tier: plan.subscription_tier,
        auto_renew: true,           // TODO is this always true initially?
        status: Default::default(), // This will be updated in the webhook
        current_period_end: Utc
            .timestamp_opt(stripe_subscription.current_period_end, 0)
            .latest()
            .ok_or(anyhow::anyhow!("Invalid timestamp"))?,
        user_id: user_profile.id,
        latest_invoice_id,
        amount_due_in_cents,
    };

    let subscription_id = db::billing::create_subscription(db.as_ref(), subscription).await?;

    // Save the subscription ID against the user
    db::user::save_subscription_id(db.as_ref(), &user_id, subscription_id).await?;

    Ok((Json(create_response), http::StatusCode::CREATED))
}

#[instrument(skip_all)]
async fn create_setup_intent(
    auth: TokenUser,
    db: Data<PgPool>,
    settings: Data<RuntimeSettings>,
) -> Result<
    (
        Json<<CreateSetupIntent as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    error::Billing,
> {
    let user_id = auth.user_id();

    // Fetch the profile for the user that wants to subscribe
    let user_profile: UserProfile = db::user::get_profile(db.as_ref(), &user_id)
        .await?
        .ok_or(error::Billing::NotFound)?;

    let client = create_stripe_client(&settings)?;

    let customer_id = get_or_create_customer(db.as_ref(), &client, &user_profile).await?;

    let create_setup_intent = stripe::CreateSetupIntent {
        customer: Some(customer_id.into()),
        payment_method_types: Some(vec!["card".into(), "link".into()]),
        // TODO need to set `automatic_payment_methods` but it isn't available in async-stripe?
        ..Default::default()
    };
    let setup_intent = stripe::SetupIntent::create(&client, create_setup_intent).await?;

    Ok((
        Json(
            setup_intent
                .client_secret
                .ok_or(anyhow!("Missing client secret"))?,
        ),
        http::StatusCode::CREATED,
    ))
}

#[instrument(skip_all)]
fn create_stripe_client(settings: &RuntimeSettings) -> Result<Client, error::Billing> {
    let secret = settings
        .stripe_secret_key
        .as_ref()
        .ok_or(error::Service::DisabledService(error::ServiceKind::Stripe))?;

    Ok(Client::new(secret))
}

/// Get the users customer ID. If they don't have one yet, then we create one here.
#[instrument(skip_all)]
async fn get_or_create_customer(
    db: &PgPool,
    client: &Client,
    user_profile: &UserProfile,
) -> Result<CustomerId, error::Billing> {
    Ok(match &user_profile.stripe_customer_id {
        Some(customer_id) => customer_id.clone(),
        None => {
            let customer_id = Customer::create(
                client,
                CreateCustomer {
                    email: Some(user_profile.email.as_str()),
                    ..Default::default()
                },
            )
            .await
            .map_err(error::Billing::Stripe)?
            .id
            .into();

            db::user::save_customer_id(db, &user_profile.id, &customer_id).await?;

            customer_id
        }
    })
}

#[instrument(skip_all)]
async fn webhook(
    db: Data<PgPool>,
    req: HttpRequest,
    settings: Data<RuntimeSettings>,
    payload: web::Bytes,
) -> Result<HttpResponse, error::Billing> {
    let secret = settings
        .stripe_webhook_secret
        .as_ref()
        .ok_or(error::Service::DisabledService(error::ServiceKind::Stripe))?;

    let payload_str = std::str::from_utf8(payload.borrow()).unwrap();

    let stripe_signature = req
        .headers()
        .get("Stripe-Signature")
        .ok_or(error::Billing::BadRequest)?
        .to_str()
        .ok()
        .unwrap_or_default();

    match Webhook::construct_event(payload_str, stripe_signature, secret) {
        Ok(event) => match event.event_type {
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

                        db::billing::save_subscription(db.as_ref(), update_subscription).await?;
                    }
                    EventObject::Invoice(invoice) => {
                        let _span = tracing::info_span!("invoice event");

                        let invoice_id = StripeInvoiceId::from(&invoice.id);

                        if let Some(subscription_id) =
                            db::billing::get_stripe_subscription_id_with_invoice_id(
                                db.as_ref(),
                                &invoice_id,
                            )
                            .await?
                        {
                            db::billing::set_subscription_amount_due(
                                db.as_ref(),
                                subscription_id,
                                AmountInCents::new(invoice.amount_remaining.unwrap_or_default()),
                            )
                            .await?;
                        }
                    }
                    _ => {
                        log::trace!(
                            "Unknown event encountered in webhook: {:?}",
                            event.event_type
                        );
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

    match db::user::get_user_id_by_customer_id(db.as_ref(), &customer_id).await? {
        Some(user_id) => {
            db::user::save_payment_method(
                db.as_ref(),
                &user_id,
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

pub async fn get_subscription_plans(
    db: Data<PgPool>,
) -> Result<Json<<GetSubscriptionPlans as ApiEndpoint>::Res>, error::Billing> {
    let plans = db::billing::get_subscription_plans(db.as_ref()).await?;

    Ok(Json(plans.try_into()?))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <CreateSubscription as ApiEndpoint>::Path::PATH,
        CreateSubscription::METHOD.route().to(create_subscription),
    )
    .route(
        <CreateSetupIntent as ApiEndpoint>::Path::PATH,
        CreateSetupIntent::METHOD.route().to(create_setup_intent),
    )
    .route("/v1/stripe-webhook", Method::Post.route().to(webhook))
    .route(
        <GetSubscriptionPlans as ApiEndpoint>::Path::PATH,
        GetSubscriptionPlans::METHOD
            .route()
            .to(get_subscription_plans),
    );
}
