use shared::domain::image::ImageKind;
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use web_sys::HtmlInputElement;
use dominator_helpers::futures::AsyncLoader;
use components::firebase::FirebaseListener;

pub struct State {
    pub kind: RefCell<ImageKind>,
    pub file_input: RefCell<Option<HtmlInputElement>>,
    pub loader: AsyncLoader,
    pub upload_listener: RefCell<Option<FirebaseListener>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            kind: RefCell::new(ImageKind::Sticker),
            file_input: RefCell::new(None),
            loader: AsyncLoader::new(),
            upload_listener: RefCell::new(None),
        }
    }
}
