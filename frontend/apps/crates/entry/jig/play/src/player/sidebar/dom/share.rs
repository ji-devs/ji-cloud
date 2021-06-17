use std::rc::Rc;

use dominator::{Dom, clone, html};
use futures_signals::signal::{Signal, SignalExt};
use utils::events;

use super::super::state::{ActivePopup, State};

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
    state.active_popup.signal_cloned().map(|active_popup| {
        match active_popup {
            ActivePopup::ShareMain | ActivePopup::ShareStudents | ActivePopup::ShareEmbed => true,
            _ => false,
        }
    })
}

fn render_share_main(state: Rc<State>) -> Dom {
    html!("jig-play-sidebar-share-main", {
        .property("slot", "overlay")
        .children(&mut [
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
                .event(clone!(state => move |_: events::Click| {
                    todo!();
                }))
            }),
        ])
    })
}

fn render_share_students(_state: Rc<State>) -> Dom {
    html!("jig-play-sidebar-share-students", {
        .property("slot", "overlay")
        .property("url", "ji.zone/play/3692")
        .property("code", "3692")

        .children(&mut [
            html!("button-text", {
                .property("slot", "copy-url")
                .text("Copy URL")
            }),
            html!("button-text", {
                .property("slot", "copy-code")
                .text("Copy Code")
            }),
        ])
    })
}

fn render_share_embed(_state: Rc<State>) -> Dom {
    html!("jig-play-sidebar-share-embed", {
        .property("slot", "overlay")
        .child(html!("button-text", {
            .property("slot", "copy")
            .text("Copy code")
        }))
    })
}
