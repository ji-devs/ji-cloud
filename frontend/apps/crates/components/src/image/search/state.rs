use crate::image::tag::ImageTag;

use super::actions::get_styles;
use super::callbacks::Callbacks;
use dominator::clone;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use shared::domain::meta::ImageStyle;
use shared::domain::{jig::module::body::Image, meta::ImageStyleId};
use shared::domain::user::UserProfile;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

pub const RECENT_COUNT: u16 = 16;

pub struct State {
    pub image_list: MutableVec<Image>,
    pub recent_list: MutableVec<Image>,
    pub search: Mutable<Option<String>>,
    pub options: ImageSearchOptions,
    pub init_loader: AsyncLoader,
    pub loader: AsyncLoader,
    pub checkbox_checked: Mutable<bool>,
    pub query: Mutable<String>,
    pub page: Mutable<Option<u32>>,
    pub styles: Rc<RefCell<Option<Vec<ImageStyle>>>>,
    pub selected_styles: Rc<RefCell<HashSet<ImageStyleId>>>,
    pub callbacks: Callbacks,
    pub user: Rc<RefCell<Option<UserProfile>>>,
}

impl State {
    pub fn new(image_search_options: ImageSearchOptions, callbacks: Callbacks) -> Self {
        let styles = Rc::new(RefCell::new(None));
        let selected_styles = HashSet::new();
        let init_loader = AsyncLoader::new();
        init_loader.load(clone!(styles => async move {
            *styles.borrow_mut() = Some(get_styles().await);
        }));

        Self {
            options: image_search_options,
            search: Mutable::new(Some(String::new())),
            image_list: MutableVec::new(),
            recent_list: MutableVec::new(),
            init_loader,
            loader: AsyncLoader::new(),
            selected_styles: Rc::new(RefCell::new(selected_styles)),
            checkbox_checked: Mutable::new(true),
            query: Mutable::new(String::new()),
            page: Mutable::new(None),
            styles,
            callbacks,
            user: Rc::new(RefCell::new(None)),
        }
    }
}

pub struct ImageSearchOptions {
    pub checkbox_kind: Option<ImageSearchCheckboxKind>,
    pub upload: bool,
    pub filters: bool,
    pub recent: bool,
    pub tags: Option<Vec<ImageTag>>,
    pub tags_priority: Option<Vec<ImageTag>>,
}

impl Default for ImageSearchOptions {
    fn default() -> Self {
        Self {
            checkbox_kind: None,
            upload: true,
            filters: true,
            recent: true,
            tags: None,
            tags_priority: None,
        }
    }
}

pub enum ImageSearchCheckboxKind {
    BackgroundLayer1Filter, // adds TagId::BackgroundLayer1 to the image_tags filter
    BackgroundLayer2Filter, // adds TagId::BackgroundLayer2 to the image_tags filter
    StickersFilter, // sets `kind` to Some(ImageKind::Sticker)
}
