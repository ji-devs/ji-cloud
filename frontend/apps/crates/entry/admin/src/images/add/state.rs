use shared::domain::image::ImageSize;

use dominator_helpers::futures::AsyncLoader;
use std::cell::RefCell;
use web_sys::HtmlInputElement;

pub struct State {
    pub size: RefCell<ImageSize>,
    pub file_input: RefCell<Option<HtmlInputElement>>,
    pub loader: AsyncLoader,
}

impl State {
    pub fn new() -> Self {
        Self {
            size: RefCell::new(ImageSize::Sticker),
            file_input: RefCell::new(None),
            loader: AsyncLoader::new(),
        }
    }
}
