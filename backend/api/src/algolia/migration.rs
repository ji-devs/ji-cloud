use algolia::{
    model::attribute::{Attribute, FacetAttribute, SearchableAttributes},
    request::SetSettings,
};

use sqlx::PgConnection;

use hashfn::hashfn;

pub const JIG_INDEX: &str = "jig_index";
pub const MEDIA_INDEX: &str = "media_index";
pub const PLAYLIST_INDEX: &str = "playlist_index";
pub const CIRCLE_INDEX: &str = "circle_index";
pub const PUBLIC_USER_INDEX: &str = "public_user_index";
pub const USER_INDEX: &str = "user_index";
pub const RESOURCE_INDEX: &str = "resource_index";
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
                .single(Attribute("translated_description".to_owned()))
                .single(Attribute("translated_name".to_owned()))
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
                .single(Attribute("category_names".to_owned()))
                .single(Attribute("translated_description".to_owned()))
                .single(Attribute("resource_type_names".to_owned()))
                .single(Attribute("language".to_owned()))
                .single(Attribute("other_keywords".to_owned()))
                .single(Attribute("translated_name".to_owned()))
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
            FacetAttribute::filter_only(Attribute("rating".to_owned())),
        ]),
    };

    client.set_settings(jig_index, &settings).await?;

    sqlx::query!(r#"update algolia_index_settings set updated_at = now(), index_hash = $1 where index_name = $2"#, JIG_HASH, JIG_INDEX).execute(txn).await?;

    Ok(())
}

#[hashfn(PLAYLIST_HASH)]
pub(crate) async fn playlist_index(
    txn: &mut PgConnection,
    client: &super::Inner,
    playlist_index: &str,
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
                .single(Attribute("translated_name".to_owned()))
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
            FacetAttribute::filter_only(Attribute("rating".to_owned())),
        ]),
    };

    client.set_settings(playlist_index, &settings).await?;

    sqlx::query!(r#"update algolia_index_settings set updated_at = now(), index_hash = $1 where index_name = $2"#, PLAYLIST_HASH, PLAYLIST_INDEX).execute(txn).await?;

    Ok(())
}

