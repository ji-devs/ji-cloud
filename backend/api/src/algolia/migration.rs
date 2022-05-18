use algolia::{
    model::attribute::{Attribute, FacetAttribute, SearchableAttributes},
    request::SetSettings,
};

use sqlx::PgConnection;

use hashfn::hashfn;

pub const JIG_INDEX: &str = "jig_index";
pub const MEDIA_INDEX: &str = "media_index";
pub const COURSE_INDEX: &str = "course_index";

#[hashfn(MEDIA_HASH)]
pub(crate) async fn media_index(
    txn: &mut PgConnection,
    client: &super::Inner,
    media_index: &str,
) -> anyhow::Result<()> {
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

    client.set_settings(media_index, &settings).await?;

    sqlx::query!(r#"update algolia_index_settings set updated_at = now(), index_hash = $1 where index_name = $2"#, MEDIA_HASH, MEDIA_INDEX).execute(txn).await?;

    Ok(())
}

#[hashfn(JIG_HASH)]
pub(crate) async fn jig_index(
    txn: &mut PgConnection,
    client: &super::Inner,
    jig_index: &str,
) -> anyhow::Result<()> {
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

    client.set_settings(jig_index, &settings).await?;

    sqlx::query!(r#"update algolia_index_settings set updated_at = now(), index_hash = $1 where index_name = $2"#, JIG_HASH, JIG_INDEX).execute(txn).await?;

    Ok(())
}

#[hashfn(COURSE_HASH)]
pub(crate) async fn course_index(
    txn: &mut PgConnection,
    client: &super::Inner,
    course_index: &str,
) -> anyhow::Result<()> {
    let settings = SetSettings {
        searchable_attributes: Some(
            SearchableAttributes::build()
                .single(Attribute("name".to_owned()))
                .single(Attribute("author_name".to_owned()))
                .single(Attribute("translated_keywords".to_owned()))
                .single(Attribute("description".to_owned()))
                .single(Attribute("category_names".to_owned()))
                .single(Attribute("translated_description".to_owned()))
                .single(Attribute("resource_type_names".to_owned()))
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

    client.set_settings(course_index, &settings).await?;

    sqlx::query!(r#"update algolia_index_settings set updated_at = now(), index_hash = $1 where index_name = $2"#, COURSE_HASH, COURSE_INDEX).execute(txn).await?;

    Ok(())
}
