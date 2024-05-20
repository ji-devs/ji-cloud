// TODO: move some of this logic to the algolia module and/or algolia crate

use std::{pin::Pin, sync::RwLock};

use futures::{future::Shared, Future, FutureExt};
use serde::Deserialize;
use serde_json::json;
use shared::domain::jig::JigId;

use crate::service::ServiceData;

static TRENDING_CACHE: RwLock<TrendingCacheState> = RwLock::new(TrendingCacheState::Init);

const REFRESH_INTERVAL_SECONDS: i64 = 60 * 60; // 1 hour
const TRENDING_ITEMS: &str = "trending-items";
const TRENDING_JIG_COUNT: u32 = 20;

#[derive(Clone)]
enum TrendingCacheState {
    Init,
    Loading(Shared<Pin<Box<dyn Future<Output = Result<Vec<JigId>, ()>> + Send + Sync>>>),
    Loaded(TrendingCache),
}

#[derive(Clone)]
struct TrendingCache {
    expires_at: chrono::DateTime<chrono::Utc>,
    jigs: Vec<JigId>,
}

#[derive(Deserialize, Debug)]
struct Response {
    results: Vec<ResponseResult>,
}

#[derive(Deserialize, Debug)]
struct ResponseResult {
    hits: Vec<Jig>,
}

#[derive(Deserialize, Debug)]
struct Jig {
    #[serde(rename = "objectID")]
    pub object_id: JigId,
}

async fn fetch_trending_algolia(
    algolia: ServiceData<crate::algolia::Client>,
) -> Result<Vec<JigId>, ()> {
    let client = reqwest::Client::new();
    let mut res = client
        .post(&format!(
            "https://{}-dsn.algolia.net/1/indexes/*/recommendations",
            algolia.application_id()
        ))
        .header("x-algolia-api-key", algolia.search_key())
        .header("x-algolia-application-id", algolia.application_id())
        .json(&json!({
            "requests": [{
                "indexName": algolia.jig_index(),
                "model": TRENDING_ITEMS,
                "threshold": 0,
                "maxRecommendations": TRENDING_JIG_COUNT,
            }]
        }))
        .send()
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            ()
        })?
        .json::<Response>()
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            ()
        })?;

    let res = res
        .results
        .remove(0)
        .hits
        .into_iter()
        .map(|jig| jig.object_id)
        .collect::<Vec<JigId>>();
    Ok(res)
}

pub async fn get_trending(
    algolia: ServiceData<crate::algolia::Client>,
) -> anyhow::Result<Vec<JigId>> {
    // TODO: instead of unwrapping the lock, reset the value and clear the locks poison
    // check the following example https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.clear_poison
    let cache = TRENDING_CACHE.read().unwrap().clone();
    Ok(match cache {
        TrendingCacheState::Loaded(l) if l.expires_at > chrono::Utc::now() => l.jigs.clone(),
        TrendingCacheState::Loading(future) => future
            .await
            .map_err(|_| anyhow::Error::msg("failed to fetch trending jigs from algolia"))?,
        _ => {
            let future = Box::pin(fetch_trending_algolia(algolia))
                as Pin<Box<dyn Future<Output = _> + Send + Sync + 'static>>;
            let future = future.shared();
            *TRENDING_CACHE.write().unwrap() = TrendingCacheState::Loading(future.clone());
            let jigs = future
                .await
                .map_err(|_| anyhow::Error::msg("failed to fetch trending jigs from algolia"))?;
            let expires_at = expiration_time();
            *TRENDING_CACHE.write().unwrap() = TrendingCacheState::Loaded(TrendingCache {
                expires_at,
                jigs: jigs.clone(),
            });
            jigs
        }
    })
}

fn expiration_time() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now() + chrono::Duration::seconds(REFRESH_INTERVAL_SECONDS)
}
