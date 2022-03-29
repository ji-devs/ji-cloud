use std::rc::Rc;

use futures_signals::signal::Mutable;
use utils::{storage, unwrap::UnwrapJiExt};

pub struct JigziHelp {
    pub title: String,
    pub body: String,
    pub show_id: String,
    pub show_info_tooltip: Mutable<bool>,
    pub(super) permanently_closed: Mutable<bool>,
}

impl JigziHelp {
    pub fn new(title: String, body: String, show_id: &str) -> Rc<Self> {
        let permanently_closed = storage::get_local_storage()
            .unwrap_ji()
            .get_item(&format!("tooltip-{}", show_id))
            .unwrap_ji()
            .map_or(false, |v| v == "hidden");

        Rc::new(Self {
            title,
            body,
            show_id: show_id.to_string(),
            show_info_tooltip: Mutable::new(false),
            permanently_closed: Mutable::new(permanently_closed),
        })
    }
}
