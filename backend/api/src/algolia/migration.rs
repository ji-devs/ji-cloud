use algolia::{
    model::attribute::{Attribute, FacetAttribute, SearchableAttributes},
    request::SetSettings,
};
use futures::future::BoxFuture;

#[derive(Copy, Clone, Debug)]
pub enum ResyncKind {
    None,
    Complete,
}

async fn media_index<'a>(
    client: &'a super::Inner,
    media_index: &'a str,
    _jig_index: &'a str,
    _course_index: &'a str,
) -> BoxFuture<'a, anyhow::Result<()>> {
    let settings = SetSettings {
        searchable_attributes: Some(
            SearchableAttributes::build()
                .single(Attribute("name".to_owned()))
                .single(Attribute("description".to_owned()))
                .single(Attribute("translated_description".to_owned()))
                .finish(),
        ),
        attributes_for_faceting: Some(vec![
            FacetAttribute::filter_only(Attribute("_tags".to_owned())),
            FacetAttribute::filter_only(Attribute("affiliations".to_owned())),
            FacetAttribute::filter_only(Attribute("categories".to_owned())),
            FacetAttribute::filter_only(Attribute("age_ranges".to_owned())),
            FacetAttribute::filter_only(Attribute("image_tags".to_owned())),
            FacetAttribute::filter_only(Attribute("media_kind".to_owned())),
            FacetAttribute::filter_only(Attribute("media_subkind".to_owned())),
            FacetAttribute::filter_only(Attribute("styles".to_owned())),
        ]),
    };

    Box::pin(async move {
        client.set_settings(media_index, &settings).await?;
        Ok(())
    })
}

async fn jig_index<'a>(
    client: &'a super::Inner,
    _media_index: &'a str,
    jig_index: &'a str,
    _course_index: &'a str,
) -> BoxFuture<'a, anyhow::Result<()>> {
    let settings = SetSettings {
        searchable_attributes: Some(
            SearchableAttributes::build()
                .single(Attribute("name".to_owned()))
                .single(Attribute("author_name".to_owned()))
                .single(Attribute("translated_keywords".to_owned()))
                .single(Attribute("description".to_owned()))
                .multi(vec![
                    Attribute("category_names".to_owned()),
                    Attribute("translated_description".to_owned()),
                    Attribute("resource_type_names".to_owned()),
                ])
                .single(Attribute("language".to_owned()))
                .single(Attribute("other_keywords".to_owned()))
                .finish(),
        ),
        attributes_for_faceting: Some(vec![
            FacetAttribute::filter_only(Attribute("age_ranges".to_owned())),
            FacetAttribute::filter_only(Attribute("_tags".to_owned())),
            FacetAttribute::filter_only(Attribute("jig_focus".to_owned())),
            FacetAttribute::filter_only(Attribute("author_id".to_owned())),
            FacetAttribute::filter_only(Attribute("author_name".to_owned())),
            FacetAttribute::filter_only(Attribute("blocked".to_owned())),
            FacetAttribute::filter_only(Attribute("categories".to_owned())),
            FacetAttribute::filter_only(Attribute("affiliations".to_owned())),
            FacetAttribute::filter_only(Attribute("categories".to_owned())),
            FacetAttribute::filter_only(Attribute("resource_types".to_owned())),
            FacetAttribute::filter_only(Attribute("language".to_owned())),
        ]),
    };

    Box::pin(async move {
        client.set_settings(jig_index, &settings).await?;
        Ok(())
    })
}

async fn course_index<'a>(
    client: &'a super::Inner,
    _media_index: &'a str,
    _jig_index: &'a str,
    course_index: &'a str,
) -> BoxFuture<'a, anyhow::Result<()>> {
    let settings = SetSettings {
        searchable_attributes: Some(
            SearchableAttributes::build()
                .single(Attribute("name".to_owned()))
                .single(Attribute("author_name".to_owned()))
                .single(Attribute("translated_keywords".to_owned()))
                .single(Attribute("description".to_owned()))
                .multi(vec![
                    Attribute("category_names".to_owned()),
                    Attribute("translated_description".to_owned()),
                    Attribute("resource_type_names".to_owned()),
                ])
                .single(Attribute("language".to_owned()))
                .single(Attribute("other_keywords".to_owned()))
                .finish(),
        ),
        attributes_for_faceting: Some(vec![
            FacetAttribute::filter_only(Attribute("age_ranges".to_owned())),
            FacetAttribute::filter_only(Attribute("_tags".to_owned())),
            FacetAttribute::filter_only(Attribute("author_id".to_owned())),
            FacetAttribute::filter_only(Attribute("author_name".to_owned())),
            FacetAttribute::filter_only(Attribute("blocked".to_owned())),
            FacetAttribute::filter_only(Attribute("categories".to_owned())),
            FacetAttribute::filter_only(Attribute("affiliations".to_owned())),
            FacetAttribute::filter_only(Attribute("categories".to_owned())),
            FacetAttribute::filter_only(Attribute("resource_types".to_owned())),
            FacetAttribute::filter_only(Attribute("language".to_owned())),
        ]),
    };

    Box::pin(async move {
        client.set_settings(course_index, &settings).await?;
        Ok(())
    })
}

#[ignore]
#[inline(always)]
fn empty<'a>(
    _client: &'a super::Inner,
    _media_index: &'a str,
    _jig_index: &'a str,
    _course_index: &'a str,
) -> BoxFuture<'a, anyhow::Result<()>> {
    Box::pin(futures::future::ok(()))
}

pub type MigrateFunction =
    for<'a> fn(&'a super::Inner, &'a str, &'a str, &'a str) -> BoxFuture<'a, anyhow::Result<()>>;

pub const INDEXING_MIGRATIONS: &[(ResyncKind, MigrateFunction)] = &[
    (ResyncKind::None, media_index),
    (ResyncKind::None, jig_index),
    (ResyncKind::None, course_index),
];

pub const INDEX_VERSION: i16 = INDEXING_MIGRATIONS.len() as i16;
