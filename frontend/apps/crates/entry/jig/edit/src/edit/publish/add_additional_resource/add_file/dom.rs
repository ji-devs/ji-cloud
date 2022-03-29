use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{not, Signal},
};

use utils::events;

use crate::edit::publish::add_additional_resource::ActivePopup;

use super::state::AddFile;

const STR_SAVE: &str = "Save";
const STR_CANCEL: &str = "Cancel";
const STR_BACK: &str = "Back";

impl AddFile {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = Rc::clone(&self);
        html!("jig-edit-publish-resource-add-file", {
            .children(&mut [
                html!("button-rect", {
                    .property("slot", "back")
                    .property("color", "blue")
                    .property("kind", "text")
                    .child(html!("fa-icon", {
                        .property("icon", "fa-solid fa-angle-left")
                    }))
                    .text(STR_BACK)
                    .event(clone!(state => move|_: events::Click| {
                        state.add_resources_state.active_popup.set(Some(ActivePopup::Main));
                    }))
                }),
                html!("fa-button", {
                    .property("icon", "fa-light fa-xmark")
                    .property("slot", "close")
                    .event(clone!(state => move|_: events::Click| {
                        state.add_resources_state.active_popup.set(None);
                    }))
                }),
                html!("input-file", {
                    .property("slot", "input-file")
                    .property("accept", "image/*,audio/*,application/pdf")
                    .text_signal(state.file.signal_ref(|file| {
                        match file {
                            Some(file) => file.name(),
                            None => String::from("Select file")
                        }
                    }))
                    .event(clone!(state => move |e: events::CustomFile| {
                        let file = e.file();
                        state.file.set(Some(file));
                    }))
                }),
                html!("button-rect", {
                    .property("slot", "actions")
                    .property("color", "blue")
                    .property("kind", "text")
                    .text(STR_CANCEL)
                    .event(clone!(state => move|_: events::Click| {
                        state.add_resources_state.active_popup.set(None);
                    }))
                }),
                html!("button-rect", {
                    .property("slot", "actions")
                    .property("color", "blue")
                    .property("kind", "filled")
                    .text(STR_SAVE)
                    .property_signal("disabled", not(state.form_filled_out()))
                    .event(clone!(state => move|_: events::Click| {
                        state.save();
                    }))
                }),
            ])
            .child_signal(state.add_resources_state.publish_state.resource_types.signal_ref(clone!(state => move|resource_types| {
                Some(html!("input-select", {
                    .property("slot", "type")
                    .property("label", "Select type")
                    .property("placeholder", "Select one")
                    .property_signal("value", state.resource_type.signal_ref(|resource_type| {
                        match resource_type {
                            Some(resource_type) => resource_type.display_name.clone(),
                            None => String::new(),
                        }
                    }))
                    .children(resource_types.iter().map(clone!(state => move |resource_type| {
                        html!("input-select-option", {
                            .text(&resource_type.display_name)
                            .event(clone!(state, resource_type => move |evt:events::CustomSelectedChange| {
                                if evt.selected() {
                                    state.resource_type.set(Some(resource_type.clone()));
                                }
                            }))
                        })
                    })))
                }))
            })))
        })
    }

    fn form_filled_out(self: &Rc<Self>) -> impl Signal<Item = bool> {
        map_ref! {
            let file = self.file.signal_cloned(),
            let resource_type = self.resource_type.signal_cloned()
                => move {
                    file.is_some() && resource_type.is_some()
                }
        }
    }
}
