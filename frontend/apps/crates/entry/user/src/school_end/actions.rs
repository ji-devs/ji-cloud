use anyhow::anyhow;
use components::image::upload::upload_image;
use dominator::clone;
use shared::{
    api::endpoints,
    domain::{
        billing::{SchoolAccountPath, UpdateSchoolAccountRequest},
        image::{
            user::{UserImageCreatePath, UserImageCreateRequest},
            ImageId, ImageSize,
        },
    },
    media::MediaLibrary,
};

use utils::{
    prelude::ApiEndpointExt,
    routes::{Route, UserRoute},
    unwrap::UnwrapJiExt,
};
use web_sys::File;

use super::state::*;
use std::rc::Rc;

impl SchoolEnd {
    pub fn save(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let image_id = match state.profile_image.get_cloned() {
                Some(profile_image) => Some(upload_logo(profile_image).await.unwrap_ji()),
                None => None
            };

            let req = UpdateSchoolAccountRequest {
                website: state.website.get_cloned().into(),
                profile_image: image_id.into(),
                description: state.description.get_cloned().into(),
                organization_type: state.organization_type.get_cloned().into(),
                ..Default::default()
            };
            endpoints::account::UpdateSchoolAccount::api_with_auth_empty(SchoolAccountPath(state.school_id), Some(req)).await.unwrap_ji();
            dominator::routing::go_to_url(&Route::User(UserRoute::Welcome).to_string());
        }));
    }
}

async fn upload_logo(file: File) -> anyhow::Result<ImageId> {
    let req = UserImageCreateRequest {
        size: ImageSize::UserProfile,
    };

    let image_id = endpoints::image::user::Create::api_with_auth(UserImageCreatePath(), Some(req))
        .await
        .map_err(|_err| anyhow!("Error creating image in db"))?
        .id;

    upload_image(image_id, MediaLibrary::User, &file, None)
        .await
        .map_err(|_err| anyhow!("Error uploading image)"))?;

    Ok(image_id)
}
