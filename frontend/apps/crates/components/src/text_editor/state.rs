use std::cell::RefCell;
use std::future::ready;
use std::rc::Rc;

use dominator::clone;
use futures_signals::signal::{Mutable, ReadOnlyMutable, SignalExt};
use js_sys::Reflect;
use utils::{
    fonts::font_families_iter,
    themes::{ThemeId, ThemeIdExt},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

use super::callbacks::Callbacks;
use super::dom::text_editor_controls::color_controls::ColorState;
use super::wysiwyg_types::{
    ControlsChange, ControlsState, ElementType, BOLD_WEIGHT, REGULAR_WEIGHT,
};

pub struct State {
    pub controls: Mutable<ControlsState>,
    pub wysiwyg_ref: Mutable<Option<HtmlElement>>,
    pub fonts: Mutable<Vec<String>>,
    pub value: RefCell<Option<String>>,
    pub theme_id: ReadOnlyMutable<ThemeId>,
    pub color_state: RefCell<Option<Rc<ColorState>>>,
    pub callbacks: Callbacks,
}

pub const ELEMENT_DEFAULT_KEY: &'static str = "elementDefault";

impl State {
    pub fn new(
        theme_id: ReadOnlyMutable<ThemeId>,
        value: Option<String>,
        callbacks: Callbacks,
    ) -> Rc<Self> {
        let _self = Rc::new(Self {
            controls: Mutable::new(ControlsState::new()),
            wysiwyg_ref: Mutable::new(None),
            fonts: Mutable::new(vec![]),
            callbacks,
            value: RefCell::new(value),
            theme_id,
            color_state: RefCell::new(None),
        });

        *_self.color_state.borrow_mut() = Some(Rc::new(ColorState::new(_self.clone())));

        Self::handle_fonts(Rc::clone(&_self));

        _self
    }

    pub fn text_to_value(text: &str) -> String {
        format!("{{\"version\":\"0.1.0\",\"content\":[{{\"children\":[{{\"text\":\"{}\",\"element\":\"H1\"}}]}}]}}", text)
    }

    pub fn set_value(&self, value: Option<String>) {
        self.value.replace(value);
        if let Some(wysiwyg_ref) = &*self.wysiwyg_ref.lock_ref() {
            self.update_wysiwyg_value(&wysiwyg_ref);
        }
    }

    pub fn select_all(&self) {
        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.lock_ref().as_ref() {
            let select_all_method =
                Reflect::get(&wysiwyg_ref, &JsValue::from_str("selectAll")).unwrap();
            let select_all_method = select_all_method.dyn_ref::<js_sys::Function>().unwrap();
            let _ = select_all_method.call0(&wysiwyg_ref);
        }
    }

    fn update_wysiwyg_value(&self, wysiwyg_ref: &HtmlElement) {
        match &*self.value.borrow() {
            Some(value) => {
                let _ = Reflect::set(
                    &wysiwyg_ref,
                    &JsValue::from_str("valueAsString"),
                    &JsValue::from_str(&value),
                );
            }
            None => {
                let reset_value_method =
                    Reflect::get(&wysiwyg_ref, &JsValue::from_str("clearValue")).unwrap();
                let reset_value_method = reset_value_method.dyn_ref::<js_sys::Function>().unwrap();
                let _ = reset_value_method.call0(&wysiwyg_ref);
            }
        };
    }

    fn handle_fonts(state: Rc<State>) {
        spawn_local(
            state
                .theme_id
                .signal_cloned()
                .for_each(clone!(state => move |theme| {
                    let mut fonts: Vec<String> = Vec::from(theme.get_text_editor_fonts());
                    let mut static_fonts: Vec<String> = font_families_iter().map(|font_family| {
                        font_family.to_string()
                    }).collect();
                    fonts.append(&mut static_fonts);
                    state.fonts.set(fonts);
                    ready(())
                })),
        );
    }

    pub(super) fn set_wysiwyg_ref(&self, wysiwyg_ref: HtmlElement) {
        let _ = Reflect::set(
            &wysiwyg_ref,
            &JsValue::from_str(ELEMENT_DEFAULT_KEY),
            &JsValue::from_str(&ElementType::default().to_string()),
        );

        self.update_wysiwyg_value(&wysiwyg_ref);

        self.wysiwyg_ref.set(Some(wysiwyg_ref));
    }

    pub(super) fn clear_wysiwyg_ref(&self) {
        self.wysiwyg_ref.set(None);
    }

    pub(super) fn toggle_bold(&self) {
        let mut weight = self.controls.lock_mut().weight;
        weight = if weight == BOLD_WEIGHT {
            REGULAR_WEIGHT
        } else {
            BOLD_WEIGHT
        };

        self.set_control_value(ControlsChange::Weight(weight));
    }
    pub(super) fn toggle_italic(&self) {
        let italic_active = self.controls.lock_ref().italic;
        self.set_control_value(ControlsChange::Italic(!italic_active));
    }
    pub(super) fn toggle_underline(&self) {
        let underline_active = self.controls.lock_ref().underline;
        self.set_control_value(ControlsChange::Underline(!underline_active));
    }

    pub(super) fn set_control_value(&self, control: ControlsChange) {
        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.lock_ref().as_ref() {
            let (key, value) = control.to_js_key_value();
            let set_control_value_method =
                Reflect::get(&wysiwyg_ref, &JsValue::from_str("setControlValue")).unwrap();
            let set_control_value_method = set_control_value_method
                .dyn_ref::<js_sys::Function>()
                .unwrap();
            let _ = set_control_value_method.call2(&wysiwyg_ref, &key, &value);
        }
    }
}
