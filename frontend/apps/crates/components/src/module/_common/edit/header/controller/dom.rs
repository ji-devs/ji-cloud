use dominator::{clone, html, Dom};

use std::{fmt::Debug, rc::Rc};
use utils::{
    events,
    keyboard::{Key, KeyEvent},
};

use crate::module::_common::edit::history::state::HistoryState;

#[derive(Debug)]
pub struct ControllerDom {}

//TODO - move on_undoredo into HistoryState itself
impl ControllerDom {
    pub fn render<T, OnChangeFn, OnUndoRedoFn, OnPreviewFn>(
        history: Rc<HistoryState<T, OnChangeFn, OnUndoRedoFn>>,
        on_preview: OnPreviewFn,
    ) -> Dom
    where
        T: Clone + Debug + 'static,
        OnChangeFn: Fn(T) + 'static,
        OnUndoRedoFn: Fn(T) + 'static,
        OnPreviewFn: Fn() + 'static,
    {
        html!("module-header-controller", {
            .prop("slot", "controller")
            .prop_signal("undoable", history.undoable())
            .prop_signal("redoable", history.redoable())
            .event(clone!(history => move |evt:events::CustomString| {
                match evt.value().as_ref() {
                    "undo" => {
                        history.undo();
                    },
                    "redo" => {
                        history.redo();
                    },
                    "preview" => {
                        on_preview();
                    }
                    _ => {}
                };
            }))
            .global_event(clone!(history => move |evt: events::KeyDown| {
                let key_event = KeyEvent::from(evt);
                let key = &key_event.key;

                if let Key::Other(other) = key {
                    let other = other.to_uppercase();
                    let other: &str = other.as_ref();
                    if key_event.ctrl_cmd && !key_event.shift && other == "Z" {
                        history.undo();
                    } else {
                        let is_osx_redo = key_event.is_osx && key_event.ctrl_cmd && key_event.shift && other == "Z";
                        let is_regular_redo = !key_event.is_osx && key_event.ctrl_cmd && other == "Y";

                        if is_osx_redo || is_regular_redo {
                            history.redo();
                        }
                    }
                }
            }))
        })
    }
}
