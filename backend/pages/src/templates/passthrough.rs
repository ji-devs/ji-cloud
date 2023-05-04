// routes that are passed on to the frontend server. Needed for resources that have to be served from the same origin

use core::settings::RuntimeSettings;
use std::{collections::HashMap, error::Error};

use actix_web::{error::ErrorInternalServerError, web::Data, HttpResponse};
use cached::proc_macro::cached;
use shared::config::RemoteTarget;

#[cached(time = 60, result = true, sync_writes = true)]
async fn get_from_frontend_server(
    route: String,
) -> Result<(Vec<u8>, HashMap<String, Vec<u8>>), Box<dyn Error>> {
    let res = reqwest::get(route).await?;

    let headers = res
        .headers()
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.as_bytes().to_vec()))
        .collect();

    let body = res.bytes().await?.to_vec();

    Ok((body, headers))
}

fn static_frontend_url(remote_target: RemoteTarget, path: &str) -> String {
    format!("{}/static/{}", remote_target.frontend_url(), path)
}

macro_rules! create_passthrough_route_handler {
    ($fn_name:ident, $route:literal) => {
        pub async fn $fn_name(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
            let route = static_frontend_url(settings.remote_target(), $route);

            let (body, headers) = get_from_frontend_server(route).await.map_err(|e| {
                println!("Error fetching {}: {:?}", $route, e);
                ErrorInternalServerError("")
            })?;

            let mut res = actix_web::HttpResponse::Ok();
            headers.into_iter().for_each(|header| {
                res.insert_header(header);
            });
            let res = res.body(body);

            Ok(res)
        }
    };
}

create_passthrough_route_handler!(manifest, "manifest.json");
create_passthrough_route_handler!(service_worker, "service-worker.js");
create_passthrough_route_handler!(icon, "icon.png");
create_passthrough_route_handler!(icon_192, "icon-192x192.png");
create_passthrough_route_handler!(icon_512, "icon-512x512.png");
