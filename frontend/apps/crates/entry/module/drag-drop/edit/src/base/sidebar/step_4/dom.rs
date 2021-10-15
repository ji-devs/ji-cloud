use dominator::{html, Dom};

pub fn render_step_4() -> Dom {
    html!("module-sidebar-body", {
        .property("slot", "body")
        .child(
            html!("module-sidebar-drag-prompt")
        )
    })
}
