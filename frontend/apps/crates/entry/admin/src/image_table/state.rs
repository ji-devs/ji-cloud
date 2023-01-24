use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{asset::OrderBy, image::ImageId};
use std::{cell::RefCell, collections::HashSet, rc::Rc};

use super::editable_image::EditableImage;

pub struct ImageTable {
    pub loader: AsyncLoader,
    pub mass_editing: Mutable<bool>,
    pub selected_images: Mutable<HashSet<ImageId>>,
    pub images: MutableVec<Rc<EditableImage>>,
    pub fetch_mode: RefCell<FetchMode>,
    pub active_page: Mutable<u32>,
    pub total_pages: Mutable<Option<u32>>,
    pub order_by: Mutable<OrderBy>,
}

impl ImageTable {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            mass_editing: Mutable::new(false),
            selected_images: Default::default(),
            images: MutableVec::new(),
            fetch_mode: RefCell::new(FetchMode::Browse),
            active_page: Mutable::new(0),
            total_pages: Mutable::new(None),
            order_by: Mutable::new(OrderBy::PublishedAt),
        })
    }

    pub(super) fn clear_selected(&self) {
        self.selected_images.lock_mut().clear();
    }
}

#[derive(Clone, Debug)]
pub enum FetchMode {
    Browse,
    Search(String),
}
