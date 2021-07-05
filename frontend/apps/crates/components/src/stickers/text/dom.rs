use dominator::{html, Dom, DomBuilder, clone};
use std::{borrow::BorrowMut, rc::Rc};
use utils::{prelude::*, math::transform_signals};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{self, always, Signal, ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::jig::module::body::{_groups::design::Text as RawText, Transform};
use crate::{
    transform::{
        dom::render_transform,
        events::Move as TransformMove,
        state::{TransformState, Action as TransformAction},
    },
    text_editor::dom::render_wysiwyg,
};
use super::{
    state::Text,
    super::state::Stickers,
    menu::dom::render_sticker_text_menu
};
use web_sys::HtmlElement;

const BASE_WIDTH:f64 = 300.0;
const BASE_HEIGHT:f64 = 30.0;


pub fn render_sticker_text(stickers:Rc<Stickers>, index: ReadOnlyMutable<Option<usize>>, text: Rc<Text>) -> Dom {
    text.transform.size.set(Some((BASE_WIDTH, BASE_HEIGHT)));

    let get_visible_signals = || map_ref! {
        let is_editing = text.is_editing.signal(),
        let is_active = stickers.selected_signal(index.clone())
            => {
                (*is_active, *is_editing)
            }
    };

    fn apply_transform<A: AsRef<HtmlElement>>(dom:DomBuilder<A>, transform: &TransformState) -> DomBuilder<A> {
        dom
                .style("position", "absolute")
                .style_signal("transform", transform.rotation_matrix_string_signal())
                .style_signal("top", transform.y_px_signal().map(|x| format!("{}px", x)))
                .style_signal("left", transform.x_px_signal().map(|x| format!("{}px", x)))
                .style_signal("width", transform.width_px_signal().map(|x| format!("{}px", x)))
                .style_signal("height", transform.height_px_signal().map(|x| format!("{}px", x)))
    }

    html!("empty-fragment", {
        //TODO - remove the nesting when upgrading dominator, just for multiple dynamic children
        .child(html!("empty-fragment", {
            .child_signal(get_visible_signals().map(clone!(stickers, text, index => move |(is_active, is_editing)| {
                if !is_active || !is_editing {
                    Some(html!("wysiwyg-output-renderer", {
                        .property_signal("valueAsString", text.value.signal_cloned())
                        .style("cursor", "pointer")
                        .apply(|dom| apply_transform(dom, &text.transform))
                        .event(clone!(index, stickers, text => move |evt:events::Click| {
                            if let Some(index) = index.get_cloned() {
                                let value = text.value.get_cloned();

                                text.editor.set_value(if value.is_empty() { None } else { Some(value) });
                                stickers.select_index(index);
                            }
                        }))
                    }))
                } else {
                    let value = text.value.get_cloned();
                    text.editor.set_value(if value.is_empty() { None } else { Some(value) });

                    Some(html!("div", {
                        .style("display", "block")
                        .style("border", "green dashed 1px")
                        .style("box-sizing", "border-box")
                        .style("align-self", "baseline")
                        .apply(|dom| apply_transform(dom, &text.transform))
                        .child(render_wysiwyg(text.editor.clone()))
                    }))
                }
            })))
        }))
        .child_signal(get_visible_signals().map(clone!(stickers, text, index => move |(is_active, is_editing)| {
            if is_active && !is_editing {
                Some(render_transform(
                    text.transform.clone(),
                    Some(clone!(stickers, index, text => move || render_sticker_text_menu(stickers.clone(), index.clone(), text.clone())))
                ))
            } else {
                None
            }
        })))

    })

}
pub fn render_sticker_text_raw(text: &RawText) -> Dom {
    const COORDS_IN_CENTER:bool = true;

    let size = Some((BASE_WIDTH, BASE_HEIGHT));

    let width_signal = transform_signals::width_px(
        COORDS_IN_CENTER, 
        always(text.transform.clone()), 
        always(size.clone())
    );
    let height_signal = transform_signals::height_px(
        COORDS_IN_CENTER, 
        always(text.transform.clone()), 
        always(size.clone())
    );
    let x_signal = transform_signals::x_px(
        COORDS_IN_CENTER, 
        always(text.transform.clone()), 
        always(size.clone())
    );
    let y_signal = transform_signals::y_px(
        COORDS_IN_CENTER, 
        always(text.transform.clone()), 
        always(size.clone())
    );

    html!("wysiwyg-output-renderer", {
        .property("valueAsString", &text.value)
        .style("position", "absolute")
        .style("transform", text.transform.rotation_matrix_string())
        .style_signal("width", width_signal.map(|x| format!("{}px", x)))
        .style_signal("height", height_signal.map(|x| format!("{}px", x)))
        .style_signal("left", x_signal.map(|x| format!("{}px", x)))
        .style_signal("top", y_signal.map(|x| format!("{}px", x)))
    })
}
