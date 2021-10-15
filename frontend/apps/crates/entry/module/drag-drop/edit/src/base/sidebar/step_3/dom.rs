use dominator::{html, Dom};

pub fn render_step_3() -> Dom {
    html!("module-sidebar-body", {
        .property("slot", "body")
        .child(
            html!("div", {
                .text(crate::strings::STR_SIDEBAR_TRACE)
            })
        )
    })
}
