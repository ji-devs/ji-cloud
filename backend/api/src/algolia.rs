use algolia::{
    filter::AndFilterable,
    filter::BooleanFilter,
    filter::CmpFilter,
    filter::CommonFilter,
    filter::FacetFilter,
    filter::FilterOperator,
    request::{PartialUpdateQuery, SearchQuery},
    response::SearchResponse,
    Client as Inner,
};
use chrono::Utc;
use core::settings::AlgoliaSettings;
use serde::Serialize;
use shared::domain::{
    category::CategoryId, image::ImageId, meta::AffiliationId, meta::AgeRangeId, meta::StyleId,
};
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
#[serde(rename_all = "camelCase")]
pub struct Image<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub styles: &'a [StyleId],
    pub age_ranges: &'a [AgeRangeId],
    pub affiliations: &'a [AffiliationId],
    pub categories: &'a [CategoryId],
}

#[derive(Serialize, Default, Debug)]
pub struct ImageUpdate<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<&'a [StyleId]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_ranges: Option<&'a [AgeRangeId]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliations: Option<&'a [AffiliationId]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<&'a [CategoryId]>,
}

#[derive(Clone)]
pub struct AlgoliaClient {
    inner: Option<Inner>,
}

fn filters_for_ids<T: Into<Uuid> + Copy>(
    filters: &mut Vec<Box<dyn AndFilterable>>,
    facet_name: &str,
    ids: &[T],
) {
    for id in ids.iter().copied() {
        let id: Uuid = id.into();
        filters.push(Box::new(CommonFilter {
            filter: FacetFilter {
                facet_name: facet_name.to_owned(),
                value: id.to_string(),
            },
            invert: false,
        }))
    }
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
        is_premium: Option<bool>,
        is_published: Option<bool>,
        styles: &[StyleId],
        age_ranges: &[AgeRangeId],
        affiliations: &[AffiliationId],
        categories: &[CategoryId],
    ) -> anyhow::Result<(Vec<Uuid>, u32)> {
        let compare_time = Utc::now().timestamp_nanos();

        let client = with_client!(self.inner; (vec![], 0));

        let mut filters = algolia::filter::AndFilter { filters: vec![] };

        if let Some(is_published) = is_published {
            filters.filters.push(Box::new(CommonFilter {
                filter: CmpFilter::new("publishAt".to_owned(), FilterOperator::Le, compare_time),
                invert: !is_published,
            }))
        }

        if let Some(is_premium) = is_premium {
            filters.filters.push(Box::new(CommonFilter {
                filter: BooleanFilter {
                    facet_name: "isPremium".to_owned(),
                    value: is_premium,
                },
                invert: false,
            }))
        }

        filters_for_ids(&mut filters.filters, "styles", styles);
        filters_for_ids(&mut filters.filters, "ageRanges", age_ranges);
        filters_for_ids(&mut filters.filters, "affiliations", affiliations);
        filters_for_ids(&mut filters.filters, "categories", categories);

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
