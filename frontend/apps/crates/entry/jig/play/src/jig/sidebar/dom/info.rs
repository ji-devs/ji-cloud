use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use shared::domain::{jig::JigResponse, meta::ResourceTypeId};
use utils::{
    ages::AgeRangeVecExt,
    asset::{published_at_string, ResourceContentExt},
    events,
};

use super::{super::state::State, report, track_action};

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
                track_action("Information Click", state.clone());
            }))
        }))
        .child_signal({
            (map_ref!{
                let info_popup_active = state.info_popup_active.signal_cloned(),
                let jig = state.player_state.jig.signal_cloned() => {
                    (*info_popup_active, jig.clone())
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
    state.info_popup_active.signal_cloned()
}

fn render_jig_info(state: Rc<State>, jig: &JigResponse) -> Dom {
    html!("jig-play-sidebar-jig-info", {
        .property("slot", "overlay")
        .property("name", &jig.jig_data.display_name)
        .property("playedCount", jig.plays as usize)
        .property("likedCount", jig.likes as usize)
        .property("language", &jig.jig_data.language)
        // .property("author", jig.author_id)
        .property("publishedAt", {
            match jig.published_at {
                Some(publish_at) => published_at_string(publish_at, false),
                None => String::new(),
            }
        })
        .property("description", &jig.jig_data.description)
        .child_signal(state.all_ages.signal_cloned().map(clone!(jig => move |age_ranges| {
            let range = age_ranges.range(&jig.jig_data.age_ranges);
            Some(html!("age-range", {
                .property("slot", "ages")
                .property("icon", "entry/jig/play/sidebar/age.svg")
                .property("from", range.0)
                .property("to", range.1)
            }))
        })))
        .child(html!("button-empty", {
            .property("slot", "close")
            .text("×")
            .event(clone!(state => move |_: events::Click| {
                state.info_popup_active.set(false);
            }))
        }))
        .children(jig.jig_data.categories.iter().map(|category_id| {
            html!("pill-close", {
                .property("slot", "categories")
                .property("label", &category_id.0.to_string())
            })
        }))

        .children(jig.jig_data.additional_resources.iter().map(|resource| {
            html!("a", {
                .property("slot", "additional-resources")
                .property("target", "_BLANK")
                .property("title", &resource.display_name)
                .property("href", resource.resource_content.get_link())
                .child(html!("fa-icon", {
                    .property("icon", "fa-light fa-file")
                }))
                .text(" ")
                .text(&resource.display_name)
                .text_signal(resource_type_name_signal(Rc::clone(&state), resource.resource_type_id))
            })
        }))
        .child(html!("button-rect", {
            .property("slot", "courses")
            .property("kind", "text")
            .text("Sefer Bereishit")
        }))
        .children_signal_vec(report::render(Rc::clone(&state)).to_signal_vec())
    })
}

fn resource_type_name_signal(
    state: Rc<State>,
    resource_type_id: ResourceTypeId,
) -> impl Signal<Item = String> {
    state
        .player_state
        .resource_types
        .signal_cloned()
        .map(move |resource_types| {
            let resource_type = resource_types
                .iter()
                .find(|resource_type| resource_type_id == resource_type.id);

            match resource_type {
                None => String::new(),
                Some(resource_type) => resource_type.display_name.to_owned(),
            }
        })
}
