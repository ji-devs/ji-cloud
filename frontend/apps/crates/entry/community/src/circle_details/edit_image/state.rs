use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{circle::Circle, image::ImageId};
use web_sys::File;

use super::super::callbacks::EditCirclesCallbacks;

pub struct EditImage {
    circle: Circle,
    pub callbacks: EditCirclesCallbacks,
    pub(super) image: Mutable<Option<ImageIfOrFile>>,
    pub loader: AsyncLoader,
}

impl EditImage {
    pub fn new(circle: Circle, callbacks: EditCirclesCallbacks) -> Rc<Self> {
        Rc::new(Self {
            callbacks,
            image: Mutable::new(Some(ImageIfOrFile::ImageId(circle.image))),
            circle,
            loader: AsyncLoader::new(),
        })
    }

    pub fn get_circle_update_data(&self, image_id: ImageId) -> Circle {
        let mut circle = self.circle.clone();

        circle.image = image_id;

        circle
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(super) enum ImageIfOrFile {
    ImageId(ImageId),
    File(File),
}
