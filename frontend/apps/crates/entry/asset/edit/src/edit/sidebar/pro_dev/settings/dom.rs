use std::rc::Rc;

use dominator::{class, clone, html, pseudo, Dom};
use futures_signals::signal::SignalExt;
use utils::events;

use super::state::ProDevSettings;

impl ProDevSettings {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("anchored-overlay", {
            .class(class! {
                .pseudo!("::part(overlay)", {
                    .style("z-index", "2")
                })
            })
            .prop("slot", "settings")
            .prop("positionX", "right-out")
            .prop("positionY", "top-in")
            .prop("styled", true)
            .prop_signal("open", state.popup_open.signal())
            .event(clone!(state => move |_: events::Close| {
                state.popup_open.set(false);
            }))
            .child(html!("fa-button", {
                .prop("slot", "anchor")
                .prop("icon", "fa-solid fa-gear")
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
                    true => Some(state.render_settings()),
                    false => None,
                }
            })))
        })
    }

    pub fn render_settings(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("pro_dev-settings", {
            .prop("slot", "overlay")
            .children(&mut [
                html!("button-icon", {
                    .prop("icon", "x")
                    .prop("slot", "close")
                    .event(clone!(state => move |_:events::Click| {
                        state.popup_open.set(false);
                    }))
                }),
                html!("label", {
                    .child(html!("input-switch", {
                        .prop_signal("enabled", state.play_in_order.signal())
                        .event(clone!(state => move|evt :events::CustomToggle| {
                            state.play_in_order.set(evt.value());
                            state.update_pro_dev_settings();
                        }))
                    }))
                    .text("play_in_order")
                }),
            ])
        })
    }
}
