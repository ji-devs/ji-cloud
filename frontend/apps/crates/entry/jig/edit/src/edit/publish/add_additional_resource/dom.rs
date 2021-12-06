use std::{rc::Rc, str::FromStr};

use components::{
    input::simple_select::SimpleSelect,
    overlay::handle::OverlayHandle,
    tooltip::{
        callbacks::TooltipErrorCallbacks,
        dom::render as TooltipDom,
        state::{Anchor, ContentAnchor, MoveStrategy, TooltipData, TooltipError, TooltipTarget, State as TooltipState},
    }
};
use dominator::{Dom, clone, html, with_node};
use futures_signals::signal::SignalExt;
use url::Url;
use utils::events;
use web_sys::HtmlElement;

use crate::edit::{iframe::actions, publish::add_additional_resource::{add_file::state::AddFile, add_link::state::AddLink}};

use super::state::{AddAdditionalResource, ActivePopup};

const STR_UPLOAD_FILE: &str = "Upload file";
const STR_ADD_LINK: &str = "Add link";
const STR_SELECT_REQUIRED: &str = "Please select type\nbefore moving on";

impl AddAdditionalResource {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = Rc::clone(&self);
        html!("empty-fragment", {
            .style("display", "contents")
            .property("slot", "additional-resources")
            .child_signal(state.loader.is_loading().map(clone!(state => move|is_loading| {
                match is_loading {
                    true => Some(
                        html!("progress-bar", {
                            .property("progress", "infinite")
                        })
                    ),
                    false => Some(
                        html!("jig-edit-publish-resource-button-add", {
                            .event(clone!(state => move |event: events::Click| {
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
                match active_popup {
                    None => None,
                    Some(popup) => {
                        Some(state.render_popup(popup))
                    }
                }
            })))
        })
    }

    fn render_popup(self: &Rc<Self>, popup: ActivePopup) -> Dom {
        let state = Rc::clone(&self);
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
        let state = Rc::clone(&self);
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
