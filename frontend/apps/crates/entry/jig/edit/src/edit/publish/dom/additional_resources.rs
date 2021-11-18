use std::rc::Rc;

use dominator::{html, Dom};

use super::super::state::Publish;

impl Publish {
    pub fn render_additional_resources(self: Rc<Self>) -> Vec<Dom> {
        vec![
            html!("jig-edit-publish-add-resource", {
                .property("slot", "additional-resources")
                .property("label", "Add Lesson Plan")
                .children(&mut [
                    html!("jig-edit-publish-add-resource-method", {
                        .property("slot", "add-method")
                        .property("kind", "upload")
                    }),
                    html!("jig-edit-publish-add-resource-method", {
                        .property("slot", "add-method")
                        .property("kind", "link")
                    }),
                ])
            }),
            html!("jig-edit-publish-add-resource", {
                .property("slot", "additional-resources")
                .property("label", "Add Curriculum")
            }),
            html!("jig-edit-publish-add-resource", {
                .property("slot", "additional-resources")
                .property("label", "Add Activities Ideas")
            }),
            html!("jig-edit-publish-add-resource", {
                .property("slot", "additional-resources")
                .property("label", "Add Link")
            }),
        ]
    }
}
