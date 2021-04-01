use wasm_bindgen::prelude::*;
use shared::{
    domain::image::{SearchQuery, SearchResponse, ImageId, GetResponse, Image},
    media::{image_id_to_key, MediaLibraryKind, MediaVariant},
};
use utils::math::RectF64;
use serde::{Serialize, Deserialize};

pub const SEARCH_THUMBNAIL_DATA_TRANSFER:&'static str = "search-thumbnail";

pub trait ImageExt {
    //Just need to implement these
    fn id(&self) -> &ImageId;
    fn id_str(&self) -> &str;
    fn library_kind(&self) -> &MediaLibraryKind;

    //These come for free
    fn id_string(&self) -> String {
        self.id_str().to_string()
    }
    fn thumbnail_src(&self) -> String {
        utils::path::image_lib_url(*self.library_kind(), MediaVariant::Thumbnail, *self.id())
    }

    fn full_src(&self) -> String {
        utils::path::image_lib_url(*self.library_kind(), MediaVariant::Resized, *self.id())
    }
}

#[derive(Clone, Debug)]
pub struct MetaImage {
    _id_string: String,
    _library_kind: MediaLibraryKind,
    pub meta: Image
}

impl MetaImage {
    pub fn new_debug(id_str:&str, name:&str, library_kind: MediaLibraryKind) -> Self {
        let id = uuid::Uuid::parse_str(&id_str).unwrap_throw();
        let id = ImageId(id);
        let _id_string = id_str.to_string();
        let _library_kind = library_kind;

        let meta = Image {
            id,
            name: name.to_string(),
            description: "".to_string(),
            is_premium: false,
            publish_at: None,
            styles: Vec::new(),
            age_ranges: Vec::new(),
            affiliations: Vec::new(),
            categories: Vec::new(),
            created_at: chrono::Utc::now(),
            updated_at: None,
        };

        Self {
            _id_string,
            _library_kind,
            meta
        }
    }

    pub fn is_published(&self) -> bool {
        match self.meta.publish_at {
            None => false,
            Some(_) => true // TODO - compare to Utc::now
        }
    }
}
impl ImageExt for MetaImage {
    fn id(&self) -> &ImageId {
        &self.meta.id
    }
    fn id_str(&self) -> &str {
        &self._id_string
    }
    fn library_kind(&self) -> &MediaLibraryKind {
        &self._library_kind
    }
}

impl From<(Image, MediaLibraryKind)> for MetaImage {
    fn from((meta, _library_kind):(Image, MediaLibraryKind)) -> Self {
        let _id_string = meta.id.0.to_string();

        Self {
            _id_string,
            _library_kind,
            meta
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SimpleImage {
    _id: ImageId,
    _id_string: String,
    _library_kind: MediaLibraryKind,
}

impl ImageExt for SimpleImage {
    fn id(&self) -> &ImageId {
        &self._id
    }
    fn id_str(&self) -> &str {
        &self._id_string
    }
    fn library_kind(&self) -> &MediaLibraryKind {
        &self._library_kind
    }
}


impl From<(&str, MediaLibraryKind)> for SimpleImage {
    fn from((_id_str, _library_kind):(&str, MediaLibraryKind)) -> Self {
        let _id_string = _id_str.to_string();
        let id = uuid::Uuid::parse_str(&_id_str).unwrap_throw();
        let _id = ImageId(id);
        Self {
            _id,
            _id_string,
            _library_kind,
        }
    }
}
impl From<(String, MediaLibraryKind)> for SimpleImage {
    fn from((_id_string, _library_kind):(String, MediaLibraryKind)) -> Self {

        let id = uuid::Uuid::parse_str(&_id_string).unwrap_throw();
        let _id = ImageId(id);
        Self {
            _id,
            _id_string,
            _library_kind,
        }
    }
}

impl From<(ImageId, MediaLibraryKind)> for SimpleImage {
    fn from((_id, _library_kind):(ImageId, MediaLibraryKind)) -> Self {

        let _id_string = _id.0.to_string();
        Self {
            _id,
            _id_string,
            _library_kind,
        }
    }
}


impl From<(Image, MediaLibraryKind)> for SimpleImage {
    fn from((meta, _library_kind):(Image, MediaLibraryKind)) -> Self {
        
        let _id = meta.id;
        let _id_string = _id.0.to_string();

        Self {
            _id,
            _id_string,
            _library_kind,
        }
    }
}

impl From<MetaImage> for SimpleImage {
    fn from(img:MetaImage) -> Self {
        
        let _id = img.meta.id;
        let _id_string = _id.0.to_string();
        let _library_kind = img.library_kind().clone();

        Self {
            _id,
            _id_string,
            _library_kind,
        }
    }
}
