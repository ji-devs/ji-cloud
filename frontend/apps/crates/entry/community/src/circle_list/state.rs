use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::circle::Circle;

pub struct CirclesList {
    pub items_per_page: u32,
    pub circles: Mutable<Option<Vec<Circle>>>,
    pub loader: AsyncLoader,
    pub create_popup_open: Mutable<bool>,
    pub total_pages: Mutable<u32>,
    pub active_page: Mutable<u32>,
}

impl CirclesList {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            items_per_page: 20,
            circles: Mutable::new(None),
            loader: AsyncLoader::new(),
            create_popup_open: Mutable::new(false),
            total_pages: Mutable::new(0),
            active_page: Mutable::new(1),
        })
    }
}
