// routes that are passed on to the frontend server. Needed for resources that have to be served from the same origin

use core::settings::RuntimeSettings;
use std::{collections::HashMap, error::Error};

use actix_web::{error::ErrorInternalServerError, web::Data, HttpResponse};
use cached::proc_macro::cached;
use shared::config::RemoteTarget;

#[cached(time = 60, result = true, sync_writes = true)]
async fn get_from_frontend_server(
    route: String,
) -> Result<(String, HashMap<String, Vec<u8>>), Box<dyn Error>> {
    let res = reqwest::get(route).await?;

    let headers = res
        .headers()
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.as_bytes().to_vec()))
        .collect();

    let body = res.text().await?;

    Ok((body, headers))
}

fn static_frontend_url(remote_target: RemoteTarget, path: &str) -> String {
    format!("{}/static/{}", remote_target.frontend_url(), path)
}
pub async fn service_worker(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    let route = static_frontend_url(settings.remote_target(), "service-worker.js");

    let (body, headers) = get_from_frontend_server(route).await.map_err(|e| {
        println!("Error fetching service-worker.js: {:?}", e);
        ErrorInternalServerError("")
    })?;

    let mut res = actix_web::HttpResponse::Ok();
    headers.into_iter().for_each(|header| {
        res.insert_header(header);
    });
    let res = res.body(body);

    Ok(res)
}

pub async fn manifest(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    let route = static_frontend_url(settings.remote_target(), "manifest.json");

    let (body, headers) = get_from_frontend_server(route).await.map_err(|e| {
        println!("Error fetching manifest.json: {:?}", e);
        ErrorInternalServerError("")
    })?;

    let mut res = actix_web::HttpResponse::Ok();
    headers.into_iter().for_each(|header| {
        res.insert_header(header);
    });
    let res = res.body(body);

    Ok(res)
}
