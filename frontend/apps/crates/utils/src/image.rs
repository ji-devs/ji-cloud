use crate::unwrap::UnwrapJiExt;
use dominator::{html, Dom};
use shared::domain::module::body::Image;

pub trait ImageExt {
    fn render(&self, slot: Option<&str>) -> Dom;
}

impl ImageExt for Image {
    fn render(&self, slot: Option<&str>) -> Dom {
        html!("img-ji", {
            .prop("id", self.id.0.to_string())
            .prop("lib", self.lib.to_str())
            .apply_if(slot.is_some(), |dom| {
                dom.prop("slot", slot.unwrap_ji())
            })
        })
    }
}
