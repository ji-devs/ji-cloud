use shared::domain::image::ImageKind;
use std::cell::RefCell;
use web_sys::HtmlInputElement;

pub struct State {
    pub kind: RefCell<ImageKind>,
    pub file_input: RefCell<Option<HtmlInputElement>>
}

impl State {
    pub fn new() -> Self {
        Self {
            kind: RefCell::new(ImageKind::Sticker),
            file_input: RefCell::new(None)
        }
    }
}
