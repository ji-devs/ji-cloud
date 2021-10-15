use shared::domain::image::ImageKind;

use dominator_helpers::futures::AsyncLoader;
use std::cell::RefCell;
use web_sys::HtmlInputElement;

pub struct State {
    pub kind: RefCell<ImageKind>,
    pub file_input: RefCell<Option<HtmlInputElement>>,
    pub loader: AsyncLoader,
}

impl State {
    pub fn new() -> Self {
        Self {
            kind: RefCell::new(ImageKind::Sticker),
            file_input: RefCell::new(None),
            loader: AsyncLoader::new(),
        }
    }
}
