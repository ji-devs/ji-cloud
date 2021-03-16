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

fn bad_batch_object<'a>(
    client: &'a super::Inner,
    index: &'a str,
) -> BoxFuture<'a, anyhow::Result<()>> {
    Box::pin(async move {
        client.delete_object(index, "batch").await?;
        Ok(())
    })
}

fn set_searchable_fields_v1<'a>(
    client: &'a super::Inner,
    index: &'a str,
) -> BoxFuture<'a, anyhow::Result<()>> {
    // superceeded by `set_searchable_fields_v2`
    empty(client, index)
}

fn set_attributes_for_faceting_v1<'a>(
    client: &'a super::Inner,
    index: &'a str,
) -> BoxFuture<'a, anyhow::Result<()>> {
    // superceeded by `set_attributes_for_faceting_v3`
    empty(client, index)
}

fn set_searchable_fields_v2<'a>(
    client: &'a super::Inner,
    index: &'a str,
) -> BoxFuture<'a, anyhow::Result<()>> {
    let settings = SetSettings {
        searchable_attributes: Some(
            SearchableAttributes::build()
                .single(Attribute("name".to_owned()))
                .single(Attribute("description".to_owned()))
                .multi(vec![
                    Attribute("category_names".to_owned()),
                    Attribute("style_names".to_owned()),
                    Attribute("age_range_names".to_owned()),
                    Attribute("affiliation_names".to_owned()),
                ])
                .finish(),
        ),
        attributes_for_faceting: None,
    };

    Box::pin(async move {
        client.set_settings(index, &settings).await?;
        Ok(())
    })
}

fn set_attributes_for_faceting_v2<'a>(
    client: &'a super::Inner,
    index: &'a str,
) -> BoxFuture<'a, anyhow::Result<()>> {
    // superceeded by `set_attributes_for_faceting_v3`
    empty(client, index)
}

fn set_attributes_for_faceting_v3<'a>(
    client: &'a super::Inner,
    index: &'a str,
) -> BoxFuture<'a, anyhow::Result<()>> {
    let settings = SetSettings {
        searchable_attributes: None,
        attributes_for_faceting: Some(vec![
            FacetAttribute::filter_only(Attribute("styles".to_owned())),
            FacetAttribute::filter_only(Attribute("age_ranges".to_owned())),
            FacetAttribute::filter_only(Attribute("affiliations".to_owned())),
            FacetAttribute::filter_only(Attribute("categories".to_owned())),
            FacetAttribute::filter_only(Attribute("media_kind".to_owned())),
        ]),
    };

    Box::pin(async move {
        client.set_settings(index, &settings).await?;
        Ok(())
    })
}

#[inline(always)]
fn empty<'a>(_client: &'a super::Inner, _index: &'a str) -> BoxFuture<'a, anyhow::Result<()>> {
    Box::pin(futures::future::ok(()))
}

pub type MigrateFunction =
    for<'a> fn(&'a super::Inner, &'a str) -> BoxFuture<'a, anyhow::Result<()>>;

pub const INDEXING_MIGRATIONS: &[(ResyncKind, MigrateFunction)] = &[
    (ResyncKind::Complete, bad_batch_object),
    (ResyncKind::Complete, set_searchable_fields_v1),
    (ResyncKind::Complete, empty),
    (ResyncKind::Complete, set_attributes_for_faceting_v1),
    (ResyncKind::Complete, set_searchable_fields_v2),
    (ResyncKind::Complete, empty),
    (ResyncKind::Complete, empty),
    (ResyncKind::Complete, set_attributes_for_faceting_v2),
    (ResyncKind::Complete, set_attributes_for_faceting_v3),
];

pub const INDEX_VERSION: i16 = INDEXING_MIGRATIONS.len() as i16;
