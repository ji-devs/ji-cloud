use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::course::TextDirection;
use utils::events;

use crate::edit::sidebar::course::settings::{
    actions::{set_active_popup, update_course_settings},
    state::{ActiveSettingsPopup, FeedbackTab},
};

use super::super::state::State;

// const STR_DISPLAY_SCORE: &str = "Display score";
const STR_ASSESSMENT_MODE: &str = "Assessment mode";
const STR_DRAG_ASSIST: &str = "Drag & Drop assist";

pub fn render(state: Rc<State>) -> Dom {
    html!("course-settings", {
        .property("slot", "overlay")
        .children(&mut [
            html!("button-icon", {
                .property("icon", "x")
                .property("slot", "close")
                .event(clone!(state => move |_:events::Click| {
                    state.active_popup.set(None);
                }))
            }),
            html!("course-settings-button", {
                .property("slot", "creator")
                .property("kind", "background")
                .event(clone!(state => move|_: events::Click| {
                    set_active_popup(Rc::clone(&state), ActiveSettingsPopup::Background);
                }))
            }),
            html!("course-settings-button", {
                .property("slot", "creator")
                .property("kind", "feedback")
                .event(clone!(state => move|_: events::Click| {
                    set_active_popup(Rc::clone(&state), ActiveSettingsPopup::Feedback(FeedbackTab::Positive));
                }))
            }),
            html!("course-preview-settings", {
                .property("slot", "preview")
                .children(&mut [
                    html!("course-preview-settings-direction", {
                        .property_signal("direction", state.direction.signal().map(|dir| {
                            match dir {
                                TextDirection::LeftToRight => "ltr",
                                TextDirection::RightToLeft => "rtl",
                            }
                        }))
                        .event(clone!(state => move|evt :events::CustomDirection| {
                            state.direction.set(evt.direction());
                            update_course_settings(Rc::clone(&state));
                        }))
                    }),
                    // html!("label", {
                    //     .child(html!("input-switch", {
                    //         .property_signal("enabled", state.display_score.signal())
                    //         .event(clone!(state => move|evt :events::CustomToggle| {
                    //             state.display_score.set(evt.value());
                    //             update_course_settings(Rc::clone(&state));
                    //         }))
                    //     }))
                    //     .text(STR_DISPLAY_SCORE)
                    // }),
                    html!("label", {
                        .child(html!("input-switch", {
                            .property_signal("enabled", state.track_assessments.signal())
                            .event(clone!(state => move|evt :events::CustomToggle| {
                                state.track_assessments.set(evt.value());
                                update_course_settings(Rc::clone(&state));
                            }))
                        }))
                        .text(STR_ASSESSMENT_MODE)
                    }),
                    html!("label", {
                        .child(html!("input-switch", {
                            .property_signal("enabled", state.drag_assist.signal())
                            .event(clone!(state => move|evt :events::CustomToggle| {
                                state.drag_assist.set(evt.value());
                                update_course_settings(Rc::clone(&state));
                            }))
                        }))
                        .text(STR_DRAG_ASSIST)
                    }),
                ])
            }),
        ])
    })
}
