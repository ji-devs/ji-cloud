use std::rc::Rc;

use components::overlay::handle::OverlayHandle;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;

use utils::events;
use web_sys::HtmlElement;

use crate::edit::publish::add_additional_resource::{
    add_file::state::AddFile, add_link::state::AddLink,
};

use super::state::{ActivePopup, AddAdditionalResource};

const STR_UPLOAD_FILE: &str = "Upload file";
const STR_ADD_LINK: &str = "Add link";

impl AddAdditionalResource {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("empty-fragment", {
            .style("display", "contents")
            .property("slot", "resources")
            .child_signal(state.loader.is_loading().map(clone!(state => move|is_loading| {
                match is_loading {
                    true => Some(
                        html!("progress-bar", {
                            .property("progress", "infinite")
                        })
                    ),
                    false => Some(
                        html!("jig-edit-publish-resource-button-add", {
                            .event(clone!(state => move |_event: events::Click| {
                                let mut active_popup = state.active_popup.lock_mut();
                                match &*active_popup {
                                    Some(_) => *active_popup = None,
                                    None => *active_popup = Some(ActivePopup::Main),
                                }
                            }))
                        })
                    )
                }
            })))
            .child_signal(state.active_popup.signal().map(clone!(state => move|active_popup| {
                active_popup.map(|popup| {
                    state.render_popup(popup)
                })
            })))
        })
    }

    fn render_popup(self: &Rc<Self>, popup: ActivePopup) -> Dom {
        let state = Rc::clone(self);
        html!("div" => HtmlElement, {
            .with_node!(elem => {
                .apply(OverlayHandle::lifecycle(
                    move || {
                        html!("overlay-content", {
                            .property("target", &elem)
                            .property("contentAnchor", "oppositeV")
                            .property("targetAnchor", "bm")
                            .child({
                                match popup {
                                    ActivePopup::Main => {
                                        state.render_main_popup()
                                    },
                                    ActivePopup::File => {
                                        AddFile::new(Rc::clone(&state)).render()
                                    },
                                    ActivePopup::Link => {
                                        AddLink::new(Rc::clone(&state)).render()
                                    },
                                }
                            })
                        })
                    }
                ))
            })
        })
    }

    fn render_main_popup(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("jig-edit-publish-resource-add", {
            .children(&mut [
                html!("fa-button", {
                    .property("icon", "fa-light fa-xmark")
                    .property("slot", "close")
                    .event(clone!(state => move|_: events::Click| {
                        state.active_popup.set(None);
                    }))
                }),
                html!("button", {
                    .property("slot", "options")
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(Some(ActivePopup::File));
                    }))
                    .children(&mut [
                        html!("fa-icon", {
                            .property("icon", "fa-light fa-arrow-up-from-bracket")
                        }),
                        html!("span", {
                            .text(STR_UPLOAD_FILE)
                        }),
                    ])
                }),
                html!("button", {
                    .property("slot", "options")
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(Some(ActivePopup::Link));
                    }))
                    .children(&mut [
                        html!("fa-icon", {
                            .property("icon", "fa-light fa-link-simple")
                        }),
                        html!("span", {
                            .text(STR_ADD_LINK)
                        }),
                    ])
                }),
            ])
        })
    }
}
