use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::jig::TextDirection;
use utils::events;

use crate::edit::sidebar::jig::settings::{
    actions::{set_active_popup, update_jig_settings},
    state::{ActiveSettingsPopup, FeedbackTab},
};

use super::super::state::JigSettings;

const STR_SCORING: &str = "Scoring & Assessment";
// const STR_ASSESSMENT_MODE: &str = "Assessment mode";
// const STR_DRAG_ASSIST: &str = "Drag & Drop assist";

impl JigSettings {
    pub fn render_main(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("jig-settings", {
            .prop("slot", "overlay")
            .children(&mut [
                html!("button-icon", {
                    .prop("icon", "x")
                    .prop("slot", "close")
                    .event(clone!(state => move |_:events::Click| {
                        state.active_popup.set(None);
                    }))
                }),
                html!("jig-settings-button", {
                    .prop("slot", "creator")
                    .prop("kind", "background")
                    .event(clone!(state => move|_: events::Click| {
                        set_active_popup(Rc::clone(&state), ActiveSettingsPopup::Background);
                    }))
                }),
                html!("jig-settings-button", {
                    .prop("slot", "creator")
                    .prop("kind", "feedback")
                    .event(clone!(state => move|_: events::Click| {
                        set_active_popup(Rc::clone(&state), ActiveSettingsPopup::Feedback(FeedbackTab::Positive));
                    }))
                }),
                html!("jig-preview-settings", {
                    .prop("slot", "preview")
                    .children(&mut [
                        html!("input-switch-direction", {
                            .prop_signal("direction", state.jig.direction.signal().map(|dir| {
                                match dir {
                                    TextDirection::LeftToRight => "ltr",
                                    TextDirection::RightToLeft => "rtl",
                                }
                            }))
                            .event(clone!(state => move|evt :events::CustomDirection| {
                                state.jig.direction.set(evt.direction());
                                update_jig_settings(Rc::clone(&state));
                            }))
                        }),
                        html!("label", {
                            .child(html!("input-switch", {
                                .prop_signal("enabled", state.jig.scoring.signal())
                                .event(clone!(state => move|evt :events::CustomToggle| {
                                    state.jig.scoring.set(evt.value());
                                    update_jig_settings(Rc::clone(&state));
                                }))
                            }))
                            .text(STR_SCORING)
                        }),
                        // html!("label", {
                        //     .child(html!("input-switch", {
                        //         .prop_signal("enabled", state.jig.drag_assist.signal())
                        //         .event(clone!(state => move|evt :events::CustomToggle| {
                        //             state.jig.drag_assist.set(evt.value());
                        //             update_jig_settings(Rc::clone(&state));
                        //         }))
                        //     }))
                        //     .text(STR_DRAG_ASSIST)
                        // }),
                    ])
                }),
            ])
        })
    }
}
