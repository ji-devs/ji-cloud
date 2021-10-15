use shared::{
    api::{endpoints, ApiEndpoint},
    domain::image::*,
    error::EmptyError,
};

use utils::prelude::*;

#[derive(Clone, Debug)]
pub struct DateTimeStrings {
    pub publish: String,
    pub created: String,
    pub updated: String,
}

pub async fn load_date_time_strings(id: ImageId) -> DateTimeStrings {
    let path = endpoints::image::Get::PATH.replace("{id}", &id.0.to_string());

    let image =
        api_with_auth::<ImageResponse, EmptyError, ()>(&path, endpoints::image::Get::METHOD, None)
            .await
            .unwrap_ji()
            .metadata;

    DateTimeStrings {
        publish: match image.publish_at {
            Some(x) => x.to_rfc2822(),
            None => String::from("Unpublished"),
        },

        created: image.created_at.to_rfc2822(),

        updated: match image.updated_at {
            Some(x) => x.to_rfc2822(),
            None => String::from("Never updated"),
        },
    }
}
