use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{image::ImageId, user::UserProfile};
use web_sys::File;

use crate::member_details::callbacks::EditProfileCallbacks;

pub struct EditImage {
    user: UserProfile,
    pub callbacks: EditProfileCallbacks,
    pub(super) image: Mutable<Option<ImageIfOrFile>>,
    pub loader: AsyncLoader,
}

impl EditImage {
    pub fn new(user: UserProfile, callbacks: EditProfileCallbacks) -> Rc<Self> {
        let image = Mutable::new(
            user.profile_image
                .map(|image_id| ImageIfOrFile::ImageId(image_id)),
        );
        Rc::new(Self {
            callbacks,
            image,
            user,
            loader: AsyncLoader::new(),
        })
    }

    pub fn get_user_profile_from_image(&self, image_id: Option<ImageId>) -> UserProfile {
        let mut user = self.user.clone();

        user.profile_image = image_id;

        user
    }
}

#[derive(Clone)]
pub(super) enum ImageIfOrFile {
    ImageId(ImageId),
    File(File),
}
