use core::settings::AlgoliaSettings;
use algolia::{
    request::{PartialUpdateQuery, SearchQuery},
    response::SearchResponse,
    Client as Inner,
};
use serde::{Deserialize, Serialize};
use shared::domain::image::ImageId;
use uuid::Uuid;

macro_rules! with_client {
    ($client:expr; $ret:expr) => {{
        let c: &Inner = match &$client {
            Some(c) => c,
            None => return Ok($ret),
        };

        c
    }};

    ($client:expr) => {{
        let c: &Inner = match &$client {
            Some(c) => c,
            None => return Ok(()),
        };

        c
    }};
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

#[derive(Deserialize, Debug)]
pub struct Empty {}

impl AlgoliaClient {
    pub fn new(settings: Option<AlgoliaSettings>) -> anyhow::Result<Self> {
        let inner = if let Some(settings) = settings {
            let app_id = algolia::AppId::new(settings.application_id);
            let api_key = algolia::ApiKey(settings.key);

            Some(Inner::new(app_id, api_key).map_err(|e| anyhow::anyhow!(e))?)
        } else {
            None
        };

        Ok(Self { inner })
    }

    // todo: return ImageId (can't because of repr issues in sqlx)
    pub async fn search_image(&self, query: &str) -> anyhow::Result<Vec<Uuid>> {
        let client = with_client!(self.inner; vec![]);

        let results: SearchResponse<Empty> = client
            .search(
                "image",
                SearchQuery {
                    query: Some(query.to_owned()),
                    get_ranking_info: true,
                    ..SearchQuery::default()
                },
            )
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        results
            .hits
            .into_iter()
            .map(|hit| hit.object_id.parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub async fn put_image(&self, ImageId(id): ImageId, img: Image<'_>) -> anyhow::Result<()> {
        with_client!(self.inner)
            .add_or_update_object("image", &id.to_string(), &img)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

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
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(())
    }

    pub async fn delete_image(&self, ImageId(id): ImageId) -> anyhow::Result<()> {
        with_client!(self.inner)
            .delete_object("image", &id.to_string())
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(())
    }
}
