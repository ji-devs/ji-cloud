use futures_signals::signal::Mutable;
use js_sys::Reflect;
use utils::unwrap::UnwrapJiExt;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use std::rc::Rc;

use dominator::clone;
use shared::domain::module::body::{
    HoverAnimation, StickerHidden, Transform, _groups::design::Text as RawText,
};

use crate::{
    text_editor::TextEditor,
    transform::state::{TransformCallbacks, TransformState},
};

#[derive(Clone)]
pub struct Text {
    pub value: Mutable<String>,
    pub transform: Rc<TransformState>,
    pub hidden: Mutable<Option<StickerHidden>>,
    pub hover_animation: Mutable<Option<HoverAnimation>>,
    pub editor: Rc<TextEditor>,
    /// Optional reference to the wysiwyg-output-renderer
    pub renderer_ref: Mutable<Option<HtmlElement>>,
    pub measurer_ref: Mutable<Option<HtmlElement>>,
    pub is_editing: Mutable<bool>,
    pub is_editable: Mutable<bool>,
    pub can_delete: Mutable<bool>,
    pub highlight: Mutable<bool>,
}

impl Text {
    pub fn new(
        editor: Rc<TextEditor>,
        text: &RawText,
        on_transform_finished: Option<impl Fn(Transform) + 'static>,
    ) -> Self {
        let text = text.clone();
        let is_editing = Mutable::new(false);
        let is_editable = Mutable::new(true);

        let transform_callbacks = TransformCallbacks::new(
            on_transform_finished,
            //transform double-click
            Some(clone!(is_editable, is_editing => move || {
                if is_editable.get() {
                    is_editing.set_neq(true)
                }
            })),
            None::<fn()>,
        );
        Self {
            value: Mutable::new(text.value),
            transform: Rc::new(TransformState::new(
                text.transform,
                None,
                true,
                transform_callbacks,
            )),
            hover_animation: Mutable::new(text.hover_animation),
            hidden: Mutable::new(text.hidden),
            editor,
            renderer_ref: Mutable::new(None),
            measurer_ref: Mutable::new(None),
            is_editing,
            is_editable,
            can_delete: Mutable::new(true),
            highlight: Mutable::new(false),
        }
    }

    pub fn to_raw(&self) -> RawText {
        RawText {
            value: self.value.get_cloned(),
            transform: self.transform.get_inner_clone(),
            hover_animation: self.hover_animation.get(),
            hidden: self.hidden.get_cloned(),
        }
    }

    /// Retrieves the text value without any formatting or tags
    pub fn get_text_value(&self) -> Option<String> {
        let renderer_ref = &*self.renderer_ref.lock_ref();
        renderer_ref
            .clone()
            .map(|renderer_ref| {
                let value =
                    Reflect::get(&renderer_ref, &JsValue::from_str("textValue")).unwrap_ji();
                value.as_string()
            })
            .unwrap_or_default()
    }
}
