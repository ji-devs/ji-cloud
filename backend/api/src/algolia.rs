use algolia::{
    request::{PartialUpdateQuery, SearchQuery},
    response::SearchResponse,
    Client as Inner,
};
use core::settings::AlgoliaSettings;
use serde::Serialize;
use shared::domain::image::ImageId;
use std::convert::TryInto;
use uuid::Uuid;

macro_rules! with_client {
    ($client:expr; $ret:expr) => {{
        let c: &Inner = match &$client {
            Some(c) => c,
            None => return Ok($ret),
        };

        c
    }};

    ($client:expr) => { with_client!($client; ()) };
}

#[derive(Serialize, Debug)]
pub struct Image<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Serialize, Default, Debug)]
pub struct ImageUpdate<'a> {
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
}

#[derive(Clone)]
pub struct AlgoliaClient {
    inner: Option<Inner>,
}

impl AlgoliaClient {
    pub fn new(settings: Option<AlgoliaSettings>) -> anyhow::Result<Self> {
        let inner = if let Some(settings) = settings {
            let app_id = algolia::AppId::new(settings.application_id);
            let api_key = algolia::ApiKey(settings.key);

            Some(Inner::new(app_id, api_key)?)
        } else {
            None
        };

        Ok(Self { inner })
    }

    // todo: return ImageId (can't because of repr issues in sqlx)
    pub async fn search_image(
        &self,
        query: &str,
        page: Option<u32>,
    ) -> anyhow::Result<(Vec<Uuid>, u32)> {
        let client = with_client!(self.inner; (vec![], 0));

        let mut filters = algolia::filter::AndFilter { filters: vec![] };

        

        let results: SearchResponse = client
            .search(
                "image",
                SearchQuery {
                    query: Some(query),
                    page,
                    get_ranking_info: true,
                    filters: Some(filters),
                    hits_per_page: None,
                },
            )
            .await?;

        let pages = results.page_count.try_into()?;

        let results = results
            .hits
            .into_iter()
            .map(|hit| hit.object_id.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok((results, pages))
    }

    pub async fn put_image(&self, ImageId(id): ImageId, img: Image<'_>) -> anyhow::Result<()> {
        with_client!(self.inner)
            .add_or_update_object("image", &id.to_string(), &img)
            .await?;

        Ok(())
    }

    pub async fn update_image(
        &self,
        ImageId(id): ImageId,
        req: ImageUpdate<'_>,
    ) -> anyhow::Result<()> {
        with_client!(self.inner)
            .partially_update_object(
                "image",
                &id.to_string(),
                &req,
                &PartialUpdateQuery {
                    create_if_not_exists: false,
                },
            )
            .await?;

        Ok(())
    }

    pub async fn delete_image(&self, ImageId(id): ImageId) -> anyhow::Result<()> {
        with_client!(self.inner)
            .delete_object("image", &id.to_string())
            .await?;

        Ok(())
    }
}
