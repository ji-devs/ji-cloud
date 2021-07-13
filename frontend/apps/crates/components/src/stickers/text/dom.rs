use dominator::{html, Dom, DomBuilder, clone, with_node, apply_methods};
use dominator_helpers::signals::{DefaultSignal, DomRectSignal};
use std::{borrow::BorrowMut, rc::Rc};
use utils::{math::{BoundsF64, transform_signals}, prelude::*, resize::resize_info_signal};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{self, always, Always, Signal, Mutable, ReadOnlyMutable, SignalExt},
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
    super::{dom::OffsetMutable, state::{Stickers, AsSticker}},
    menu::dom::render_sticker_text_menu
};
use web_sys::{DomRect, HtmlElement};


pub fn render_sticker_text<T: AsSticker>(stickers:Rc<Stickers<T>>, index: ReadOnlyMutable<Option<usize>>, text: Rc<Text>) -> Dom {
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
                //the text determines transform size, not the other way around
                //.style_signal("width", transform.width_px_signal().map(|x| format!("{}px", x)))
                //.style_signal("height", transform.height_px_signal().map(|x| format!("{}px", x)))
    }

    html!("empty-fragment", {
        .child(
            //invisible element for measuring text size
            //required so that the transform will match
            html!("wysiwyg-output-renderer" => HtmlElement, {
                .property_signal("valueAsString", text.value.signal_cloned())
                .property_signal("theme", text.editor.theme_id.signal_cloned().map(|theme_id| theme_id.as_str_id()))
                .apply(|dom| mixin_measured_text(dom, clone!(text => move |(width, height)| {
                    if width > 0.0 && height > 0.0 {
                        text.transform.size.set(Some((width, height)));
                    } else {
                        text.transform.size.set(None);
                    }
                })))
            })
        )
        .child_signal(get_visible_signals().map(clone!(stickers, text, index => move |(is_active, is_editing)| {
            if !is_active || !is_editing {
                //non-interactive rendering of wysiwyg text
                Some(html!("wysiwyg-output-renderer", {
                    .property_signal("valueAsString", text.value.signal_cloned())
                    .property_signal("theme", text.editor.theme_id.signal_cloned().map(|theme_id| theme_id.as_str_id()))
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
                // the full wysiwyg editor with green outline
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
        .child_signal(get_visible_signals().map(clone!(stickers, text, index => move |(is_active, is_editing)| {
            if is_active && !is_editing {
                Some(render_transform(
                    text.transform.clone(),
                    false,
                    Some(clone!(stickers, index, text => move || render_sticker_text_menu(stickers.clone(), index.clone(), text.clone())))
                ))
            } else {
                None
            }
        })))
    })
}


pub fn render_sticker_text_raw(text: &RawText, theme_id: ThemeId) -> Dom {
    _render_sticker_text_raw_mixin(text,theme_id, None::<fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>)
}

pub fn render_sticker_text_raw_mixin<F>(text: &RawText, theme_id: ThemeId, mixin: F) -> Dom 
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>
{
    _render_sticker_text_raw_mixin(text, theme_id, Some(mixin))
}

fn _render_sticker_text_raw_mixin<F>(text: &RawText, theme_id: ThemeId, mixin: Option<F>) -> Dom 
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>
{

    _render_sticker_text_raw_offset_parent_mixin(DomBuilder::new_html("empty-fragment"), text, theme_id, None, mixin)

}
pub fn render_sticker_text_raw_offset(text: &RawText, theme_id: ThemeId, offset: OffsetMutable) -> Dom
{
    let parent = DomBuilder::new_html("empty-fragment");
    _render_sticker_text_raw_offset_parent_mixin(parent, text, theme_id, Some(offset), None::<fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>) 
}

pub fn render_sticker_text_raw_offset_mixin<F>(text: &RawText, theme_id: ThemeId, offset: OffsetMutable, mixin: F) -> Dom
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>
{
    let parent = DomBuilder::new_html("empty-fragment");
    _render_sticker_text_raw_offset_parent_mixin(parent, text, theme_id, Some(offset), Some(mixin)) 
}

pub fn render_sticker_text_raw_parent(parent: DomBuilder<HtmlElement>, theme_id: ThemeId, text: &RawText) -> Dom
{
    _render_sticker_text_raw_offset_parent_mixin(parent, text, theme_id, None, None::<fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>) 
}

pub fn render_sticker_text_raw_parent_mixin<F>( parent: DomBuilder<HtmlElement>, text: &RawText, theme_id: ThemeId, child_mixin: F) -> Dom 
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>
{
    _render_sticker_text_raw_offset_parent_mixin(parent, text, theme_id, None, Some(child_mixin))
}

pub fn render_sticker_text_raw_offset_parent_mixin<F>( parent: DomBuilder<HtmlElement>, text: &RawText, theme_id: ThemeId, offset: OffsetMutable, child_mixin: F) -> Dom 
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>
{
    _render_sticker_text_raw_offset_parent_mixin(parent, text, theme_id, Some(offset), Some(child_mixin))
}

//The parent part is a bit weird, but helpful for creating generic containers like StickerOutline
//The idea is that the sticker sets styles on the parent and then appends itself
//So the parent gets transformed etc.
fn _render_sticker_text_raw_offset_parent_mixin<F>(parent: DomBuilder<HtmlElement>, text: &RawText, theme_id: ThemeId, offset: Option<OffsetMutable>, child_mixin: Option<F>) -> Dom
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>
{
    const COORDS_IN_CENTER:bool = true;

    let size = Mutable::new(None);

    let transform = text.transform.clone();

    let get_transform_signal = clone!(offset, transform => move || {
        DefaultSignal::new(
            transform.clone(),
            offset.clone().map(clone!(transform => move |offset| {
                transform_signals::map_offset(transform, offset.signal_cloned())
            }))
        )
    });

    let x_signal = transform_signals::x_px(
        COORDS_IN_CENTER, 
        get_transform_signal(), 
        size.signal_cloned(), 
    );
    let y_signal = transform_signals::y_px(
        COORDS_IN_CENTER, 
        get_transform_signal(), 
        size.signal_cloned(), 
    );

    html!("empty-fragment", {
        .child(
            //invisible element for measuring text size
            //required so that the transform will match
            html!("wysiwyg-output-renderer" => HtmlElement, {
                .property("valueAsString", &text.value)
                .property("theme", theme_id.as_str_id())
                .apply(|dom| mixin_measured_text(dom, clone!(size => move |(width, height)| {
                    if width > 0.0 && height > 0.0 {
                        size.set(Some((width, height)));
                    } else {
                        size.set(None);
                    }
                })))
            })
        )
        .child(
            parent
                .style("position", "absolute")
                .style("transform", text.transform.rotation_matrix_string())
                //the text determines transform size, not the other way around
                //.style_signal("width", width_signal.map(|x| format!("{}px", x)))
                //.style_signal("height", height_signal.map(|x| format!("{}px", x)))
                .style_signal("left", x_signal.map(|x| format!("{}px", x)))
                .style_signal("top", y_signal.map(|x| format!("{}px", x)))
                .child(
                    html!("wysiwyg-output-renderer", {
                        .property("valueAsString", &text.value)
                        .property("theme", theme_id.as_str_id())
                        .apply_if(child_mixin.is_some(), move |dom| {
                            dom.apply(child_mixin.unwrap_ji())
                        })
                    })
            ).into_dom()
        )
    })
}

fn mixin_measured_text(dom:DomBuilder<HtmlElement>, mut on_size: impl FnMut((f64, f64)) + 'static) -> DomBuilder<HtmlElement> {
    apply_methods!(dom, {
        .with_node!(elem => {
            .future(
                map_ref! {
                    let dom_rect = DomRectSignal::new(&elem),
                    let resize_info = resize_info_signal()
                        => {
                            resize_info.get_size_rem(dom_rect.width(), dom_rect.height())
                        }
                }
                .for_each(move |size| {
                    on_size(size);
                    async {}
                })
            )
        })
        .style("position", "fixed")
        .style("visibility", "hidden")
        .style("pointer-events", "none")
    })
}
