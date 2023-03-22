use actix_web::{
    web::{self, Data, Json, ServiceConfig},
    HttpRequest, HttpResponse,
};
use chrono::{TimeZone, Utc};
use core::settings::RuntimeSettings;
use shared::domain::billing::{
    CreateSubscriptionRecord, StripeSubscriptionId, SubscriptionStatus, UpdateSubscriptionRecord,
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
/// - Trial subscriptions will be paused if payment could not be collected
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

    if let Some(subscription_id) = user_profile.subscription_id {
        // They have at least had a subscription before
        prev_subscription_exists = true;

        let subscription = db::billing::get_subscription(db.as_ref(), subscription_id).await?;

        if let Some(subscription) = subscription {
            // Check whether they have an _active_ subscription
            if !matches!(subscription.status, SubscriptionStatus::Expired) {
                // If a subscription exists, we don't want to create a new subscription
                return Err(error::Billing::SubscriptionExists)?;
            }
        }
    }

    let secret = settings
        .stripe_secret_key
        .as_ref()
        .ok_or(error::Service::DisabledService(error::ServiceKind::Stripe))?;

    let client = Client::new(secret);

    // Get the users customer ID. If they don't have one yet, then we create one here.
    let customer_id = match user_profile.stripe_customer_id {
        Some(customer_id) => customer_id,
        None => {
            let customer_id = Customer::create(
                &client,
                CreateCustomer {
                    email: Some(user_profile.email.as_str()),
                    ..Default::default()
                },
            )
            .await
            .map_err(|error| error::Billing::Stripe(error))?
            .id
            .into();

            db::user::save_customer_id(db.as_ref(), &user_profile.id, &customer_id).await?;

            customer_id
        }
    };

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
        params.payment_behavior = Some(stripe::SubscriptionPaymentBehavior::DefaultIncomplete);
        params.expand = &["latest_invoice.payment_intent"];

        // If the user hasn't previously had a subscription, then we can set their trial period.
        if !prev_subscription_exists {
            params.trial_period_days = Some(7);
            params.trial_settings = Some(stripe::CreateSubscriptionTrialSettings {
                end_behavior: stripe::CreateSubscriptionTrialSettingsEndBehavior {
                    missing_payment_method: stripe::CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Pause,
                },
            });
        }

        stripe::Subscription::create(&client, params)
            .await
            .map_err(|error| error::Billing::Stripe(error))?
    };

    let stripe_subscription_id: StripeSubscriptionId = stripe_subscription.id.into();

    // Fetch the latest invoice so that we can retrieve the client secret. This is useful if the
    // user doesn't get a trial, and needs to add a payment method so that the subscription can be
    // completed.
    let latest_invoice = stripe_subscription.latest_invoice.unwrap();

    let create_response = latest_invoice
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
                .to_owned(),
        });

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
    };

    let subscription_id = db::billing::create_subscription(db.as_ref(), subscription).await?;

    // Save the subscription ID against the user
    db::user::save_subscription_id(db.as_ref(), &user_id, subscription_id).await?;

    Ok((Json(create_response), http::StatusCode::CREATED))
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
            EventType::CustomerSubscriptionCreated => {
                save_subscription(db, event.data.object).await?;
            }
            EventType::CustomerSubscriptionUpdated => {
                save_subscription(db, event.data.object).await?;
            }
            EventType::CustomerSubscriptionDeleted => {
                save_subscription(db, event.data.object).await?;
            }
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
                log::trace!(
                    "Unknown event encountered in webhook: {:?}",
                    event.event_type
                );
            }
        },
        Err(error) => {
            log::warn!("Failed to construct webhook event: {error:#?}");
        }
    }

    Ok(HttpResponse::Ok().finish())
}

/// Save a subscription from a subscription event
#[instrument(skip(db, event_object))]
async fn save_subscription(db: Data<PgPool>, event_object: EventObject) -> anyhow::Result<()> {
    let subscription = match event_object {
        EventObject::Subscription(subscription) => {
            UpdateSubscriptionRecord::try_from(subscription)?
        }
        _ => {
            log::warn!("EventObject was not a `Subscription`");
            return Ok(());
        }
    };

    db::billing::save_subscription(db.as_ref(), subscription).await?;

    Ok(())
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
    let (customer_id, payment_method) = match event_object {
        EventObject::PaymentMethod(payment_method) => {
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
        }
        _ => {
            log::warn!("EventObject was not `PaymentMethod`");
            return Ok(());
        }
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

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <CreateSubscription as ApiEndpoint>::Path::PATH,
        CreateSubscription::METHOD.route().to(create_subscription),
    )
    .route("/v1/stripe-webhook", Method::Post.route().to(webhook));
}
