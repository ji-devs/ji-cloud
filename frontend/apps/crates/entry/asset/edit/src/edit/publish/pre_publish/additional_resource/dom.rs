use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use utils::{asset::ResourceContentExt, events};

use super::state::AdditionalResourceComponent;

impl AdditionalResourceComponent {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        let resource_content = &state.additional_resource.resource_content;
        log::info!("{:#?}", resource_content.get_link());
        html!("jig-edit-publish-resource", {
            .prop("slot", "resources")
            .prop("label", &state.additional_resource.display_name)
            .prop("resourceHref", &state.additional_resource.resource_content.get_link())
            .prop_signal("resourceType", state.resource_type_name_signal())
            .child(html!("fa-button", {
                .prop("slot", "delete")
                .prop("icon", "fa-light fa-trash-can")
                .event(clone!(state => move |_: events::Click| {
                    state.delete();
                }))
            }))
        })
    }

    fn resource_type_name_signal(self: &Rc<Self>) -> impl Signal<Item = String> {
        let state = Rc::clone(self);

        self.publish_state
            .resource_types
            .signal_cloned()
            .map(move |resource_types| {
                let resource_type = resource_types.iter().find(|resource_type| {
                    state.additional_resource.resource_type_id == resource_type.id
                });

                match resource_type {
                    None => String::new(),
                    Some(resource_type) => resource_type.display_name.to_owned(),
                }
            })
    }
}
