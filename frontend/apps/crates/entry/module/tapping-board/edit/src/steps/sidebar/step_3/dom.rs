use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt
};
use components::{
    audio_input::dom::render as render_audio_input,
};

pub fn render(state: Rc<Step3>) -> Dom {

    html!("menu-tabs", {
        .visible_signal(state.has_tab_signal())
        .children(&mut [
            render_tab(state.clone(), TabKind::Audio),
            render_tab(state.clone(), TabKind::Text),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(clone!(state => move |tab| {
                    tab.map(|tab| {
                        match tab {
                            Tab::Text(index, text_state) => {
                                html!("div", {
                                    .child(html!("input-form-textarea", {
                                        .property_signal("value", text_state.signal_cloned().map(|text| {
                                            text.unwrap_or_default()
                                        }))
                                        .property("label", crate::strings::step_3::STR_LABEL)
                                        .property("placeholder", crate::strings::step_3::STR_PLACEHOLDER)
                                        .property("rows", 4)
                                        //Input is just local 
                                        //Change pushes history and sets at a higher level
                                        .event(clone!(text_state => move |evt:events::CustomInput| {
                                            let value = evt.value();
                                            text_state.set(if value.is_empty() { None } else { Some(value) });
                                        }))
                                        .event(clone!(state => move |evt:events::CustomChange| {
                                            let value = evt.value();

                                            state.base.set_trace_meta_text(index, if value.is_empty() { None } else { Some(value) });
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
                    })
                })))
            })
        ])
    })
}


fn render_tab(state: Rc<Step3>, tab_kind:TabKind) -> Dom {
    html!("menu-tab", {
        .property("slot", "tabs")
        .property_signal("active", state.tab.signal_ref(clone!(tab_kind => move |curr| {
            match curr {
                Some(curr) => curr.kind() == tab_kind,
                None => false
            }
        })))
        .child(html!("menu-tab-title", {
            .property("kind", tab_kind.as_str())
        }))
        .event(clone!(state, tab_kind => move |evt:events::Click| {
            state.activate_tab(tab_kind);
        }))
    })
}
