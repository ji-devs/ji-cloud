use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use std::cell::RefCell;
use shared::{domain::{image::ImageId, jig::module::body::{_groups::design::Text as RawText, Transform}}, media::MediaLibrary};
use dominator::clone;

use crate::{
    transform::state::{TransformState, TransformCallbacks},
    text_editor::state::State as TextEditorState
};

#[derive(Clone)]
pub struct Text {
    pub value: Mutable<String>,
    pub transform: Rc<TransformState>,
    pub editor: Rc<TextEditorState>,
    pub is_editing: Mutable<bool>,
}

impl Text {
    pub fn new(editor: Rc<TextEditorState>, text:&RawText, on_transform_finished: Option<impl Fn(Transform) + 'static>) -> Self {
        let text = text.clone();
        let is_editing = Mutable::new(false);

        let transform_callbacks = TransformCallbacks::new(
            on_transform_finished,
            //transform double-click
            Some(clone!(is_editing => move || {
                is_editing.set_neq(true)
            }))
        );
        Self {
            value: Mutable::new(text.value),
            transform: Rc::new(TransformState::new(text.transform, None, true, transform_callbacks)),
            editor,
            is_editing,
        }
    }

    pub fn to_raw(&self) -> RawText {
        RawText {
            value: self.value.get_cloned(),
            transform: self.transform.get_inner_clone()
        }
    }
}


