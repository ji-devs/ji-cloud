// routes that are passed on to the frontend server. Needed for resources that have to be served from the same origin

use core::settings::RuntimeSettings;
use std::error::Error;

use actix_web::{error::ErrorInternalServerError, web::Data, HttpResponse};
use cached::proc_macro::cached;
use shared::config::RemoteTarget;

#[cached(time = 60, result = true, sync_writes = true)]
async fn get_from_frontend_server(route: String) -> Result<String, Box<dyn Error>> {
    Ok(reqwest::get(route)
        .await?
        .error_for_status()?
        .text()
        .await?)
}

fn static_frontend_url(remote_target: RemoteTarget, path: &str) -> String {
    format!("{}/static/{}", remote_target.frontend_url(), path)
}
pub async fn service_worker(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    let route = static_frontend_url(settings.remote_target(), "service-worker.js");

    let res: String = get_from_frontend_server(route).await.map_err(|e| {
        println!("Error fetching service-worker.js: {:?}", e);
        ErrorInternalServerError("")
    })?;

    Ok(actix_web::HttpResponse::Ok().body(res))
}

pub async fn manifest(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    let route = static_frontend_url(settings.remote_target(), "manifest.json");

    let res: String = get_from_frontend_server(route).await.map_err(|e| {
        println!("Error fetching manifest.json: {:?}", e);
        ErrorInternalServerError("")
    })?;

    Ok(actix_web::HttpResponse::Ok().body(res))
}
