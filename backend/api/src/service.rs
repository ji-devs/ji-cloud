use std::{ops::Deref, sync::Arc};

use actix_web::FromRequest;
use chrono::{DateTime, Duration, Utc};
use futures::future::{ready, Ready};
use tokio::sync::RwLock;

use crate::algolia;
use crate::error;
use crate::error::ServiceKind;
use crate::translate;
use core::google::GoogleAccessTokenResponse;

use self::translate::GoogleTranslate;
use self::upload::cleaner::UploadCleaner;

pub mod event_arc;
pub mod mail;
pub mod notifications;
pub mod s3;
pub mod storage;
pub mod upload;

pub trait Service {
    const DISABLED_ERROR: error::ServiceKind;
}

impl Service for algolia::Client {
    const DISABLED_ERROR: error::ServiceKind = error::ServiceKind::Algolia;
}

impl Service for algolia::SearchKeyStore {
    // todo: this should have a different error?
    const DISABLED_ERROR: error::ServiceKind = error::ServiceKind::Algolia;
}

impl Service for s3::Client {
    const DISABLED_ERROR: error::ServiceKind = error::ServiceKind::S3;
}

impl Service for storage::Client {
    const DISABLED_ERROR: error::ServiceKind = error::ServiceKind::GoogleCloudStorage;
}

impl Service for crate::service::event_arc::Client {
    const DISABLED_ERROR: error::ServiceKind = error::ServiceKind::GoogleCloudEventArc;
}

impl Service for crate::service::notifications::Client {
    const DISABLED_ERROR: error::ServiceKind = error::ServiceKind::FirebaseCloudMessaging;
}

impl Service for GcpAccessKeyStore {
    const DISABLED_ERROR: ServiceKind = ServiceKind::Algolia;
}

// TODO: set up for algolia
impl Service for algolia::Manager {
    const DISABLED_ERROR: ServiceKind = ServiceKind::Algolia;
}

impl Service for UploadCleaner {
    const DISABLED_ERROR: ServiceKind = error::ServiceKind::UploadCleaner;
}

impl Service for GoogleTranslate {
    const DISABLED_ERROR: ServiceKind = error::ServiceKind::GoogleTranslate;
}
#[derive(Debug)]
pub struct ServiceData<T: ?Sized>(Arc<T>);

impl<T: Service> ServiceData<T> {
    pub fn new(service: T) -> Self {
        Self(Arc::new(service))
    }
    /// Get reference to inner app data.
    pub fn _get_ref(&self) -> &T {
        self.0.as_ref()
    }

    /// Convert to the internal Arc<T>
    pub fn _into_inner(self) -> Arc<T> {
        self.0
    }
}

impl<T: Service + ?Sized> Deref for ServiceData<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Arc<T> {
        &self.0
    }
}

impl<T: Service + ?Sized> Clone for ServiceData<T> {
    fn clone(&self) -> ServiceData<T> {
        ServiceData(self.0.clone())
    }
}

impl<T: Service + ?Sized> From<Arc<T>> for ServiceData<T> {
    fn from(arc: Arc<T>) -> Self {
        ServiceData(arc)
    }
}

impl<T: Service + ?Sized + 'static> FromRequest for ServiceData<T> {
    type Future = Ready<Result<Self, Self::Error>>;
    type Error = error::Service;

    #[inline]
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_http::Payload) -> Self::Future {
        let data = req
            .app_data::<ServiceData<T>>()
            .cloned()
            .ok_or(error::Service::DisabledService(T::DISABLED_ERROR));

        ready(data)
    }
}

#[derive(Debug)]
pub struct GcpAccessKey {
    pub access_token: String,
    pub expires_at: Option<DateTime<Utc>>,
}

impl GcpAccessKey {
    const KEY_EXPIRATION_PAD_SECONDS: i64 = 360;

    /// checks if the key is stale
    ///
    /// returns `None` if there is no information about expiry time
    pub fn is_stale(&self) -> Option<bool> {
        if let Some(expires_at) = self.expires_at {
            let is_stale =
                Utc::now() >= (expires_at - Duration::seconds(Self::KEY_EXPIRATION_PAD_SECONDS));
            log::debug!(
                "GCP key is stale in: {:?}",
                expires_at - Duration::seconds(Self::KEY_EXPIRATION_PAD_SECONDS) - Utc::now()
            );
            Some(is_stale)
        } else {
            None
        }
    }

    fn update(&mut self, access_token: String, expires_at: Option<DateTime<Utc>>) {
        self.access_token = access_token;
        self.expires_at = expires_at;
    }
}

/// GCP auth token store, for calling other GCP services from the Cloud Run instance.
///
/// This shouldn't be invoked directly when handling requests, but instead let the the `GcpServiceData`
/// handler pass it as part of fetching the service data and check whether the token is still valid.
#[derive(Debug)]
pub struct GcpAccessKeyStore(RwLock<GcpAccessKey>);

impl GcpAccessKeyStore {
    pub fn new(resp: GoogleAccessTokenResponse) -> anyhow::Result<Self> {
        let key = GcpAccessKey {
            access_token: resp
                .access_token
                .expect("should have a token once reached here"),
            expires_at: resp.expires_at,
        };

        Ok(Self(RwLock::new(key)))
    }

    /// Fetch an access token.
    pub async fn fetch_token(&self) -> anyhow::Result<String> {
        let key = self.0.read().await;

        match key.is_stale() {
            None => {
                drop(key);
                return Err(anyhow::anyhow!("No token expiry info found?"));
            }
            Some(false) => {
                let token = key.access_token.clone();
                return Ok(token);
            }
            Some(true) => drop(key),
        }

        self.update_stale_token().await?;

        let access_token = self.0.read().await.access_token.clone();

        Ok(access_token)
    }

    async fn update_stale_token(&self) -> anyhow::Result<()> {
        let key_handler = self.0.read().await;

        // if the key is not stale, then we don't need to refresh it. some other request might have updated it
        // by the time the write lock was acquired
        match key_handler.is_stale() {
            None => {
                return Err(anyhow::anyhow!("No token expiry info found?"));
            }
            Some(false) => {
                log::debug!("GCP token is not stale, will not update.");
                return Ok(());
            }
            Some(true) => {
                log::debug!("GCP token is stale, fetching new token");
            }
        }

        drop(key_handler);

        let token_response = core::google::get_google_token_response_from_metadata_server().await?;

        (*self.0.write().await).update(
            token_response
                .access_token
                .expect("should fetch valid token on cloud"),
            token_response.expires_at,
        );

        Ok(())
    }
}
