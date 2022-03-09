use dominator::{html, Dom};

pub fn render_step_3() -> Dom {
    html!("module-sidebar-body", {
        .property("slot", "body")
        .child(
            html!("sidebar-empty", {
                .property("label", crate::strings::STR_SIDEBAR_TRACE)
                .property("imagePath", "module/_common/edit/sidebar/illustration-trace-area.svg")
            })
        )
    })
}
