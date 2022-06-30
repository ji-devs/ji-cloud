use std::rc::Rc;

use components::image::upload::upload_image;
use dominator::clone;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{
        badge::{Badge, BadgeCreateRequest},
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

use super::CreateBadge;

impl CreateBadge {
    pub fn save_badges(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            match state.save_badge_async().await {
                Ok(badge) => {
                    let mut badges = state.badge_list_state.badges.lock_mut();
                    if let Some(badges) = &mut *badges {
                        badges.insert(0, badge);
                    }
                    state.badge_list_state.create_popup_open.set(false);
                },
                Err(_) => todo!(),
            }
        }));
    }

    async fn save_badge_async(self: &Rc<Self>) -> anyhow::Result<Badge> {
        let state = self;

        upload_badge_image(state.image.get_cloned().unwrap_ji()).await?;

        let req = BadgeCreateRequest {
            display_name: state.name.get_cloned().unwrap_or_default(),
            description: state.description.get_cloned().unwrap_or_default(),
            thumbnail: url::Url::parse(
                "data:image/gif;base64,R0lGODlhAQABAAAAACH5BAEKAAEALAAAAAABAAEAAAICTAEAOw==",
            )
            .unwrap(),
        };

        let id = endpoints::badge::Create::api_with_auth(Some(req)).await?.id;

        let path = endpoints::badge::Get::PATH.replace("{id}", &id.0.to_string());
        let badge =
            api_with_auth::<Badge, EmptyError, ()>(&path, endpoints::badge::Get::METHOD, None)
                .await?;

        Ok(badge)
    }
}

async fn upload_badge_image(file: File) -> anyhow::Result<ImageId> {
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
