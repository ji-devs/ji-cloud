use std::rc::Rc;

use components::image::upload::upload_user_image;
use dominator::clone;
use shared::domain::image::{ImageId, ImageSize};
use utils::unwrap::UnwrapJiExt;
use web_sys::File;

use super::{EditImage, ImageIfOrFile};

impl EditImage {
    pub fn apply_changes(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let image_id = match state.image.get_cloned() {
                None => unreachable!("Circle needs an image"),
                Some(ImageIfOrFile::ImageId(image_id)) => image_id,
                Some(ImageIfOrFile::File(image_file)) => {
                    upload_profile_image(image_file).await.unwrap_ji()
                },
            };

            let circle = state.get_circle_update_data(image_id);

            (state.callbacks.save_changes)(circle);
        }));
    }
}

async fn upload_profile_image(file: File) -> Result<ImageId, Box<dyn std::error::Error>> {
    let image_id = upload_user_image(ImageSize::UserProfile, &file, None)
        .await
        .map_err(|_err| "Error uploading image")?;

    Ok(image_id)
}
