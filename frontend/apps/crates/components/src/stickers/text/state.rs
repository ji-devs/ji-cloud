use futures_signals::signal::Mutable;

use std::rc::Rc;

use dominator::clone;
use shared::domain::jig::module::body::{Transform, _groups::design::Text as RawText};

use crate::{
    text_editor::state::State as TextEditorState,
    transform::state::{TransformCallbacks, TransformState},
};

#[derive(Clone)]
pub struct Text {
    pub value: Mutable<String>,
    pub transform: Rc<TransformState>,
    pub editor: Rc<TextEditorState>,
    pub is_editing: Mutable<bool>,
}

impl Text {
    pub fn new(
        editor: Rc<TextEditorState>,
        text: &RawText,
        on_transform_finished: Option<impl Fn(Transform) + 'static>,
        on_blur: Option<impl Fn() + 'static>,
    ) -> Self {
        let text = text.clone();
        let is_editing = Mutable::new(false);

        let transform_callbacks = TransformCallbacks::new(
            on_transform_finished,
            //transform double-click
            Some(clone!(is_editing => move || {
                is_editing.set_neq(true)
            })),
            on_blur.map(clone!(is_editing => move|on_blur| {
                move || {
                    if !is_editing.get() {
                        on_blur();
                    }
                }
            }))
        );
        Self {
            value: Mutable::new(text.value),
            transform: Rc::new(TransformState::new(
                text.transform,
                None,
                true,
                transform_callbacks,
            )),
            editor,
            is_editing,
        }
    }

    pub fn to_raw(&self) -> RawText {
        RawText {
            value: self.value.get_cloned(),
            transform: self.transform.get_inner_clone(),
        }
    }
}