#[hashfn(CIRCLE_HASH)]
pub(crate) async fn circle_index(
    txn: &mut PgConnection,
    client: &super::Inner,
    circle_index: &str,
) -> anyhow::Result<()> {
    let settings = SetSettings {
        searchable_attributes: Some(
            SearchableAttributes::build()
                .single(Attribute("name".to_owned()))
                .single(Attribute("creator_name".to_owned()))
                .single(Attribute("description".to_owned()))
                .single(Attribute("language".to_owned()))
                .single(Attribute("member_count".to_owned()))
                .finish(),
        ),
        attributes_for_faceting: Some(vec![
            FacetAttribute::filter_only(Attribute("creator_id".to_owned())),
            FacetAttribute::filter_only(Attribute("creator_name".to_owned())),
            FacetAttribute::filter_only(Attribute("language".to_owned())),
        ]),
    };

    client.set_settings(circle_index, &settings).await?;

    sqlx::query!(r#"update algolia_index_settings set updated_at = now(), index_hash = $1 where index_name = $2"#, CIRCLE_HASH, CIRCLE_INDEX).execute(txn).await?;

    Ok(())
}

#[hashfn(USER_HASH)]
pub(crate) async fn user_index(
    txn: &mut PgConnection,
    client: &super::Inner,
    public_user_index: &str,
) -> anyhow::Result<()> {
    let settings = SetSettings {
        searchable_attributes: Some(
            SearchableAttributes::build()
                .single(Attribute("username".to_owned()))
                .single(Attribute("given_name".to_owned()))
                .single(Attribute("family_name".to_owned()))
                .single(Attribute("email".to_owned()))
                .single(Attribute("language".to_owned()))
                .single(Attribute("organization".to_owned()))
                .single(Attribute("country".to_owned()))
                .single(Attribute("state".to_owned()))
                .single(Attribute("city".to_owned()))
                .finish(),
        ),
        attributes_for_faceting: Some(vec![
            FacetAttribute::filter_only(Attribute("creator_id".to_owned())),
            FacetAttribute::filter_only(Attribute("creator_name".to_owned())),
            FacetAttribute::filter_only(Attribute("bio".to_owned())),
            FacetAttribute::filter_only(Attribute("languages_spoken".to_owned())),
            FacetAttribute::filter_only(Attribute("organization".to_owned())),
            FacetAttribute::filter_only(Attribute("persona".to_owned())),
            FacetAttribute::filter_only(Attribute("location".to_owned())),
        ]),
    };

    client.set_settings(public_user_index, &settings).await?;

    sqlx::query!(r#"update algolia_index_settings set updated_at = now(), index_hash = $1 where index_name = $2"#, USER_HASH, USER_INDEX).execute(txn).await?;

    Ok(())
}

#[hashfn(PUBLIC_USER_HASH)]
pub(crate) async fn public_user_index(
    txn: &mut PgConnection,
    client: &super::Inner,
    public_user_index: &str,
) -> anyhow::Result<()> {
    let settings = SetSettings {
        searchable_attributes: Some(
            SearchableAttributes::build()
                .single(Attribute("username".to_owned()))
                .single(Attribute("name".to_owned()))
                .single(Attribute("bio".to_owned()))
                .single(Attribute("languages_spoken".to_owned()))
                .single(Attribute("organization".to_owned()))
                .single(Attribute("persona".to_owned()))
                .single(Attribute("location".to_owned()))
                .finish(),
        ),
        attributes_for_faceting: Some(vec![
            FacetAttribute::filter_only(Attribute("creator_id".to_owned())),
            FacetAttribute::filter_only(Attribute("creator_name".to_owned())),
            FacetAttribute::filter_only(Attribute("bio".to_owned())),
            FacetAttribute::filter_only(Attribute("languages_spoken".to_owned())),
            FacetAttribute::filter_only(Attribute("organization".to_owned())),
            FacetAttribute::filter_only(Attribute("persona".to_owned())),
            FacetAttribute::filter_only(Attribute("location".to_owned())),
        ]),
    };

    client.set_settings(public_user_index, &settings).await?;

    sqlx::query!(r#"update algolia_index_settings set updated_at = now(), index_hash = $1 where index_name = $2"#, PUBLIC_USER_HASH, PUBLIC_USER_INDEX).execute(txn).await?;

    Ok(())
}

#[hashfn(RESOURCE_HASH)]
pub(crate) async fn resource_index(
    txn: &mut PgConnection,
    client: &super::Inner,
    resource_index: &str,
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
                .single(Attribute("translated_name".to_owned()))
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
            FacetAttribute::filter_only(Attribute("rating".to_owned())),
        ]),
    };

    client.set_settings(resource_index, &settings).await?;

    sqlx::query!(r#"update algolia_index_settings set updated_at = now(), index_hash = $1 where index_name = $2"#, RESOURCE_HASH, RESOURCE_INDEX).execute(txn).await?;

    Ok(())
}

#[hashfn(COURSE_HASH)]
pub(crate) async fn course_index(
    txn: &mut PgConnection,
    client: &super::Inner,
    resource_index: &str,
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
                .single(Attribute("translated_name".to_owned()))
                .single(Attribute("units".to_owned()))
                .finish(),
        ),
        attributes_for_faceting: Some(vec![
            FacetAttribute::filter_only(Attribute("age_ranges".to_owned())),
            FacetAttribute::filter_only(Attribute("_tags".to_owned())),
            FacetAttribute::filter_only(Attribute("author_id".to_owned())),
            FacetAttribute::filter_only(Attribute("author_name".to_owned())),
            FacetAttribute::filter_only(Attribute("categories".to_owned())),
            FacetAttribute::filter_only(Attribute("resource_types".to_owned())),
            FacetAttribute::filter_only(Attribute("language".to_owned())),
        ]),
    };

    client.set_settings(resource_index, &settings).await?;

    sqlx::query!(r#"update algolia_index_settings set updated_at = now(), index_hash = $1 where index_name = $2"#, COURSE_HASH, COURSE_INDEX).execute(txn).await?;

    Ok(())
}
