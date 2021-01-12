use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{Category, CategoryId};

use paperclip::v2::{
    models::{DataType, DefaultSchemaRaw},
    schema::Apiv2Schema,
};

impl Apiv2Schema for Category {
    const NAME: Option<&'static str> = Some("Category");
    const DESCRIPTION: &'static str = "Over-the-wire representation of a category. NOTE: there's a optional `children` category array, it's currently missing from this schema.";

    fn raw_schema() -> DefaultSchemaRaw {
        let mut schema = DefaultSchemaRaw {
            name: Self::NAME.map(str::to_owned),
            description: Some(Self::DESCRIPTION.to_owned()),
            data_type: Some(DataType::Object),
            ..Default::default()
        };

        schema.properties.insert(
            "id".to_owned(),
            Box::new(DefaultSchemaRaw {
                description: Some("The category's id.".to_owned()),
                ..CategoryId::raw_schema()
            }),
        );

        schema.required.insert("id".to_owned());

        schema.properties.insert(
            "name".to_owned(),
            Box::new(DefaultSchemaRaw {
                description: Some("The category's name.".to_owned()),
                ..String::raw_schema()
            }),
        );

        schema.required.insert("name".to_owned());

        // children is _not_ required. But also skip it until we understand _how_ to impl it
        // schema.properties.insert(
        //     "children".to_owned(),
        //     Box::new(DefaultSchemaRaw {
        //         description: Some("The category's children, if any".to_owned()),
        //         data_type: Some(DataType::Array),
        //         items: Some(T::schema_with_ref().into()),
        //         ..Default::default()
        //     }),
        // );

        schema.properties.insert(
            "created_at".to_owned(),
            Box::new(DefaultSchemaRaw {
                description: Some("When the category was initially created.".to_owned()),
                ..DateTime::<Utc>::raw_schema()
            }),
        );

        schema.required.insert("created_at".to_owned());

        schema.properties.insert(
            "updated_at".to_owned(),
            Box::new(DefaultSchemaRaw {
                description: Some("When the category was last updated.".to_owned()),
                ..Option::<DateTime<Utc>>::raw_schema()
            }),
        );

        schema.required.insert("updated_at".to_owned());

        schema.properties.insert(
            "image_count".to_owned(),
            Box::new(DefaultSchemaRaw {
                description: Some("The number of images associated with the category.".to_owned()),
                ..u64::raw_schema()
            }),
        );

        schema.required.insert("image_count".to_owned());

        schema.properties.insert(
            "jig_count".to_owned(),
            Box::new(DefaultSchemaRaw {
                description: Some("The number of JIGs associated with the category.".to_owned()),
                ..u64::raw_schema()
            }),
        );

        schema.required.insert("jig_count".to_owned());

        schema
    }
}

impl Apiv2Schema for super::GetCategoryRequest {
    const NAME: Option<&'static str> = Some("GetCategoryRequest");
    const DESCRIPTION: &'static str = "Request to get a tree of categories.";

    fn raw_schema() -> DefaultSchemaRaw {
        let mut schema = DefaultSchemaRaw {
            name: Self::NAME.map(str::to_owned),
            description: Some(Self::DESCRIPTION.to_owned()),
            data_type: Some(DataType::Object),
            ..Default::default()
        };

        schema.properties.insert(
            "ids".to_owned(),
            Box::new(DefaultSchemaRaw {
                description: Some("The exact ids to be included in the response.".to_owned()),
                ..Vec::<Uuid>::raw_schema()
            }),
        );

        schema.properties.insert(
            "scope".to_owned(),
            Box::new(DefaultSchemaRaw {
                description: Some("Which direction to follow the tree.".to_owned()),
                ..Option::<super::CategoryTreeScope>::raw_schema()
            }),
        );

        schema
    }
}
