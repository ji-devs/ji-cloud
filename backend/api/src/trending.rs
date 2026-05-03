// TODO: move some of this logic to the algolia module and/or algolia crate

use std::{pin::Pin, sync::RwLock};

use futures::{future::Shared, Future, FutureExt};
use serde::Deserialize;
use serde_json::json;
use shared::{config::RemoteTarget, domain::jig::JigId};

use crate::service::ServiceData;

static TRENDING_CACHE: RwLock<TrendingCacheState> = RwLock::new(TrendingCacheState::Init);

const REFRESH_INTERVAL_SECONDS: i64 = 60 * 60; // 1 hour
const TRENDING_ITEMS: &str = "trending-items";
const TRENDING_JIG_COUNT: u32 = 20;
const SANDBOX_MIN_TRENDING_PLAYS: u32 = 5;
const RELEASE_MIN_TRENDING_PLAYS: u32 = 25;

#[derive(Clone)]
enum TrendingCacheState {
    Init,
    Loading(Shared<Pin<Box<dyn Future<Output = Result<Vec<JigId>, ()>> + Send + Sync>>>),
    Loaded(TrendingCache),
}

#[derive(Clone)]
struct TrendingCache {
    expires_at: chrono::DateTime<chrono::Utc>,
    min_plays: u32,
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
    min_plays: u32,
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
                "queryParameters": {
                    "filters": format!("blocked:false AND plays >= {}", min_plays)
                }
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
    remote_target: RemoteTarget,
) -> anyhow::Result<Vec<JigId>> {
    let min_plays = min_trending_plays(remote_target);

    // TODO: instead of unwrapping the lock, reset the value and clear the locks poison
    // check the following example https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.clear_poison
    let cache = TRENDING_CACHE.read().unwrap().clone();
    Ok(match cache {
        TrendingCacheState::Loaded(l)
            if l.expires_at > chrono::Utc::now() && l.min_plays == min_plays =>
        {
            l.jigs.clone()
        }
        TrendingCacheState::Loading(future) => future
            .await
            .map_err(|_| anyhow::Error::msg("failed to fetch trending jigs from algolia"))?,
        _ => {
            let future = Box::pin(fetch_trending_algolia(algolia, min_plays))
                as Pin<Box<dyn Future<Output = _> + Send + Sync + 'static>>;
            let future = future.shared();
            *TRENDING_CACHE.write().unwrap() = TrendingCacheState::Loading(future.clone());
            let jigs = future
                .await
                .map_err(|_| anyhow::Error::msg("failed to fetch trending jigs from algolia"))?;
            let expires_at = expiration_time();
            *TRENDING_CACHE.write().unwrap() = TrendingCacheState::Loaded(TrendingCache {
                expires_at,
                min_plays,
                jigs: jigs.clone(),
            });
            jigs
        }
    })
}

fn min_trending_plays(remote_target: RemoteTarget) -> u32 {
    match remote_target {
        RemoteTarget::Release => RELEASE_MIN_TRENDING_PLAYS,
        RemoteTarget::Local | RemoteTarget::Sandbox => SANDBOX_MIN_TRENDING_PLAYS,
    }
}

fn expiration_time() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now() + chrono::Duration::seconds(REFRESH_INTERVAL_SECONDS)
}
