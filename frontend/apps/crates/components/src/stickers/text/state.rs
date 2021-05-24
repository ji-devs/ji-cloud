use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use std::cell::RefCell;
use shared::{domain::{image::ImageId, jig::module::body::{Text as RawText, Transform}}, media::MediaLibrary};

use crate::{
    transform::state::TransformState,
    text_editor::state::State as TextEditorState
};

#[derive(Clone)]
pub struct Text {
    pub value: Mutable<String>,
    pub transform: Rc<TransformState>,
    pub editor: Rc<TextEditorState>,
}

impl Text {
    pub fn new(editor: Rc<TextEditorState>, text:&RawText, on_transform_finished: Option<impl Fn(Transform) + 'static>) -> Self {
        let text = text.clone();
        Self {
            value: Mutable::new(text.value),
            transform: Rc::new(TransformState::new(text.transform, None, true, on_transform_finished)),
            editor,
        }
    }

    pub fn to_raw(&self) -> RawText {
        RawText {
            value: self.value.get_cloned(),
            transform: self.transform.get_inner_clone()
        }
    }
}


