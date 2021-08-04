use std::rc::Rc;

use dominator::{Dom, clone, html};

use futures_signals::signal::SignalExt;
use utils::{events, jig::AudioBackgroundExt};
use shared::domain::jig::AudioBackground;
use crate::edit::sidebar::settings::{actions::{self, set_active_popup}, dom::STR_BACK_TO_SETTINGS, state::ActiveSettingsPopup};

use super::super::state::State;

pub fn render(state: Rc<State>) -> Dom {
    html!("jig-audio-body", {
        .property("slot", "overlay")
        .property("kind", "background")
        .children(&mut [
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
        .children(AudioBackground::variants().iter().map(clone!(state => move|option| {
            line(Rc::clone(&state), option)
        })).collect::<Vec<Dom>>())
    })
}

fn line(state: Rc<State>, option: &AudioBackground) -> Dom {
    html!("jig-audio-line", {
        .property("slot", "lines")
        .property("label", option.display_name())
        .property_signal("selected", state.background_audio.signal_cloned().map(clone!(option => move|selected_audio| {
            match selected_audio {
                Some(selected_audio) if selected_audio == option => {
                    true
                },
                _ => {
                    false
                },
            }
        })))
        .event(clone!(state, option => move |_ :events::Click| {
            actions::on_background_audio_click(Rc::clone(&state), option.clone());
        }))
        .children(&mut [
            html!("jig-audio-play-pause", {
                .property("mode", "play")
                .property("slot", "play-pause")
            }),
        ])
    })
}
