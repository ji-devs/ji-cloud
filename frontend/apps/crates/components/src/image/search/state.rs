use crate::image::tag::ImageTag;

use super::actions::get_styles;
use super::callbacks::ImageSearchCallbacks;
use dominator::clone;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use shared::domain::image::ImageId;
use shared::domain::meta::ImageStyle;
use shared::domain::search::{ImageType, WebImageSearchItem};
use shared::domain::user::UserProfile;
use shared::domain::{meta::ImageStyleId, module::body::Image};
use shared::media::MediaLibrary;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

pub const RECENT_COUNT: u16 = 12;
const STR_SELECT_IMAGE: &str = "Select image";
const STR_SELECT_BACKGROUND: &str = "Select a background";
const STR_SELECT_OVERLAY: &str = "Search a shape or number";

pub struct ImageSearch {
    pub search_mode: Mutable<SearchMode>,
    pub recent_list: MutableVec<Image>,
    pub search: Mutable<Option<String>>,
    pub options: ImageSearchOptions,
    pub init_loader: AsyncLoader,
    pub loader: AsyncLoader,
    pub checkbox_checked: Mutable<bool>,
    pub query: Mutable<String>,
    pub styles: Rc<RefCell<Option<Vec<ImageStyle>>>>,
    pub selected_styles: Rc<RefCell<HashSet<ImageStyleId>>>,
    pub selected_image_type: Mutable<Option<ImageType>>,
    pub callbacks: ImageSearchCallbacks,
    pub user: Rc<RefCell<Option<UserProfile>>>,
    pub next_page: RefCell<NextPage>,
    pub recent: bool,
}

impl ImageSearch {
    pub fn new(
        image_search_options: ImageSearchOptions,
        callbacks: ImageSearchCallbacks,
    ) -> Rc<Self> {
        let styles = Rc::new(RefCell::new(None));
        let selected_styles = HashSet::new();
        let init_loader = AsyncLoader::new();
        init_loader.load(clone!(styles => async move {
            *styles.borrow_mut() = Some(get_styles().await);
        }));

        // don't display recent in overlay
        let recent = image_search_options.kind != ImageSearchKind::Overlay;

        let checkbox_checked = Mutable::new(match image_search_options.kind {
            ImageSearchKind::Background => true,
            ImageSearchKind::Sticker => false,
            ImageSearchKind::Overlay => true, // overlays don't show the checkbox
        });

        Rc::new(Self {
            options: image_search_options,
            search: Mutable::new(Some(String::new())),
            recent_list: MutableVec::new(),
            init_loader,
            loader: AsyncLoader::new(),
            selected_styles: Rc::new(RefCell::new(selected_styles)),
            selected_image_type: Mutable::new(None),
            checkbox_checked,
            query: Mutable::new(String::new()),
            styles,
            callbacks,
            user: Rc::new(RefCell::new(None)),
            search_mode: Mutable::new(SearchMode::Sticker(Rc::new(MutableVec::new()))),
            next_page: RefCell::new(NextPage::default()),
            recent,
        })
    }
}

pub struct ImageSearchOptions {
    pub kind: ImageSearchKind,
    pub upload: bool,
    pub filters: bool,
    pub tags: Option<Vec<ImageTag>>,
    pub tags_priority: Option<Vec<ImageTag>>,
}

impl Default for ImageSearchOptions {
    fn default() -> Self {
        Self {
            kind: ImageSearchKind::Sticker,
            upload: true,
            filters: true,
            tags: None,
            tags_priority: None,
        }
    }
}

// sticker is handled with ImageSize instead of ImageTag since we want all images to default to sticker
// but don't wanna add all manually so we accepted this compromise
#[derive(PartialEq)]
pub enum ImageSearchKind {
    Background, // adds ImageTag::BackgroundLayer1 to the image_tags filter
    Overlay,    // adds ImageTag::BackgroundLayer2 to the image_tags filter
    Sticker,    // sets `size` to Some(ImageSize::Sticker)
}

impl ImageSearchKind {
    pub fn label(&self) -> &str {
        match self {
            Self::Background => STR_SELECT_BACKGROUND,
            Self::Overlay => STR_SELECT_OVERLAY,
            Self::Sticker => STR_SELECT_IMAGE,
        }
    }
}

/// Image that can be premium
#[derive(Clone, Debug)]
pub struct PremiumableImage {
    pub id: ImageId,
    pub lib: MediaLibrary,
    pub is_premium: bool,
}
impl From<PremiumableImage> for Image {
    fn from(image: PremiumableImage) -> Image {
        Image {
            id: image.id,
            lib: image.lib,
        }
    }
}
impl PremiumableImage {
    pub fn from_image_free(image: Image) -> PremiumableImage {
        PremiumableImage {
            id: image.id,
            lib: image.lib,
            is_premium: false,
        }
    }
}

#[derive(Clone)]
pub enum SearchMode {
    Sticker(Rc<MutableVec<PremiumableImage>>),
    Web(Rc<MutableVec<WebImageSearchItem>>),
}

impl SearchMode {
    pub fn is_sticker(&self) -> bool {
        match self {
            Self::Sticker(_) => true,
            Self::Web(_) => false,
        }
    }

    pub fn is_web(&self) -> bool {
        match self {
            Self::Sticker(_) => false,
            Self::Web(_) => true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum NextPage {
    Page(u32),
    End,
}

impl Default for NextPage {
    fn default() -> Self {
        Self::Page(0)
    }
}
