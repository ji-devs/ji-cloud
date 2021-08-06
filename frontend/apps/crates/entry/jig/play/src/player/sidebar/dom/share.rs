use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use utils::{clipboard, events};

use super::super::state::{ActivePopup, State};

const STR_BACK: &'static str = "Back";

pub fn render(state: Rc<State>) -> Dom {
    html!("anchored-overlay", {
        .property("positionY", "bottom-out")
        .property("positionX", "left-in")
        .property("slot", "actions")
        .property_signal("open", share_open_signal(Rc::clone(&state)))
        .event(clone!(state => move |_: events::Close| {
            state.active_popup.set(ActivePopup::None);
        }))
        .child(html!("jig-play-sidebar-action", {
            .property("slot", "anchor")
            .property("kind", "share")
            .property_signal("active", share_open_signal(Rc::clone(&state)))
            .event(clone!(state => move |_: events::Click| {
                let new_value = match &*state.active_popup.lock_ref() {
                    ActivePopup::ShareMain | ActivePopup::ShareStudents | ActivePopup::ShareEmbed => ActivePopup::None,
                    _ => ActivePopup::ShareMain
                };
                state.active_popup.set(new_value);
            }))
        }))
        .child_signal(state.active_popup.signal_cloned().map(clone!(state => move|active_popup| {
            match active_popup {
                ActivePopup::ShareMain => {
                    Some(render_share_main(Rc::clone(&state)))
                },
                ActivePopup::ShareStudents => {
                    Some(render_share_students(Rc::clone(&state)))
                },
                ActivePopup::ShareEmbed => {
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
            ActivePopup::ShareMain | ActivePopup::ShareStudents | ActivePopup::ShareEmbed => true,
            _ => false,
        })
}

fn render_share_main(state: Rc<State>) -> Dom {
    html!("jig-play-sidebar-share-main", {
        .property("slot", "overlay")
        .children(&mut [
            html!("button-empty", {
                .property("slot", "close")
                .text("×")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(ActivePopup::None);
                }))
            }),
            html!("jig-play-sidebar-share-option", {
                .property("kind", "students")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(ActivePopup::ShareStudents);
                }))
            }),
            html!("jig-play-sidebar-share-option", {
                .property("kind", "embed")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(ActivePopup::ShareEmbed);
                }))
            }),
            html!("jig-play-sidebar-share-option", {
                .property("kind", "copy")
                .event(|_: events::Click| {
                    clipboard::write_text("???");
                })
            }),
        ])
    })
}

fn render_share_students(state: Rc<State>) -> Dom {
    html!("jig-play-sidebar-share-students", {
        .property("slot", "overlay")
        .property("url", "????")
        .property("code", "???")
        .children(&mut [
            html!("button-empty", {
                .property("slot", "close")
                .text("×")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(ActivePopup::None);
                }))
            }),
            html!("button-rect", {
                .property("slot", "back")
                .property("kind", "text")
                .text("< ")
                .text(STR_BACK)
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(ActivePopup::ShareMain);
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
    html!("jig-play-sidebar-share-embed", {
        .property("slot", "overlay")
        .children(&mut [
            html!("button-empty", {
                .property("slot", "close")
                .text("×")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(ActivePopup::None);
                }))
            }),
            html!("button-rect", {
                .property("slot", "back")
                .property("kind", "text")
                .text("< ")
                .text(STR_BACK)
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(ActivePopup::ShareMain);
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
