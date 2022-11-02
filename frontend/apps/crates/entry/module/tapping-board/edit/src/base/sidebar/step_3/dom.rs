use super::state::*;
use std::rc::Rc;

use components::{
    audio::input::AudioInput,
    hebrew_buttons::HebrewButtons,
    tabs::{MenuTab, MenuTabKind},
};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use utils::prelude::*;
use web_sys::HtmlTextAreaElement;

const STR_EMPTY_SELECTION: &str = "Trace and select to add audio or label";

pub fn render(state: Rc<Step3>) -> Dom {
    html!("empty-fragment", {
        .style("display", "contents")
        .child_signal(
            //we need both an ability to change tabs, and to know if we should show tabs
            //so get a Mutable<Option<MenuTabKind>>
            state.selected_tab_signal().map(clone!(state => move |selected_tab| {
                selected_tab.signal_cloned().map(clone!(selected_tab, state => move |kind| {
                    //from selected_tab kind is a None, no trace is selected - don't show anything
                    //TODO- empty-fragment so we can set tab_index?

                    match kind {
                        Some(_) => {
                            //otherwise, it means a trace is selected
                            Some(html!("menu-tabs", {
                                .future(selected_tab.signal_cloned().dedupe().for_each(clone!(state, selected_tab => move |kind| {
                                    state.sidebar.tab_kind.set(kind);

                                    // A trace is selected, so there should be some tabs rendered,
                                    // tell Continue to navigate to the next tab
                                    state.sidebar.base.continue_next_fn.set(Some(Rc::new(clone!(selected_tab => move || {
                                        if let Some(kind) = next_kind(&kind.unwrap_ji()) {
                                            selected_tab.set_neq(Some(kind));
                                            true
                                        } else {
                                            false
                                        }
                                    }))));
                                    async move {}
                                })))
                                .children(&mut [
                                    //pass down our mutable so that we can switch tabs
                                    render_tab(state.clone(), MenuTabKind::Audio, selected_tab.clone()),
                                    render_tab(state.clone(), MenuTabKind::Label, selected_tab.clone()),
                                    html!("module-sidebar-body", {
                                        .property("slot", "body")
                                        .style("overflow", "inherit") // Inherit overflow otherwise the Hebrew controls will be hidden
                                        .child_signal(
                                            //based on the selected tab kind, create and render the tab state
                                            state
                                                .tab_signal(selected_tab.signal())
                                                .map(clone!(state => move |tab| {
                                                    tab.map(|tab| {
                                                        render_tab_body(state.clone(), tab)
                                                    })
                                                }))
                                        )
                                    })
                                ])
                            }))
                        }
                        None => {
                            // When no traces are selected, we can just continue to the next step.
                            state.sidebar.base.continue_next_fn.set(Some(Rc::new(|| false)));
                            Some(html!("sidebar-empty", {
                                .property("label", STR_EMPTY_SELECTION)
                                .property("imagePath", "module/_common/edit/sidebar/illustration-trace-area.svg")
                            }))
                        }
                    }
                }))
            }))
            .flatten()
        )
    })
}

fn render_tab(
    _state: Rc<Step3>,
    tab_kind: MenuTabKind,
    selected_tab: Mutable<Option<MenuTabKind>>,
) -> Dom {
    MenuTab::render(
        MenuTab::new(
            tab_kind,
            false,
            true,
            clone!(selected_tab => move || selected_tab.signal_ref(clone!(tab_kind => move |curr| {
                match curr {
                    Some(curr) => *curr == tab_kind,
                    None => false
                }
            }))),
            clone!(tab_kind => move || {
                selected_tab.set_neq(Some(tab_kind));
            }),
        ),
        Some("tabs"),
    )
}

fn render_tab_body(state: Rc<Step3>, tab: Tab) -> Dom {
    match tab {
        Tab::Label(index, text_state) => {
            html!("tapping-board-interaction-label", {
                .child(html!("input-wrapper", {
                    .child({
                        HebrewButtons::reveal().render(Some("hebrew-inputs"))
                    })
                    .property("label", crate::strings::step_3::STR_LABEL)
                    .child(html!("textarea" => HtmlTextAreaElement, {
                        .with_node!(elem => {
                            .attribute("dir", "auto")
                            .property_signal("value", text_state.signal_cloned().map(|text| {
                                text.unwrap_or_default()
                            }))
                            .property("placeholder", crate::strings::step_3::STR_PLACEHOLDER)
                            .property("rows", 4)
                            //Input is just local
                            //Change pushes history and sets at a higher level
                            .event(clone!(text_state => move |_:events::Input| {
                                let value = elem.value();
                                text_state.set(if value.is_empty() { None } else { Some(value) });
                            }))
                            .event(clone!(state => move |evt:events::Change| {
                                let target = evt.dyn_target::<HtmlTextAreaElement>().unwrap_ji();
                                let value = target.value();

                                state.sidebar.base.traces.set_text(index, if value.is_empty() { None } else { Some(value) });
                            }))
                        })
                    }))
                }))
                .child_signal(text_state.signal_cloned().map(clone!(text_state => move |text| {
                    text.map(|_text| {
                        html!("interaction-delete-action", {
                            .property("slot", "delete")
                            .event(clone!(text_state => move |_evt:events::Click| {
                                text_state.set_neq(None);
                            }))
                        })
                    })
                })))
                .child(html!("interaction-preview-action", {
                    .property("slot", "main-action")
                    .property_signal("disabled", text_state.signal_cloned().map(|text| text.is_none()))
                    .event(clone!(state => move |_evt:events::Click| {
                        state.start_preview(index)
                    }))
                }))
            })
        }
        Tab::Audio(audio_state) => AudioInput::render(audio_state, None),
    }
}
