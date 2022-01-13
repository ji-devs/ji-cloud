use dominator::{html, Dom};
use utils::unwrap::UnwrapJiExt;

pub fn render(slot: Option<&str>) -> Dom {
    html!("page-footer", {
        .apply_if(slot.is_some(), |dom| {
            dom.property("slot", slot.unwrap_ji())
        })
    })
}
