use std::rc::Rc;

use futures_signals::signal::Mutable;

pub struct JigziHelp {
    pub title: String,
    pub body: String,
    pub show_id: String,
    pub show_info_tooltip: Mutable<bool>,
}

impl JigziHelp {
    pub fn new(
        title: String,
        body: String,
        show_id: &str,
    ) -> Rc<Self> {
        Rc::new(Self {
            title,
            body,
            show_id: show_id.to_string(),
            show_info_tooltip: Mutable::new(false),
        })
    }
}
