use std::rc::Rc;

use components::image::upload::upload_image;
use dominator::clone;
use shared::{
    api::endpoints,
    domain::image::{user::UserImageCreateRequest, ImageId, ImageSize},
    media::MediaLibrary,
};
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};
use web_sys::File;

use super::{EditImage, ImageIfOrFile};

impl EditImage {
    pub fn apply_changes(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let image_id = match state.image.get_cloned() {
                None => None,
                Some(ImageIfOrFile::ImageId(image_id)) => Some(image_id),
                Some(ImageIfOrFile::File(image_file)) => {
                    Some(upload_profile_image(image_file).await.unwrap_ji())
                },
            };

            let user = state.get_user_profile_from_image(image_id);

            (state.callbacks.save_changes)(user);
        }));
    }
}

async fn upload_profile_image(file: File) -> Result<ImageId, Box<dyn std::error::Error>> {
    let req = UserImageCreateRequest {
        size: ImageSize::UserProfile,
    };

    let image_id = endpoints::image::user::Create::api_with_auth(Some(req))
        .await
        .map_err(|_err| "Error creating image in db")?
        .id;

    upload_image(image_id, MediaLibrary::User, &file, None)
        .await
        .map_err(|_err| "Error uploading image")?;

    Ok(image_id)
}
