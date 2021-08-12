use dominator::{html, Dom};

pub fn render(slot: Option<&str>) -> Dom {
    html!("page-footer", {
        .apply_if(slot.is_some(), |dom| {
            dom.property("slot", slot.unwrap())
        })
    })
}
