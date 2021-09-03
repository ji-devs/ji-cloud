use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::signal::{Signal, SignalExt};
use components::{
    tabs::{MenuTab, MenuTabKind},
    image::search::dom::render as render_image_search,
    text_editor::dom::render_controls as render_text_editor,
    audio::input::AudioInput,
};

pub fn render_step_2(state: Rc<Step2>) -> Dom {
    html!("menu-tabs", {
        .children(&mut [
            render_tab(state.clone(), MenuTabKind::Select),
            render_tab(state.clone(), MenuTabKind::Audio),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(clone!(state => move |tab| {
                    match tab {
                        Tab::Select => {
                            Some(html!("div", {.text(crate::strings::STR_SIDEBAR_SELECT) }))
                        },
                        Tab::Audio(audio_signal_fn) => {
                            Some(render_audio(state.clone(), audio_signal_fn()))
                        },
                    }
                })))
            })
        ])
    })
}


fn render_tab(state: Rc<Step2>, tab_kind:MenuTabKind) -> Dom {
    MenuTab::render(
        MenuTab::new(
            tab_kind,
            false,
            clone!(state => move || state.tab.signal_ref(clone!(tab_kind => move |curr| {
                curr.kind() == tab_kind
            }))),
            clone!(state, tab_kind => move || {
                state.tab.set(Tab::new(state.base.clone(), tab_kind));
            })
        ),
        Some("tabs")
    )
}

fn render_audio(state: Rc<Step2>, audio_state_signal: impl Signal<Item = Option<Rc<AudioInput>>> + 'static) -> Dom {
    html!("empty-fragment", {
        .child_signal(audio_state_signal.map(|audio_state| Some({
            match audio_state {
                Some(audio_state) => {
                    AudioInput::render(audio_state, None)
                },
                None => {
                    html!("div", {.text("TODO! (disabled audio input)") })
                }
            }
        })))
    })
}
