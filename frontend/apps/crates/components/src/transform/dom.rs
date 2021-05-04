use dominator::{Dom, html, clone};
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use crate::module::history::state::HistoryState;
use web_sys::HtmlElement;
use super::state::*;

pub struct TransformDom {
}

//TODO - move on_undoredo into HistoryState itself
impl TransformDom {
    pub fn render(state: Rc<TransformState>) -> Dom {

        html!("transform-box", {
            .visible_signal(state.visible.signal_cloned())
            .style("display", "block")
            .style_signal("transform", state.matrix_string_signal().map(|x| x))
            .property("unit", "rem")
            .style_signal("width", state.width_signal().map(|x| format!("{}rem", x)))
            .style_signal("height", state.height_signal().map(|x| format!("{}rem", x)))
            .property_signal("width", state.width_signal())
            .property_signal("height", state.height_signal())
            .event(clone!(state => move |evt:super::events::Move| {
                let data = evt.data();
                state.start_tracking_action(Action::Move, data.x as i32, data.y as i32);
            }))
            .event(clone!(state => move |evt:super::events::Rotate| {
                let data = evt.data();
                state.start_tracking_action(Action::Rotate, data.x as i32, data.y as i32);
            }))
            .event(clone!(state => move |evt:super::events::Resize| {
                let data = evt.data();
                let scale_from = data.scale_from();
                state.start_tracking_action(Action::Scale(scale_from, false), data.x as i32, data.y as i32);
            }))
            .global_event_preventable(clone!(state => move |evt:events::MouseUp| {
                state.mouse_up(evt.x() as i32, evt.y() as i32);
            }))
            .global_event_preventable(clone!(state => move |evt:events::MouseMove| {
                state.mouse_move(evt.x() as i32, evt.y() as i32);
            }))
        })
    }

}
