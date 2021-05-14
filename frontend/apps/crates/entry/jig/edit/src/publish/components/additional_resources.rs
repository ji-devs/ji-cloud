use std::rc::Rc;

use dominator::{Dom, html};

use crate::publish::state::State;

pub fn render(_state: Rc<State>) -> Vec<Dom> {
    vec![
        html!("creator-publish-add-resource", {
            .property("slot", "additional-resources")
            .property("label", "Add Lesson Plan")
            .children(&mut [
                html!("creator-publish-add-resource-method", {
                    .property("slot", "add-method")
                    .property("kind", "upload")
                }),
                html!("creator-publish-add-resource-method", {
                    .property("slot", "add-method")
                    .property("kind", "link")
                }),
            ])
        }),
        html!("creator-publish-add-resource", {
            .property("slot", "additional-resources")
            .property("label", "Add Curriculum")
        }),
        html!("creator-publish-add-resource", {
            .property("slot", "additional-resources")
            .property("label", "Add Activities Ideas")
        }),
        html!("creator-publish-add-resource", {
            .property("slot", "additional-resources")
            .property("label", "Add Link")
        }),
    ]
}
