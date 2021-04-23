use std::rc::Rc;
use std::cell::RefCell;

use futures_signals::signal::Mutable;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use js_sys::Reflect;
use rgb::RGBA8;

use super::wysiwyg_types::{ControlsState, ControlsChange, Align, Weight, Font, ElementType, enum_variant_to_string};
use super::super::color_select::actions::{hex_to_rgba8, rgba8_to_hex};

pub struct State {
    pub controls: Mutable<ControlsState>,
    pub wysiwyg_ref: Rc<RefCell<Option<HtmlElement>>>,
    pub on_change: Option<fn(s: &str)>,
}

impl State {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            controls: Mutable::new(ControlsState::new()),
            wysiwyg_ref: Rc::new(RefCell::new(None)),
            on_change: None,
        })
    }

    pub fn new_with_on_change(func: fn(s: &str)) -> Rc<Self> {
        Rc::new(Self {
            controls: Mutable::new(ControlsState::new()),
            wysiwyg_ref: Rc::new(RefCell::new(None)),
            on_change: Some(func),
        })
    }

    pub fn set_wysiwyg_ref(&self, wysiwyg_ref: HtmlElement) {
        *self.wysiwyg_ref.borrow_mut() = Some(wysiwyg_ref);
    }

    // Suggestion: maybe replace all there functions with one that just takes a controls change.
    // Suggestion: might be a good idea to remove all this and just have the event listener trigger the update to the element and have the change propagate up.
    pub fn toggle_bold(&self) {
        let mut controls = self.controls.lock_mut();
        controls.bold = !controls.bold;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Bold(false))),
                &JsValue::from_bool(controls.bold)
            );
        }
    }
    pub fn toggle_italic(&self) {
        let mut controls = self.controls.lock_mut();
        controls.italic = !controls.italic;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Italic(false))),
                &JsValue::from_bool(controls.italic));
        }
   
    }
    pub fn toggle_underline(&self) {
        let mut controls = self.controls.lock_mut();
        controls.underline = !controls.underline;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Underline(false))),
                &JsValue::from_bool(controls.underline));
        }
   
    }
    pub fn set_align(&self, align: Align) {
        let mut controls = self.controls.lock_mut();
        controls.align = align;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Align(Align::Left))),
                &JsValue::from_str(&controls.align.to_string())
            );
        }
    }
    pub fn set_font_size(&self, font_size: u8) {
        let mut controls = self.controls.lock_mut();
        controls.font_size = font_size;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            // &JsValue::from_f64 might be replace with something that converts u8 directly 
            Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::FontSize(0))),
                &JsValue::from_f64(controls.font_size as f64)
            );
        }
    }
    pub fn set_indent_count(&self, indent_count: u8) {
        let mut controls = self.controls.lock_mut();
        controls.indent_count = indent_count;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::IndentCount(0))),
                &JsValue::from_f64(controls.indent_count as f64)
            );
        }
    }
    pub fn set_color(&self, color: Option<RGBA8>) {
        let mut controls = self.controls.lock_mut();
        controls.color = color;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            let js_value = match controls.color {
                Some(color) => JsValue::from_str(&rgba8_to_hex(&color)),
                None => JsValue::UNDEFINED,
            };
            Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Color(None))),
                &js_value
            );
        }
    }
    pub fn set_highlight_color
    (&self, highlight_color: Option<RGBA8>) {
        let mut controls = self.controls.lock_mut();
        controls.highlight_color = highlight_color;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            let js_value = match controls.color {
                Some(color) => JsValue::from_str(&rgba8_to_hex(&color)),
                None => JsValue::UNDEFINED,
            };
            Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::HighlightColor(None))),
                &js_value
            );
        }
    }
    pub fn set_font
    (&self, font: Font) {
        let mut controls = self.controls.lock_mut();
        controls.font = font;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Font(Font::Arial))),
                &JsValue::from_str(&controls.font.to_string())
                // &JsValue::from_str(&enum_variant_to_string(&controls.font))
            );
        }
    }
    pub fn set_element(&self, element: ElementType) {
        let mut controls = self.controls.lock_mut();
        controls.element = element;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Element(ElementType::P1))),
                &JsValue::from_str(&controls.element.to_string())
                // &JsValue::from_str(&enum_variant_to_string(&controls.element))
            );
        }
    }
    pub fn set_weight(&self, weight: Weight) {
        let mut controls = self.controls.lock_mut();
        controls.weight = weight;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Weight(Weight::Normal))),
                &JsValue::from_str(&controls.weight.to_string())
                // &JsValue::from_str(&enum_variant_to_string(&controls.weight))
            );
        }
    }
}
