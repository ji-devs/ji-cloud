use dominator::{html, Dom};

const STR_JEWISH_INTERACTIVE_URL: &str = "https://www.jewishinteractive.org/jigzi-home";

pub struct Iframe;

impl Iframe {
    pub fn render() -> Dom {
        html!("iframe", {
            .style("width", "100%")
            .style("height", "100%")
            .property("src", STR_JEWISH_INTERACTIVE_URL)
        })
    }
}