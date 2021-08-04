use std::rc::Rc;

use dominator::{Dom, clone, html};
use futures_signals::signal::SignalExt;
use shared::domain::jig::TextDirection;
use utils::events;

use crate::edit::sidebar::settings::{actions::{set_active_popup, update_jig_settings}, state::{ActiveSettingsPopup, FeedbackTab}};

use super::super::state::State;

const STR_DISPLAY_SCORE: &'static str = "Display score";
const STR_ASSESSMENT_MODE: &'static str = "Assessment mode";
const STR_DRAG_ASSIST: &'static str = "Drag & Drop assist";


pub fn render(state: Rc<State>) -> Dom {
    html!("jig-settings", {
        .property("slot", "overlay")
        .children(&mut [
            html!("button-icon", {
                .property("icon", "x")
                .property("slot", "close")
                .event(clone!(state => move |_:events::Click| {
                    state.active_popup.set(None);
                }))
            }),
            html!("jig-settings-button", {
                .property("slot", "creator")
                .property("kind", "theme")
                .event(clone!(state => move|_: events::Click| {
                    set_active_popup(Rc::clone(&state), ActiveSettingsPopup::Theme);
                }))
            }),
            html!("jig-settings-button", {
                .property("slot", "creator")
                .property("kind", "background")
                .event(clone!(state => move|_: events::Click| {
                    set_active_popup(Rc::clone(&state), ActiveSettingsPopup::Background);
                }))
            }),
            html!("jig-settings-button", {
                .property("slot", "creator")
                .property("kind", "feedback")
                .event(clone!(state => move|_: events::Click| {
                    set_active_popup(Rc::clone(&state), ActiveSettingsPopup::Feedback(FeedbackTab::Positive));
                }))
            }),
            html!("jig-preview-settings", {
                .property("slot", "preview")
                .children(&mut [
                    html!("jig-preview-settings-direction", {
                        .property_signal("direction", state.direction.signal().map(|dir| {
                            match dir {
                                TextDirection::LeftToRight => "ltr",
                                TextDirection::RightToLeft => "rtl",
                            }
                        }))
                        .event(clone!(state => move|evt :events::CustomDirection| {
                            state.direction.set(evt.direction());
                            update_jig_settings(Rc::clone(&state));
                        }))
                    }),
                    html!("label", {
                        .child(html!("input-switch", {
                            .property_signal("enabled", state.display_score.signal())
                            .event(clone!(state => move|evt :events::CustomToggle| {
                                state.display_score.set(evt.value());
                                update_jig_settings(Rc::clone(&state));
                            }))
                        }))
                        .text(STR_DISPLAY_SCORE)
                    }),
                    html!("label", {
                        .child(html!("input-switch", {
                            .property_signal("enabled", state.track_assessments.signal())
                            .event(clone!(state => move|evt :events::CustomToggle| {
                                state.track_assessments.set(evt.value());
                                update_jig_settings(Rc::clone(&state));
                            }))
                        }))
                        .text(STR_ASSESSMENT_MODE)
                    }),
                    html!("label", {
                        .child(html!("input-switch", {
                            .property_signal("enabled", state.drag_assist.signal())
                            .event(clone!(state => move|evt :events::CustomToggle| {
                                state.drag_assist.set(evt.value());
                                update_jig_settings(Rc::clone(&state));
                            }))
                        }))
                        .text(STR_DRAG_ASSIST)
                    }),
                ])
            }),
        ])
    })
}
