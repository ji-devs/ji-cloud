use std::{
    future::{ready, Ready},
    ops::Deref,
    sync::Arc,
};

use actix_web::FromRequest;
use paperclip::{actix::OperationModifier, v2::schema::Apiv2Schema};

use crate::error;

pub mod mail;

pub mod event_arc;
pub mod notifications;
pub mod storage;
pub mod uploads;

pub trait Service {
    const DISABLED_ERROR: error::ServiceKind;
}

impl Service for crate::algolia::Client {
    const DISABLED_ERROR: error::ServiceKind = error::ServiceKind::Algolia;
}

impl Service for crate::algolia::SearchKeyStore {
    // todo: this should have a different error?
    const DISABLED_ERROR: error::ServiceKind = error::ServiceKind::Algolia;
}

impl Service for crate::s3::Client {
    const DISABLED_ERROR: error::ServiceKind = error::ServiceKind::S3;
}

impl Service for storage::Client {
    const DISABLED_ERROR: error::ServiceKind = error::ServiceKind::GoogleCloudStorage;
}

impl Service for crate::service::event_arc::Client {
    const DISABLED_ERROR: error::ServiceKind = error::ServiceKind::GoogleCloudStorage;
}

impl Service for crate::service::notifications::Client {
    const DISABLED_ERROR: error::ServiceKind = error::ServiceKind::FirebaseCloudMessaging;
}

#[derive(Debug)]
pub struct ServiceData<T: ?Sized>(Arc<T>);

impl<T> Apiv2Schema for ServiceData<T> {}
impl<T> OperationModifier for ServiceData<T> {}

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
    type Config = ();

    #[inline]
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_http::Payload) -> Self::Future {
        let data = req
            .app_data::<ServiceData<T>>()
            .cloned()
            .ok_or(error::Service::DisabledService(T::DISABLED_ERROR));

        ready(data)
    }
}
