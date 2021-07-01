use std::future::ready;
use std::rc::Rc;
use std::cell::RefCell;

use dominator::clone;
use futures_signals::signal::{Mutable, ReadOnlyMutable, SignalExt};
use utils::themes::{ThemeId, ThemeIdExt};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;
use js_sys::Reflect;
use strum::IntoEnumIterator;

use super::{
    font_css_converter::font_to_css,
    wysiwyg_types::{ControlsState, ControlsChange, Align, Weight, Font, ElementType, enum_variant_to_string, BOLD_WEIGHT, REGULAR_WEIGHT}
};
use super::super::font_loader::{FontLoader, Font as StaticFont};
use super::components::text_editor_controls::color_controls::ColorState;
use super::callbacks::Callbacks;

pub struct State {
    pub controls: Mutable<ControlsState>,
    pub wysiwyg_ref: Rc<RefCell<Option<HtmlElement>>>,
    pub fonts: Mutable<Vec<String>>,
    pub value: RefCell<Option<String>>,
    pub theme_id: ReadOnlyMutable<ThemeId>,
    pub color_state: RefCell<Option<Rc<ColorState>>>,
    pub callbacks: Callbacks,
}


impl State {
    pub fn new(theme_id: ReadOnlyMutable<ThemeId>, value: Option<String>, callbacks: Callbacks) -> Rc<Self> {
        let _self = Rc::new(Self {
            controls: Mutable::new(ControlsState::new()),
            wysiwyg_ref: Rc::new(RefCell::new(None)),
            fonts: Mutable::new(vec![]),
            callbacks,
            value: RefCell::new(value),
            theme_id,
            color_state: RefCell::new(None) 
        });

        *_self.color_state.borrow_mut() = Some(Rc::new(ColorState::new(_self.clone())));

        Self::handle_fonts(Rc::clone(&_self));

        _self
    }

    pub fn set_value(&self, value: Option<String>) {
        self.value.replace(value);
        if let Some(wysiwyg_ref) = &*self.wysiwyg_ref.borrow() {
            self.update_wysiwyg_value(&wysiwyg_ref);
        }
    }

    pub fn select_all(&self) {
        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            let select_all_method = Reflect::get(
                &wysiwyg_ref,
                &JsValue::from_str("selectAll")
            )
                .unwrap();
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
                    &JsValue::from_str(&value)
                );
            },
            None => {
                let reset_value_method = Reflect::get(
                    &wysiwyg_ref,
                    &JsValue::from_str("resetValue")
                ).unwrap();
                let reset_value_method = reset_value_method.dyn_ref::<js_sys::Function>().unwrap();
                let _ = reset_value_method.call0(&wysiwyg_ref);
            },
        };
    }

    fn handle_fonts(state: Rc<State>) {
        // load all fonts in background
        spawn_local(async {
            FontLoader::new().load_all().await;
        });

        spawn_local(state.theme_id.signal_cloned().for_each(clone!(state => move |theme| {
            let mut fonts: Vec<String> = Vec::from(theme.get_fonts());
            let mut static_fonts: Vec<String> = StaticFont::iter().map(|font| {
                String::from(font.get_font_name())
            }).collect();
            fonts.append(&mut static_fonts);
            state.fonts.set(fonts);
            ready(())
        })));
    }

    pub(super) fn set_wysiwyg_ref(&self, wysiwyg_ref: HtmlElement) {
        let key = enum_variant_to_string(&ControlsChange::Element(ElementType::P1)) + &String::from("Default");
        let _ = Reflect::set(
            &wysiwyg_ref,
            &JsValue::from_str(&key),
            &JsValue::from_str(&ElementType::P1.to_string())
        );

        // let (font, color, font_size) = get_theme_element_styles(&self.theme_id.lock_ref(), &ElementType::P1);

        // let key = enum_variant_to_string(&ControlsChange::FontSize(0)) + &String::from("Default");
        // let _ = Reflect::set(
        //     &wysiwyg_ref,
        //     &JsValue::from_str(&key),
        //     &JsValue::from_f64(font_size as f64)
        // );
        // let key = enum_variant_to_string(&ControlsChange::Font(String::new())) + &String::from("Default");
        // let _ = Reflect::set(
        //     &wysiwyg_ref,
        //     &JsValue::from_str(&key),
        //     &JsValue::from_str(&font_to_css(&font))
        // );
        // let key = enum_variant_to_string(&ControlsChange::Color(None)) + &String::from("Default");
        // let _ = Reflect::set(
        //     &wysiwyg_ref,
        //     &JsValue::from_str(&key),
        //     &JsValue::from_str(&color)
        // );

        self.update_wysiwyg_value(&wysiwyg_ref);

        *self.wysiwyg_ref.borrow_mut() = Some(wysiwyg_ref);
    }

    // Suggestion: maybe replace all there functions with one that just takes a controls change.
    // Suggestion: might be a good idea to remove all this and just have the event listener trigger the update to the element and have the change propagate up.
    pub(super) fn toggle_bold(&self) {
        let mut controls = self.controls.lock_mut();
        controls.weight = if controls.weight == BOLD_WEIGHT {
            REGULAR_WEIGHT
        } else {
            BOLD_WEIGHT
        };

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            let _ = Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Weight(0))),
                &JsValue::from_f64(controls.weight as f64)
            );
        }
    }
    pub(super) fn toggle_italic(&self) {
        let mut controls = self.controls.lock_mut();
        controls.italic = !controls.italic;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            let _ = Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Italic(false))),
                &JsValue::from_bool(controls.italic));
        }
   
    }
    pub(super) fn toggle_underline(&self) {
        let mut controls = self.controls.lock_mut();
        controls.underline = !controls.underline;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            let _ = Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Underline(false))),
                &JsValue::from_bool(controls.underline));
        }
   
    }

    pub(super) fn set_control_value(&self, control: ControlsChange) {
        self.set_control_value_rust(&control);
        self.set_control_value_js(&control);
    }
    fn set_control_value_rust(&self, control: &ControlsChange) {
        let mut controls = self.controls.lock_mut();
        match control {
            ControlsChange::Font(font) => controls.font = font.clone(),
            ControlsChange::Element(element) => controls.element = element.clone(),
            ControlsChange::Weight(weight) => controls.weight = *weight,
            ControlsChange::Align(align) => controls.align = align.clone(),
            ControlsChange::FontSize(font_size) => controls.font_size = *font_size,
            ControlsChange::Color(color) => controls.color = color.clone(),
            ControlsChange::HighlightColor(highlight_color) => controls.highlight_color = highlight_color.clone(),
            ControlsChange::IndentCount(indent_count) => controls.indent_count = *indent_count,
            ControlsChange::Italic(italic) => controls.italic = *italic,
            ControlsChange::Underline(underline) => controls.underline = *underline,
        };
    }
    fn set_control_value_js(&self, control: &ControlsChange) {
        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            let (key, value) = control.to_js_key_value();
            let _ = Reflect::set(
                wysiwyg_ref,
                &key,
                &value
            );
        }
    }
}
