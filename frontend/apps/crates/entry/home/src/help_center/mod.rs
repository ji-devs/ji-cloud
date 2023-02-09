use dominator::{html, Dom};

use components::{
    page_footer,
    page_header::{PageHeader, PageHeaderConfig},
};

mod dom;
use dom::Iframe;

pub fn render_help_center() -> Dom {
    html!("div", {
        .child(
            PageHeader::new(PageHeaderConfig {
                render_beta: true,
                ..Default::default()
            }).render()
        )
        .child(
            Iframe::new().render_help()
        )
        .child(page_footer::dom::render(None))
    })
}
