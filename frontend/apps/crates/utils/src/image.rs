use crate::unwrap::UnwrapJiExt;
use dominator::{html, Dom};
use shared::domain::jig::module::body::Image;

pub trait ImageExt {
    fn render(&self, slot: Option<&str>) -> Dom;
}

impl ImageExt for Image {
    fn render(&self, slot: Option<&str>) -> Dom {
        html!("img-ji", {
            .property("id", self.id.0.to_string())
            .property("lib", self.lib.to_str())
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_ji())
            })
        })
    }
}
