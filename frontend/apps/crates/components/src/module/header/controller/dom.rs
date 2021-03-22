use dominator::{Dom, html, clone};
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use utils::events;
use futures_signals::signal::SignalExt;
use crate::module::history::state::HistoryState;

pub struct ControllerDom {
}

impl ControllerDom {
    pub fn render<T, F>(history: Rc<HistoryState<T>>, mut on_history_change: F) -> Dom 
    where T: Clone + 'static,
          F: FnMut(Option<T>) + 'static
    {
        html!("module-header-controller", {
            .property("slot", "controller")
            .property_signal("undoable", history.undoable())
            .property_signal("redoable", history.redoable())
            .event(clone!(history => move |evt:events::CustomString| {
                match evt.value().as_ref() {
                    //unchecked is ok because these events
                    //only happen when the custom element allows
                    "undo" => {
                        on_history_change(history.undo_unchecked());
                    },
                    "redo" => {
                        on_history_change(history.redo_unchecked());
                    },
                    "preview" => {
                    }
                    _ => {}
                };
            }))
        })
    }
}
