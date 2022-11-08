use crate::text_editor::font_css_converter::font_from_css;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::prelude::*;

use super::super::{state::TextEditor, wysiwyg_types::WysiwygControlsChange};

impl TextEditor {
    pub fn render_wysiwyg(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("wysiwyg-base", {
            .prop_signal("theme", state.theme_id.signal_cloned().map(|theme_id| {
                theme_id.as_str_id()
            }))
            .after_inserted(clone!(state => move |wysiwyg_ref| {
                state.set_wysiwyg_ref(wysiwyg_ref);
            }))
            .after_removed(clone!(state => move |_| {
                state.clear_wysiwyg_ref();
            }))
            .event(clone!(state => move |e: WysiwygControlsChange| {
                let value = e.value();
                // log::info!("{:?}", &value);
                let mut controls = state.controls.lock_mut();

                controls.font = font_from_css(&value.font);
                controls.element = value.element;
                controls.weight = value.weight;
                controls.align = value.align;
                controls.font_size = value.font_size;
                controls.color = value.color;
                controls.highlight_color = value.highlight_color;
                controls.box_color = value.box_color;
                controls.direction = value.direction;
                controls.italic = value.italic;
                controls.underline = value.underline;
            }))
            .event(clone!(state => move |e: events::CustomChange| {
                let value = e.value();
                if let Some(on_change) = &state.callbacks.on_change.as_ref() {
                    on_change(&value);
                }
            }))
            .event(clone!(state => move |_: events::CustomBlur| {
                if let Some(on_blur) = &state.callbacks.on_blur.as_ref() {
                    on_blur();
                }
            }))
        })
    }
}
