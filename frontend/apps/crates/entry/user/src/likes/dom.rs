use std::rc::Rc;

use components::page_header::PageHeader;
use dominator::{html, Dom};

use super::state::Likes;

impl Likes {
    pub fn render(self: Rc<Self>) -> Dom {
        self.load_data();

        html!("div", {
            .child(PageHeader::new(Default::default()).render())
            .child(html!("h1", {
                .style("text-align", "center")
                .style("color", "var(--main-red)")
                .style("background-color", "var(--light-red-alert)")
                .style("margin", "0")
                .style("padding-block", "20px")
                .text("Likes")
            }))
            .child(self.jigs.render())
            .child(self.playlists.render())
            .child(self.resources.render())
        })
    }
}
