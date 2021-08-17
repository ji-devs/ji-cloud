use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use shared::domain::jig::Jig;
use utils::{ages::AgeRangeVecExt, events};

use super::{
    super::state::State,
    report,
};

pub fn render(state: Rc<State>) -> Dom {
    html!("anchored-overlay", {
        .property("positionY", "bottom-out")
        .property("positionX", "left-in")
        .property("styled", true)
        .property("slot", "actions")
        .property_signal("open", info_open_signal(Rc::clone(&state)))
        .event(clone!(state => move |_: events::Close| {
            state.info_popup_active.set(false);
        }))
        .child(html!("jig-play-sidebar-action", {
            .property("slot", "anchor")
            .property("kind", "info")
            .property_signal("active", info_open_signal(Rc::clone(&state)))
            .event(clone!(state => move |_: events::Click| {
                let mut info_popup_active = state.info_popup_active.lock_mut();
                *info_popup_active = !*info_popup_active;
            }))
        }))
        .child_signal({
            (map_ref!{
                let info_popup_active = state.info_popup_active.signal_cloned(),
                let jig = state.player_state.jig.signal_cloned() => {
                    (info_popup_active.clone(), jig.clone())
                }
            }).map(move|(info_popup_active, jig)| {
                match (info_popup_active, jig) {
                    (true, Some(jig)) => {
                        Some(render_jig_info(Rc::clone(&state), &jig))
                    },
                    _ => None,
                }
            })
        })
    })
}

fn info_open_signal(state: Rc<State>) -> impl Signal<Item = bool> {
    state
        .info_popup_active
        .signal_cloned()
}

fn render_jig_info(state: Rc<State>, jig: &Jig) -> Dom {
    html!("jig-play-sidebar-jig-info", {
        .property("slot", "overlay")
        .property("name", &jig.display_name)
        .property("playedCount", "?")
        .property("likedCount", "?")
        .property("language", &jig.language)
        // .property("author", jig.author_id)
        .property("description", &jig.description)
        .property_signal("ages", state.all_ages.signal_cloned().map(clone!(jig => move|all_ages| {
            all_ages.range_string(&jig.age_ranges)
        })))
        .child(html!("button-empty", {
            .property("slot", "close")
            .text("×")
            .event(clone!(state => move |_: events::Click| {
                state.info_popup_active.set(false);
            }))
        }))
        .children(jig.categories.iter().map(|category_id| {
            html!("pill-close", {
                .property("slot", "categories")
                .property("label", &category_id.0.to_string())
            })
        }).collect::<Vec<Dom>>())
        .child(html!("button-rect", {
            .property("slot", "courses")
            .property("kind", "text")
            .text("Sefer Bereishit")
        }))
        .children_signal_vec(report::render(Rc::clone(&state)).to_signal_vec())
    })
}
