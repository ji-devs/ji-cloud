use dominator::{html, Dom};

use super::state::*;
use components::{
    backgrounds::dom::render_backgrounds, module::_common::edit::prelude::*,
    stickers::dom::render_stickers,
};
use std::rc::Rc;

impl DomRenderable for Main {
    fn render(state: Rc<Main>) -> Dom {
        html!("empty-fragment", {
            .style("grid-column", "1")
            .style("grid-row", "1")
            .style("width", "100%")
            .style("height", "100%")
            .style("overflow", "hidden")
            .child(html!("img-ui", {
                .prop("path", "jig/play/design-grid-jig.svg")
                .style("height", "100%")
                .style("width", "100%")
            }))
            .child(render_stickers(state.base.stickers.clone()))
        })
    }
}
impl MainDomRenderable for Main {
    fn render_bg(state: Rc<Main>) -> Option<Dom> {
        Some(render_backgrounds(state.base.backgrounds.clone(), None))
    }
}
