use std::rc::Rc;
use std::cell::RefCell;

use futures_signals::signal::Mutable;
use utils::themes::{ThemeId, ThemeIdExt};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;
use js_sys::Reflect;
use strum::IntoEnumIterator;

use super::{
    font_css_converter::font_to_css,
    theme_element_styles::get_theme_element_styles, 
    wysiwyg_types::{ControlsState, ControlsChange, Align, Weight, Font, ElementType, enum_variant_to_string, BOLD_WEIGHT, REGULAR_WEIGHT}
};
use super::super::font_loader::{FontLoader, Font as StaticFont};
use super::components::text_editor_controls::color_controls::ColorState;

pub struct State {
    pub controls: Mutable<ControlsState>,
    pub wysiwyg_ref: Rc<RefCell<Option<HtmlElement>>>,
    pub fonts: Mutable<Vec<String>>,
    pub on_new_text: RefCell<Option<Box<dyn Fn(&str)>>>,
    pub on_change: RefCell<Option<Box<dyn Fn(&str)>>>,
    pub on_blur: RefCell<Option<Box<dyn Fn()>>>,
    pub value: RefCell<Option<String>>,
    pub theme_id: Mutable<ThemeId>,
    pub color_state: RefCell<Option<Rc<ColorState>>>,

}

impl State {
    pub fn new(theme_id: ThemeId, value: Option<String>, on_new_text: Option<impl Fn(&str) + 'static>, on_change: Option<impl Fn(&str) + 'static>, on_blur: Option<impl Fn() + 'static>, ) -> Rc<Self> {
        let _self = Rc::new(Self {
            controls: Mutable::new(ControlsState::new()),
            wysiwyg_ref: Rc::new(RefCell::new(None)),
            fonts: Mutable::new(Self::get_fonts(theme_id)),
            on_new_text: RefCell::new(on_new_text.map(|f| Box::new(f) as _)),
            on_change: RefCell::new(on_change.map(|f| Box::new(f) as _)),
            on_blur: RefCell::new(on_blur.map(|f| Box::new(f) as _)),
            value: RefCell::new(value),
            theme_id: Mutable::new(theme_id),
            color_state: RefCell::new(None) 
        });

        *_self.color_state.borrow_mut() = Some(Rc::new(ColorState::new(_self.clone())));

        _self
    }

    pub fn set_value(&self, value: Option<String>) {
        self.value.replace(value);
        if let Some(wysiwyg_ref) = &*self.wysiwyg_ref.borrow() {
            self.update_wysiwyg_value(&wysiwyg_ref);
        }
    }

