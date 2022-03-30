use super::state::*;
use crate::base::{
    main::{drag::MainDrag, select::MainSelect},
    sidebar::state::{StickerPhase, TracePhase},
};
use components::{
    audio::input::AudioInput,
    tabs::{MenuTab, MenuTabKind},
};
use dominator::{clone, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use std::rc::Rc;

pub fn render_step_3(state: Rc<Step3>) -> Dom {
    html!("menu-tabs", {
        .future(state.tab.signal_cloned().for_each(clone!(state => move |tab| {
            let (sticker_phase, trace_phase) = match tab {
                Tab::Select | Tab::Audio(_) => (StickerPhase::Select(MainSelect::new(state.sidebar.base.clone())), None),
                Tab::Trace => (StickerPhase::Static, Some(TracePhase::Edit)),
                Tab::Place => (StickerPhase::Drag(MainDrag::new(state.sidebar.base.clone())), Some(TracePhase::Show)),
            };

            state.sidebar.sticker_phase.set_neq(Some(sticker_phase));
            state.sidebar.trace_phase.set_neq(trace_phase);

            state.sidebar.tab_kind.set_neq(Some(tab.kind()));
            async move {}
        })))
        .children(&mut [
            render_tab(state.clone(), MenuTabKind::Select),
            render_tab(state.clone(), MenuTabKind::Audio),
            render_tab(state.clone(), MenuTabKind::Trace),
            render_tab(state.clone(), MenuTabKind::Place),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(clone!(state => move |tab| {
                    match tab {
                        Tab::Select => {
                            Some(html!("div", {
                                .text(crate::strings::STR_SIDEBAR_SELECT)
                            }))
                        },
                        Tab::Audio(audio_signal_fn) => {
                            Some(render_audio(state.clone(), audio_signal_fn()))
                        },
                        Tab::Trace => {
                            Some(html!("sidebar-empty", {
                                .property("label", crate::strings::STR_SIDEBAR_TRACE)
                                .property("imagePath", "module/_common/edit/sidebar/illustration-trace-area.svg")
                            }))
                        },
                        Tab::Place => {
                            Some(html!("module-sidebar-drag-prompt"))
                        },
                    }
                })))
            })
        ])
    })
}

fn render_tab(state: Rc<Step3>, tab_kind: MenuTabKind) -> Dom {
    MenuTab::render(
        MenuTab::new(
            tab_kind,
            false,
            true,
            clone!(state => move || state.tab.signal_ref(clone!(tab_kind => move |curr| {
                curr.kind() == tab_kind
            }))),
            clone!(state, tab_kind => move || {
                state.tab.set(Tab::new(state.sidebar.base.clone(), tab_kind));
            }),
        ),
        Some("tabs"),
    )
}

fn render_audio(
    _state: Rc<Step3>,
    audio_state_signal: impl Signal<Item = Option<Rc<AudioInput>>> + 'static,
) -> Dom {
    html!("empty-fragment", {
        .child_signal(audio_state_signal.map(|audio_state| Some({
            match audio_state {
                Some(audio_state) => {
                    AudioInput::render(audio_state, None)
                },
                None => {
                    html!("div", {.text("Select an item to add audio") })
                }
            }
        })))
    })
}
