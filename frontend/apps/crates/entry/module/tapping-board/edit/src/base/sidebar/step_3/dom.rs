use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt}
};
use components::{
    audio_input::dom::render as render_audio_input,
};
use web_sys::HtmlTextAreaElement;

pub fn render(state: Rc<Step3>) -> Dom {

    html!("empty-fragment", {
        .child_signal(
            //we need both an ability to change tabs, and to know if we should show tabs
            //so get a Mutable<Option<TabKind>>
            state.selected_tab_signal().map(clone!(state => move |selected_tab| {
                selected_tab.signal_cloned().map(clone!(selected_tab, state => move |kind| {
                    //from selected_tab kind is a None, no trace is selected - don't show anything 
                    kind.map(|_| {
                        //otherwise, it means a trace is selected
                        html!("menu-tabs", {
                            .children(&mut [
                                //pass down our mutable so that we can switch tabs
                                render_tab(state.clone(), TabKind::Audio, selected_tab.clone()),
                                render_tab(state.clone(), TabKind::Text, selected_tab.clone()),
                                html!("module-sidebar-body", {
                                    .property("slot", "body")
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
                        })
                    })
                }))
            }))
            .flatten()
        )
    })
}


fn render_tab(state: Rc<Step3>, tab_kind:TabKind, selected_tab: Mutable<Option<TabKind>>) -> Dom {
    html!("menu-tab-with-title", {
        .property("slot", "tabs")
        .property("kind", tab_kind.as_str())
        .property_signal("active", selected_tab.signal_ref(clone!(tab_kind => move |curr| {
            match curr {
                Some(curr) => *curr == tab_kind,
                None => false
            }
        })))
        .event(clone!(selected_tab, tab_kind => move |evt:events::Click| {
            selected_tab.set_neq(Some(tab_kind));
        }))
    })
}

fn render_tab_body(state: Rc<Step3>, tab: Tab) -> Dom {
    match tab {
        Tab::Text(index, text_state) => {
            html!("div", {
                .child(html!("input-wrapper", {
                    .property("label", crate::strings::step_3::STR_LABEL)
                    .child(html!("textarea", {
                        .property_signal("value", text_state.signal_cloned().map(|text| {
                            text.unwrap_or_default()
                        }))
                        .property("placeholder", crate::strings::step_3::STR_PLACEHOLDER)
                        .property("rows", 4)
                        //Input is just local 
                        //Change pushes history and sets at a higher level
                        .event(clone!(text_state => move |evt:events::Input| {
                            let value = evt.value().unwrap_or_default();
                            text_state.set(if value.is_empty() { None } else { Some(value) });
                        }))
                        .event(clone!(state => move |evt:events::Change| {
                            let target = evt.dyn_target::<HtmlTextAreaElement>().unwrap();
                            let value = target.value();

                            state.base.set_trace_meta_text(index, if value.is_empty() { None } else { Some(value) });
                        }))
                    }))
                }))
                .child(html!("button", {
                    .text("Preview")
                    .event(clone!(state, text_state => move |evt:events::Click| {
                        state.start_preview(index)
                    }))
                }))
            })
        },
        Tab::Audio(audio_state) => {
            render_audio_input(audio_state.clone(), None)
        }
    }
}
