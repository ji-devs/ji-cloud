use std::{cell::RefCell, collections::{HashSet}, rc::Rc};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use dominator_helpers::futures::AsyncLoader;
use shared::domain::{image::*, meta::*};
use wasm_bindgen::UnwrapThrowExt;
use super::actions::{get_background_id, get_styles};
use utils::prelude::*;

pub const BACKGROUND_NAME: &'static str = "Background";


pub struct State {
    pub image_list: MutableVec<Image>,
    pub search: Mutable<Option<String>>,
    pub options: ImageSearchOptions,
    pub loader: AsyncLoader,

    pub query: Mutable<String>,
    pub page: Mutable<Option<u32>>,
    pub styles: Vec<Style>,
    pub selected_styles: Rc<RefCell<HashSet<StyleId>>>,
}

impl State {
    pub async fn new(image_search_options: ImageSearchOptions) -> Self {
        let styles = get_styles().await;
        let mut selected_styles = HashSet::new();

        if image_search_options.background_only.is_some() && image_search_options.background_only.unwrap_ji() {
            let style_id = get_background_id(&styles);
            selected_styles.insert(style_id);
        }

        Self {
            options: image_search_options,
            search: Mutable::new(Some(String::new())),
            image_list: MutableVec::new(),
            loader: AsyncLoader::new(),
            selected_styles: Rc::new(RefCell::new(selected_styles)),

            query: Mutable::new(String::new()),
            page: Mutable::new(None),
            styles,
        }
    }
}


// if some: control is visible and the some value is the default, if none: the control is not visible
pub struct ImageSearchOptions {
    pub background_only: Option<bool>,
    pub upload: Option<()>, // NOTE: make this a bool? - David
    pub filters: Option<()>, // NOTE: make this a bool? - David
    pub value: Mutable<Option<ImageId>>,
}