    pub fn set_theme(&self, theme_id: ThemeId) {
        self.theme_id.set_neq(theme_id.clone());
        self.fonts.set(Self::get_fonts(theme_id));
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

    fn get_fonts(theme: ThemeId) -> Vec<String> {
        // load all fonts in background
        spawn_local(async {
            FontLoader::new().load_all().await;
        });

        let mut fonts: Vec<String> = Vec::from(theme.get_fonts());
        let mut static_fonts: Vec<String> = StaticFont::iter().map(|font| {
            String::from(font.get_font_name())
        }).collect();
        fonts.append(&mut static_fonts);
        fonts
    }

    pub(super) fn set_wysiwyg_ref(&self, wysiwyg_ref: HtmlElement) {
        let key = enum_variant_to_string(&ControlsChange::Element(ElementType::P1)) + &String::from("Default");
        let _ = Reflect::set(
            &wysiwyg_ref,
            &JsValue::from_str(&key),
            &JsValue::from_str(&ElementType::P1.to_string())
        );

        let (font, color, font_size) = get_theme_element_styles(&self.theme_id.lock_ref(), &ElementType::P1);

        let key = enum_variant_to_string(&ControlsChange::FontSize(0)) + &String::from("Default");
        let _ = Reflect::set(
            &wysiwyg_ref,
            &JsValue::from_str(&key),
            &JsValue::from_f64(font_size as f64)
        );
        let key = enum_variant_to_string(&ControlsChange::Font(String::new())) + &String::from("Default");
        let _ = Reflect::set(
            &wysiwyg_ref,
            &JsValue::from_str(&key),
            &JsValue::from_str(&font_to_css(&font))
        );
        let key = enum_variant_to_string(&ControlsChange::Color(None)) + &String::from("Default");
        let _ = Reflect::set(
            &wysiwyg_ref,
            &JsValue::from_str(&key),
            &JsValue::from_str(&color)
        );

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
    pub(super) fn set_align(&self, align: Align) {
        let mut controls = self.controls.lock_mut();
        controls.align = align;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            let _ = Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Align(Align::Left))),
                &JsValue::from_str(&controls.align.to_string())
            );
        }
    }
    pub(super) fn set_font_size(&self, font_size: u8) {
        let mut controls = self.controls.lock_mut();
        controls.font_size = font_size;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            // &JsValue::from_f64 might be replace with something that converts u8 directly 
            let _ = Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::FontSize(0))),
                &JsValue::from_f64(controls.font_size as f64)
            );
        }
    }
    pub(super) fn set_indent_count(&self, indent_count: u8) {
        let mut controls = self.controls.lock_mut();
        controls.indent_count = indent_count;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            let _ = Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::IndentCount(0))),
                &JsValue::from_f64(controls.indent_count as f64)
            );
        }
    }
    pub(super) fn set_color(&self, color: Option<String>) {
        let mut controls = self.controls.lock_mut();
        controls.color = color;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            let js_value = match &controls.color {
                Some(color) => JsValue::from_str(&color),
                None => JsValue::UNDEFINED,
            };
            let _ = Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Color(None))),
                &js_value
            );
        }
    }
    pub(super) fn set_highlight_color(&self, highlight_color: Option<String>) {
        let mut controls = self.controls.lock_mut();
        controls.highlight_color = highlight_color;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            let js_value = match &controls.highlight_color {
                Some(color) => JsValue::from_str(&color),
                None => JsValue::UNDEFINED,
            };
            let _ = Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::HighlightColor(None))),
                &js_value
            );
        }
    }
    pub(super) fn set_font(&self, font: Font) {
        let mut controls = self.controls.lock_mut();
        controls.font = font;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            let _ = Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Font(String::new()))),
                &JsValue::from_str(&font_to_css(&controls.font.to_string()))
            );
        }
    }
    pub(super) fn set_element(&self, element: ElementType) {
        let mut controls = self.controls.lock_mut();
        controls.element = element;

        let element_styles = get_theme_element_styles(&self.theme_id.lock_ref(), &controls.element);
        controls.font = element_styles.0;
        controls.color = Some(element_styles.1);
        controls.font_size = element_styles.2;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            let _ = Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Element(ElementType::P1))),
                &JsValue::from_str(&controls.element.to_string())
            );
            let js_color = match &controls.color {
                Some(color) => JsValue::from_str(&color),
                None => JsValue::UNDEFINED,
            };
            let _ = Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Color(None))),
                &js_color
            );
            let _ = Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::FontSize(0))),
                &JsValue::from_f64(controls.font_size as f64)
            );
            let _ = Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Font(String::new()))),
                &JsValue::from_str(&font_to_css(&controls.font.to_string()))
            );
        }
    }
    pub(super) fn set_weight(&self, weight: Weight) {
        let mut controls = self.controls.lock_mut();
        controls.weight = weight;

        if let Some(wysiwyg_ref) = &self.wysiwyg_ref.borrow().as_ref() {
            let _ = Reflect::set(
                wysiwyg_ref,
                &JsValue::from_str(&enum_variant_to_string(&ControlsChange::Weight(0))),
                &JsValue::from_f64(controls.weight as f64)
            );
        }
    }
}
