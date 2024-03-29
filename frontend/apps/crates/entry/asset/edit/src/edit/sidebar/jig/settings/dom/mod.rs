use std::rc::Rc;

use dominator::{class, clone, html, pseudo, Dom};
use futures_signals::signal::SignalExt;
use utils::events;

use crate::edit::sidebar::jig::settings::state::ActiveSettingsPopup;

use super::state::JigSettings;

pub const STR_BACK_TO_SETTINGS: &str = "Back to JIG settings";

mod background;
mod feedback;
mod main;

impl JigSettings {
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
            .prop_signal("open", state.active_popup.signal_cloned().map(|x| x.is_some()))
            .event(clone!(state => move |_: events::Close| {
                state.active_popup.set(None);
            }))
            .child(html!("fa-button", {
                .prop("slot", "anchor")
                .prop("icon", "fa-solid fa-gear")
                .style("color", "#ffffff")
                .event(clone!(state => move |_: events::Click| {
                    let mut active_popup = state.active_popup.lock_mut();

                    *active_popup = match *active_popup {
                        Some(_) => None,
                        None => Some(ActiveSettingsPopup::Main),
                    };
                }))
            }))
            .child_signal(state.active_popup.signal_cloned().map(clone!(state => move|active_popup| {
                match active_popup {
                    Some(ActiveSettingsPopup::Main) => Some(state.render_main()),
                    Some(ActiveSettingsPopup::Background) => Some(state.render_background()),
                    Some(ActiveSettingsPopup::Feedback(tab)) => Some(state.render_feedback(tab)),
                    None => None
                }
            })))
        })
    }
}
