use wasm_bindgen::prelude::*;
use shared::{
    domain::image::{SearchQuery, SearchResponse, ImageId, GetResponse},
    media::{image_id_to_key, MediaLibraryKind, MediaVariant},
};
use crate::math::RectF64;

pub trait ImageExt {
    fn id(&self) -> &str;

    fn raw_id(&self) -> &ImageId;

    fn library_kind(&self) -> &MediaLibraryKind;

    fn name(&self) -> &str;

    fn thumbnail_src(&self) -> String {
        crate::path::library_image_id(*self.library_kind(), MediaVariant::Thumbnail, *self.raw_id())
    }

    fn full_src(&self) -> String {
        crate::path::library_image_id(*self.library_kind(), MediaVariant::Resized, *self.raw_id())
    }
}

#[derive(Clone, Debug)]
pub struct BasicImage {
    pub id: String,
    pub raw_id: ImageId,
    pub name: String,
    pub library_kind: MediaLibraryKind,
}
impl ImageExt for BasicImage {
    fn id(&self) -> &str {
        &self.id
    }
    fn raw_id(&self) -> &ImageId {
        &self.raw_id
    }
    fn library_kind(&self) -> &MediaLibraryKind {
        &self.library_kind
    }
    fn name(&self) -> &str {
        &self.name
    }
}

impl BasicImage {
    pub fn from_string(id: String, name: String, library_kind: MediaLibraryKind) -> Self {

        let raw_id = uuid::Uuid::parse_str(&id).unwrap_throw();
        let raw_id = ImageId(raw_id);

        Self {
            id,
            raw_id,
            name,
            library_kind
        }
    }
}

