use dominator::{Dom, html, clone};
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use utils::events;
use futures_signals::signal::SignalExt;
use crate::module::history::state::HistoryState;

pub struct ControllerDom {
}

//TODO - move on_undoredo into HistoryState itself
impl ControllerDom {
    pub fn render<T, UR, F>(history: Rc<HistoryState<T, F>>, mut on_undoredo: UR) -> Dom 
    where T: Clone + 'static,
          UR: FnMut(Option<T>) + 'static,
          F: Fn(Option<T>) + 'static
    {
        html!("module-header-controller", {
            .property("slot", "controller")
            .property_signal("undoable", history.undoable())
            .property_signal("redoable", history.redoable())
            .event(clone!(history => move |evt:events::CustomString| {
                match evt.value().as_ref() {
                    "undo" => {
                        on_undoredo(history.undo());
                    },
                    "redo" => {
                        on_undoredo(history.redo());
                    },
                    "preview" => {
                    }
                    _ => {}
                };
            }))
        })
    }
}
