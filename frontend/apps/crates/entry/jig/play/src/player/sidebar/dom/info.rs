use std::rc::Rc;

use dominator::{Dom, clone, html};
use futures_signals::{map_ref, signal::{Signal, SignalExt}};
use shared::domain::jig::Jig;
use utils::events;

use super::{report, super::state::{ActivePopup, State}};

pub fn render(state: Rc<State>) -> Dom {
    html!("anchored-overlay", {
        .property("positionY", "bottom-out")
        .property("positionX", "left-in")
        .property("slot", "actions")
        .property_signal("open", info_open_signal(Rc::clone(&state)))
        .event(clone!(state => move |_: events::Close| {
            state.active_popup.set(ActivePopup::None);
        }))
        .child(html!("jig-play-sidebar-action", {
            .property("slot", "anchor")
            .property("kind", "info")
            .property_signal("active", info_open_signal(Rc::clone(&state)))
            .event(clone!(state => move |_: events::Click| {
                let new_value = match &*state.active_popup.lock_ref() {
                    ActivePopup::JigInfo => ActivePopup::None,
                    _ => ActivePopup::JigInfo
                };
                state.active_popup.set(new_value);
            }))
        }))
        .child_signal({
            (map_ref!{
                let active_popup = state.active_popup.signal_cloned(),
                let jig = state.player_state.jig.signal_cloned() => {
                    (active_popup.clone(), jig.clone())
                }
            }).map(move|(active_popup, jig)| {
                match (active_popup, jig) {
                    (ActivePopup::JigInfo, Some(jig)) => {
                        Some(render_jig_info(Rc::clone(&state), &jig))
                    },
                    _ => None,
                }
            })
        })
    })
}

fn info_open_signal(state: Rc<State>) -> impl Signal<Item = bool> {
    state.active_popup.signal_cloned().map(|active_popup| {
        match active_popup {
            ActivePopup::JigInfo => true,
            _ => false,
        }
    })
}

fn render_jig_info(state: Rc<State>, jig: &Jig) -> Dom {
    html!("jig-play-sidebar-jig-info", {
        .property("slot", "overlay")
        .property("name", &jig.display_name)
        .property("playedCount", "?")
        .property("likedCount", "?")
        .property("ages", "?")
        .property("language", &jig.language)
        // .property("author", jig.author_id)
        .property("description", &jig.description)
        .children(jig.categories.iter().map(|category_id| {
            html!("pill-close", {
                .property("slot", "categories")
                .property("label", &category_id.0.to_string())
            })
        }).collect::<Vec<Dom>>())
        .child(html!("button-text", {
            .property("slot", "courses")
            .text("Sefer Bereishit")
        }))
        .children_signal_vec(report::render(Rc::clone(&state)).to_signal_vec())
    })
}
