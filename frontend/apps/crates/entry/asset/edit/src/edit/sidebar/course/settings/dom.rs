use std::rc::Rc;

use dominator::{class, clone, html, pseudo, Dom};
use futures_signals::signal::SignalExt;
use utils::events;

use super::{actions::update_course_settings, state::State};

pub fn render(state: Rc<State>) -> Dom {
    html!("anchored-overlay", {
        .class(class! {
            .pseudo!("::part(overlay)", {
                .style("z-index", "2")
            })
        })
        .property("slot", "settings")
        .property("positionX", "right-out")
        .property("positionY", "top-in")
        .property("styled", true)
        .property_signal("open", state.popup_open.signal())
        .event(clone!(state => move |_: events::Close| {
            state.popup_open.set(false);
        }))
        .child(html!("fa-button", {
            .property("slot", "anchor")
            .property("icon", "fa-solid fa-gear")
            .style("color", "#ffffff")
            .event(clone!(state => move |_: events::Click| {
                // let popup_open = state.popup_open.get();
                // state.popup_open.set(!popup_open);
                state.popup_open.replace_with(|popup_open| {
                    !*popup_open
                });
            }))
        }))
        .child_signal(state.popup_open.signal().map(clone!(state => move|popup_open| {
            match popup_open {
                true => Some(render_settings(Rc::clone(&state))),
                false => None,
            }
        })))
    })
}

pub fn render_settings(state: Rc<State>) -> Dom {
    html!("course-settings", {
        .property("slot", "overlay")
        .children(&mut [
            html!("button-icon", {
                .property("icon", "x")
                .property("slot", "close")
                .event(clone!(state => move |_:events::Click| {
                    state.popup_open.set(false);
                }))
            }),
            html!("label", {
                .child(html!("input-switch", {
                    .property_signal("enabled", state.play_in_order.signal())
                    .event(clone!(state => move|evt :events::CustomToggle| {
                        state.play_in_order.set(evt.value());
                        update_course_settings(Rc::clone(&state));
                    }))
                }))
                .text("play_in_order")
            }),
        ])
    })
}
