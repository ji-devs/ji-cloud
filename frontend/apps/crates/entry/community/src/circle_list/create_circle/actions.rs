use std::rc::Rc;

use components::image::upload::upload_image;
use dominator::clone;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{
        circle::{Circle, CircleCreateRequest},
        image::{user::UserImageCreateRequest, ImageId, ImageSize},
    },
    error::EmptyError,
    media::MediaLibrary,
};
use utils::{
    prelude::{api_with_auth, ApiEndpointExt},
    unwrap::UnwrapJiExt,
};
use web_sys::File;

use super::CreateCircle;

impl CreateCircle {
    pub fn save_circles(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            match state.save_circle_async().await {
                Ok(circle) => {
                    let mut circles = state.circle_list_state.circles.lock_mut();
                    if let Some(circles) = &mut *circles {
                        circles.insert(0, circle);
                    }
                    state.circle_list_state.create_popup_open.set(false);
                },
                Err(_) => todo!(),
            }
        }));
    }

    async fn save_circle_async(self: &Rc<Self>) -> anyhow::Result<Circle> {
        let state = self;

        let image_id = upload_circle_image(state.image.get_cloned().unwrap_ji()).await?;

        let req = CircleCreateRequest {
            display_name: state.name.get_cloned().unwrap_or_default(),
            description: state.description.get_cloned().unwrap_or_default(),
            image: Some(image_id),
        };

        let id = endpoints::circle::Create::api_with_auth(Some(req))
            .await?
            .id;

        let path = endpoints::circle::Get::PATH.replace("{id}", &id.0.to_string());
        let circle =
            api_with_auth::<Circle, EmptyError, ()>(&path, endpoints::circle::Get::METHOD, None)
                .await?;

        Ok(circle)
    }
}

async fn upload_circle_image(file: File) -> anyhow::Result<ImageId> {
    let req = UserImageCreateRequest {
        size: ImageSize::UserProfile,
    };

    let image_id = endpoints::image::user::Create::api_with_auth(Some(req))
        .await
        .map_err(|_err| anyhow::anyhow!("Error creating image in db"))?
        .id;

    upload_image(image_id, MediaLibrary::User, &file, None)
        .await
        .map_err(|_err| anyhow::anyhow!("Error uploading image"))?;

    Ok(image_id)
}
