use dominator::{html, Dom};

use std::rc::Rc;

use components::{
    page_footer,
    page_header::{self},
};

mod dom;
use dom::Iframe;

pub fn render_help_center() -> Dom {
    html!("div", {
        .child(
            page_header::dom::render(Rc::new(page_header::state::State::new()), None, None, true)
        )
        .child(
            Iframe::new().render_help()
        )
        .child(page_footer::dom::render(None))
    })
}
