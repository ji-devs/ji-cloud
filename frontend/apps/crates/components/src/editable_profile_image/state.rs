use super::EditableProfileImageConfig;
use dominator::clone;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Mutable, ReadOnlyMutable, SignalExt};
use shared::domain::image::ImageId;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::File;

pub struct EditableProfileImage {
    pub profile_image: ReadOnlyMutable<Option<ImageId>>,
    pub given_name: ReadOnlyMutable<String>,
    pub family_name: ReadOnlyMutable<String>,
    pub config: EditableProfileImageConfig,
    pub(super) image: Mutable<Option<ImageIdOrFile>>,
    pub loader: AsyncLoader,
    pub popup_open: Mutable<bool>,
}

impl EditableProfileImage {
    pub fn new(
        profile_image: ReadOnlyMutable<Option<ImageId>>,
        given_name: ReadOnlyMutable<String>,
        family_name: ReadOnlyMutable<String>,
        config: EditableProfileImageConfig,
    ) -> Rc<Self> {
        let image = Mutable::new(None);

        // wait for user's profile image to be fetched before rendering
        spawn_local(clone!(image, profile_image => async move {
            profile_image.signal().for_each(|profile_image| {
                image.set(profile_image.map(|profile_image| ImageIdOrFile::ImageId(profile_image)));
                async {

                }
             }).await;
        }));

        Rc::new(Self {
            config,
            profile_image,
            given_name,
            family_name,
            image,
            loader: AsyncLoader::new(),
            popup_open: Mutable::new(false),
        })
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(super) enum ImageIdOrFile {
    ImageId(ImageId),
    File(File),
}
