use algolia::{client::Client as Inner, request::PartialUpdateQuery};
use core::settings::AlgoliaSettings;
use serde::Serialize;
use shared::domain::image::ImageId;

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
            let app_id = algolia::app_id::AppId::new(settings.application_id);
            let api_key = algolia::ApiKey(settings.key);

            Some(Inner::new(app_id, api_key).map_err(|e| anyhow::anyhow!(e))?)
        } else {
            None
        };

        Ok(Self { inner })
    }

    pub async fn put_image(&self, ImageId(id): ImageId, img: Image<'_>) -> anyhow::Result<()> {
        if let Some(client) = &self.inner {
            client
                .add_or_update_object("image", &id.to_string(), &img)
                .await
                .map_err(|e| anyhow::anyhow!(e))?;
        }

        Ok(())
    }

    pub async fn update_image(
        &self,
        ImageId(id): ImageId,
        req: ImageUpdate<'_>,
    ) -> anyhow::Result<()> {
        if let Some(client) = &self.inner {
            client
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
        }

        Ok(())
    }

    pub async fn delete_image(&self, ImageId(id): ImageId) -> anyhow::Result<()> {
        if let Some(client) = &self.inner {
            client
                .delete_object("image", &id.to_string())
                .await
                .map_err(|e| anyhow::anyhow!(e))?;
        }

        Ok(())
    }
}
