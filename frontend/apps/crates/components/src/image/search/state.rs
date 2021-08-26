use super::actions::get_styles;
use super::callbacks::Callbacks;
use crate::image::tag::ImageTag;
use dominator::clone;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use shared::domain::user::UserProfile;
use shared::domain::{image::*, meta::*};
use std::collections::HashMap;
use std::{cell::RefCell, collections::HashSet, rc::Rc};
use utils::prelude::*;

pub const BACKGROUND_NAME: &'static str = "Background";

pub struct State {
    pub image_list: MutableVec<ImageMetadata>,
    pub search: Mutable<Option<String>>,
    pub options: ImageSearchOptions,
    pub init_loader: AsyncLoader,
    pub loader: AsyncLoader,

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

        if image_search_options.background_only.is_some()
            && image_search_options.background_only.unwrap_ji()
        {
            //TODO - ImageSearchOptions should just use like Vec<ImageTag>
            //and then at query time get the id via tag_id_lookup
            //this is old:
            //let style_id = get_background_id(&styles);
            //selected_styles.insert(style_id);
        }

        let init_loader = AsyncLoader::new();
        init_loader.load(clone!(styles => async move {
            *styles.borrow_mut() = Some(get_styles().await);
        }));

        Self {
            options: image_search_options,
            search: Mutable::new(Some(String::new())),
            image_list: MutableVec::new(),
            init_loader,
            loader: AsyncLoader::new(),
            selected_styles: Rc::new(RefCell::new(selected_styles)),

            query: Mutable::new(String::new()),
            page: Mutable::new(None),
            styles,
            callbacks,
            user: Rc::new(RefCell::new(None)),
        }
    }
}

// if some: control is visible and the some value is the default, if none: the control is not visible
pub struct ImageSearchOptions {
    pub background_only: Option<bool>,
    pub upload: bool,
    pub filters: bool,
}
