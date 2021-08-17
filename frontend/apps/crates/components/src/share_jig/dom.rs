use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use utils::{clipboard, events};

use super::state::{ActivePopup, State};

const STR_BACK: &'static str = "Back";

pub fn render(state: Rc<State>, anchor: Dom, slot: Option<&str>) -> Dom {
    html!("anchored-overlay", {
        .apply_if(slot.is_some(), |dom| {
            dom.property("slot", slot.unwrap())
        })
        .property("positionY", "bottom-out")
        .property("positionX", "left-in")
        .property("styled", true)
        .property("slot", "actions")
        .property_signal("open", share_open_signal(Rc::clone(&state)))
        .event(clone!(state => move |_: events::Close| {
            state.active_popup.set(None);
        }))
        .child(html!("empty-fragment", {
            .property("slot", "anchor")
            .event(clone!(state => move |_: events::Click| {
                let new_value = match &*state.active_popup.lock_ref() {
                    Some(_) => None,
                    _ => Some(ActivePopup::ShareMain),
                };
                state.active_popup.set(new_value);
            }))
            .child(anchor)
        }))
        .child_signal(state.active_popup.signal_cloned().map(clone!(state => move|active_popup| {
            match active_popup {
                Some(ActivePopup::ShareMain) => {
                    Some(render_share_main(Rc::clone(&state)))
                },
                Some(ActivePopup::ShareStudents) => {
                    Some(render_share_students(Rc::clone(&state)))
                },
                Some(ActivePopup::ShareEmbed) => {
                    Some(render_share_embed(Rc::clone(&state)))
                },
                _ => None,
            }
        })))
    })
}

fn share_open_signal(state: Rc<State>) -> impl Signal<Item = bool> {
    state
        .active_popup
        .signal_cloned()
        .map(|active_popup| match active_popup {
            Some(_) => true,
            _ => false,
        })
}

fn render_share_main(state: Rc<State>) -> Dom {
    html!("share-jig-main", {
        .property("slot", "overlay")
        .children(&mut [
            html!("button-empty", {
                .property("slot", "close")
                .text("×")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(None);
                }))
            }),
            html!("share-jig-option", {
                .property("kind", "students")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(Some(ActivePopup::ShareStudents));
                }))
            }),
            html!("share-jig-option", {
                .property("kind", "embed")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(Some(ActivePopup::ShareEmbed));
                }))
            }),
            html!("share-jig-option", {
                .property("kind", "copy")
                .event(|_: events::Click| {
                    clipboard::write_text("???");
                })
            }),
        ])
    })
}

fn render_share_students(state: Rc<State>) -> Dom {
    html!("share-jig-students", {
        .property("slot", "overlay")
        .property("url", "????")
        .property("code", "???")
        .children(&mut [
            html!("button-empty", {
                .property("slot", "close")
                .text("×")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(None);
                }))
            }),
            html!("button-rect", {
                .property("slot", "back")
                .property("kind", "text")
                .text("< ")
                .text(STR_BACK)
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(Some(ActivePopup::ShareMain));
                }))
            }),
            html!("button-rect", {
                .property("slot", "copy-url")
                .property("kind", "text")
                .text("Copy URL")
                .event(|_: events::Click| {
                    clipboard::write_text("???");
                })
            }),
            html!("button-rect", {
                .property("slot", "copy-code")
                .property("kind", "text")
                .text("Copy Code")
                .event(|_: events::Click| {
                    clipboard::write_text("???");
                })
            }),
        ])
    })
}

fn render_share_embed(state: Rc<State>) -> Dom {
    html!("share-jig-embed", {
        .property("slot", "overlay")
        .children(&mut [
            html!("button-empty", {
                .property("slot", "close")
                .text("×")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(None);
                }))
            }),
            html!("button-rect", {
                .property("slot", "back")
                .property("kind", "text")
                .text("< ")
                .text(STR_BACK)
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(Some(ActivePopup::ShareMain));
                }))
            }),
            html!("button-rect", {
                .property("slot", "copy")
                .property("kind", "text")
                .text("Copy code")
            })
        ])
    })
}
