use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::signal::{Signal, SignalExt};
use components::{
    image::search::dom::render as render_image_search,
    text_editor::dom::render_controls as render_text_editor,
    audio_input::{state::State as AudioInputState, dom::render as render_audio_input},
};

pub fn render_step_2(state: Rc<Step2>) -> Dom {
    html!("menu-tabs", {
        .children(&mut [
            render_tab(state.clone(), TabKind::Select),
            render_tab(state.clone(), TabKind::Audio),
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


fn render_tab(state: Rc<Step2>, tab_kind:TabKind) -> Dom {
    html!("menu-tab-with-title", {
        .property("slot", "tabs")
        .property("kind", tab_kind.as_str())
        .property_signal("active", state.tab.signal_ref(clone!(tab_kind => move |curr| {
            curr.kind() == tab_kind
        })))
        .event(clone!(state, tab_kind => move |evt:events::Click| {
            state.tab.set(Tab::new(state.base.clone(), tab_kind));
        }))
    })
}

fn render_audio(state: Rc<Step2>, audio_state_signal: impl Signal<Item = Option<Rc<AudioInputState>>> + 'static) -> Dom {
    html!("empty-fragment", {
        .child_signal(audio_state_signal.map(|audio_state| Some({
            match audio_state {
                Some(audio_state) => {
                    render_audio_input(audio_state, None)
                },
                None => {
                    html!("div", {.text("TODO! (disabled audio input)") })
                }
            }
        })))
    })
}
