use std::rc::Rc;

use dominator::{Dom, clone, html};

use futures_signals::signal::SignalExt;
use utils::{events, jig::{AudioFeedbackNegativeExt, AudioFeedbackPositiveExt}};
use shared::domain::jig::{AudioFeedbackNegative, AudioFeedbackPositive};
use crate::edit::sidebar::settings::{actions::set_active_popup, dom::STR_BACK_TO_SETTINGS, state::{ActiveSettingsPopup, FeedbackTab}};

use super::super::state::State;

const STR_CORRECT: &'static str = "Correct answer";
const STR_MISTAKE: &'static str = "Mistake";

pub fn render(state: Rc<State>, tab: FeedbackTab) -> Dom {
    html!("jig-audio-body", {
        .property("slot", "overlay")
        .property("kind", "feedback")
        .children(&mut [
            html!("label", {
                .property("slot", "correct-mistake")
                .child(html!("input", {
                    .property("name", "correct-mistake")
                    .property("type", "radio")
                    .property("checked", tab == FeedbackTab::Positive)
                    .event(clone!(state => move |_:events::Input| {
                        state.active_popup.set(Some(ActiveSettingsPopup::Feedback(FeedbackTab::Positive)));
                    }))
                }))
                .text(STR_CORRECT)
            }),
            html!("label", {
                .property("slot", "correct-mistake")
                .child(html!("input", {
                    .property("name", "correct-mistake")
                    .property("type", "radio")
                    .property("checked", tab == FeedbackTab::Negative)
                    .event(clone!(state => move |_:events::Input| {
                        state.active_popup.set(Some(ActiveSettingsPopup::Feedback(FeedbackTab::Negative)));
                    }))
                }))
                .text(STR_MISTAKE)
            }),
            html!("button-rect", {
                .property("kind", "text")
                .property("slot", "back")
                .text(STR_BACK_TO_SETTINGS)
                .event(clone!(state => move|_: events::Click| {
                    set_active_popup(Rc::clone(&state), ActiveSettingsPopup::Main);
                }))
            }),
            html!("button-icon", {
                .property("icon", "x")
                .property("slot", "close")
                .event(clone!(state => move |_:events::Click| {
                    state.active_popup.set(None);
                }))
            }),
            // html!("input-search", {
            //     .property("slot", "search")
            // }),
        ])
        .apply(|dom| {
            match tab {
                FeedbackTab::Positive => {
                    dom.children(AudioFeedbackPositive::variants().iter().map(clone!(state => move|option| {
                        line_positive(Rc::clone(&state), option)
                    })).collect::<Vec<Dom>>())
                },
                FeedbackTab::Negative => {
                    dom.children(AudioFeedbackNegative::variants().iter().map(clone!(state => move|option| {
                        line_negative(Rc::clone(&state), option)
                    })).collect::<Vec<Dom>>())
                },
            }
        })
    })
}

fn line_positive(state: Rc<State>, option: &AudioFeedbackPositive) -> Dom {
    html!("jig-audio-line", {
        .property("slot", "lines")
        .property("label", option.display_name())
        .property_signal("selected", state.feedback_positive.signal_cloned().map(clone!(option => move|feedback_positive| {
            feedback_positive.contains(&option)
        })))
        .event(clone!(state, option => move |_ :events::Click| {
            let mut feedback_positive = state.feedback_positive.lock_mut();
            match feedback_positive.contains(&option) {
                true => feedback_positive.remove(&option),
                false => feedback_positive.insert(option.clone()),
            };
        }))
        .children(&mut [
            html!("jig-audio-play-pause", {
                .property("mode", "play")
                .property("slot", "play-pause")
            }),
        ])
    })
}

fn line_negative(state: Rc<State>, option: &AudioFeedbackNegative) -> Dom {
    html!("jig-audio-line", {
        .property("slot", "lines")
        .property("label", option.display_name())
        .property_signal("selected", state.feedback_negative.signal_cloned().map(clone!(option => move|feedback_negative| {
            feedback_negative.contains(&option)
        })))
        .event(clone!(state, option => move |_ :events::Click| {
            let mut feedback_negative = state.feedback_negative.lock_mut();
            match feedback_negative.contains(&option) {
                true => feedback_negative.remove(&option),
                false => feedback_negative.insert(option.clone()),
            };
        }))
        .children(&mut [
            html!("jig-audio-play-pause", {
                .property("mode", "play")
                .property("slot", "play-pause")
            }),
        ])
    })
}
