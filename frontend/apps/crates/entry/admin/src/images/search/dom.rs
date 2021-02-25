use dominator::{html, clone, Dom};
use shared::domain::image::{ImageId, ImageSearchQuery};

pub struct ImageSearchPage {
}

impl ImageSearchPage {
    pub fn render(query: Option<ImageSearchQuery>) -> Dom {
        html!("div", {.text("TODO") })
    }
}
