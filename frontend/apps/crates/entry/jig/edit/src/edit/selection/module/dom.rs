use super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::jig::ModuleKind;
use std::rc::Rc;
use utils::{events, unwrap::UnwrapJiExt};

pub struct ModuleDom {}

impl ModuleDom {
    pub fn render(kind: ModuleKind) -> Dom {
        let state = Rc::new(State::new(kind));

        html!("jig-edit-module-card", {
            .property("slot", "modules")
            .property("module", kind.as_str())
            .event(clone!(state => move |_: events::PointerEnter| {
                state.hover.set(true);
            }))
            .event(clone!(state => move |_: events::PointerLeave| {
                state.hover.set(false);
            }))
            .event(clone!(state => move |evt: events::PointerDown| {
                let elem = evt.dyn_target().unwrap_ji();
                state.on_pointer_down(&elem, evt.x(), evt.y());
            }))
            .child(html!("img-ui", {
                .property("slot", "stationery")
                .property("draggable", false)
                .property_signal("path", state.hover_or_drag_signal().map(clone!(state => move |hover| {
                    let suffix = match hover {
                        true => "-hover",
                        false => "",
                    };
                    format!("entry/jig/modules/large/{}{}.svg", state.kind.as_str(), suffix)
                })))
                .style_signal("filter", state.drag.signal_ref(|drag| {
                    match drag {
                        Some(_) => "grayscale(100%) opacity(0.5)",
                        None => "none",
                    }
                }))
                .style("touch-action", "none")
            }))
            .child_signal(state.drag.signal_ref(clone!(state => move|drag| {
                drag.as_ref().map(|drag| {
                    html!("img-ui", {
                        .property("slot", "dragged")
                        .property("path", &format!("entry/jig/modules/large/{}-hover.svg", state.kind.as_str()))
                        .style_signal("transform", drag.transform_signal())
                        .global_event(clone!(state, drag => move |evt: events::PointerMove| {
                            state.on_pointer_move(&drag, evt.x(), evt.y());
                        }))
                        .global_event(clone!(state => move |evt: events::PointerUp| {
                            state.on_pointer_up(evt.x(), evt.y());
                        }))
                        .global_event(clone!(state => move |_:events::PointerCancel| {
                            state.stop_drag();
                        }))
                    })
                })
            })))
        })
    }
}
