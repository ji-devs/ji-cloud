use dominator::{html, Dom};
use std::rc::Rc;
use utils::routes::HomePlanRoute;

use super::Plan;

impl Plan {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("iframe", {
            .style("width", "100vw")
            .style("height", "100vh")
            .style("border", "0")
            .prop("src", state.get_page_url())
        })
    }

    fn get_page_url(self: &Rc<Self>) -> &'static str {
        match self.route {
            HomePlanRoute::Basic => "https://corinne4371.editorx.io/basic/home-2",
            HomePlanRoute::Pro => "https://corinne4371.editorx.io/basic/home-1",
            HomePlanRoute::School => "https://corinne4371.editorx.io/basic/home-3",
        }
    }
}
