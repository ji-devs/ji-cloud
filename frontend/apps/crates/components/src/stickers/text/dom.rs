use dominator::{apply_methods, clone, html, with_node, Dom, DomBuilder};
use dominator_helpers::signals::{DefaultSignal, DomRectSignal};
use std::rc::Rc;
use utils::{math::transform_signals, prelude::*, resize::resize_info_signal};

use super::{
    super::{
        dom::{BaseRawRenderOptions, BaseRenderOptions},
        state::{AsSticker, Stickers},
    },
    menu::dom::render_sticker_text_menu,
    state::Text,
};
use crate::transform::{
    dom::render_transform,
    state::{ResizeLevel, TransformState},
};
use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable, SignalExt},
};
use shared::domain::module::body::_groups::design::Text as RawText;
use web_sys::HtmlElement;

#[derive(Default)]
pub struct TextRenderOptions {
    pub base: BaseRenderOptions,
}

#[derive(Default)]
pub struct TextRawRenderOptions {
    pub base: BaseRawRenderOptions,

    //For mixing in the measurer renderer
    pub measurer_mixin: Option<Box<dyn Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
}

impl TextRawRenderOptions {
    pub fn set_measurer_mixin(
        &mut self,
        f: impl Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static,
    ) {
        self.measurer_mixin = Some(Box::new(f) as _);
    }
}

pub fn render_sticker_text<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    index: ReadOnlyMutable<Option<usize>>,
    text: Rc<Text>,
    opts: Option<TextRenderOptions>,
) -> Dom {
    let _opts = opts.unwrap_or_default();

    let show_renderer_signal = map_ref! {
        let is_editing = text.is_editing.signal(),
        let is_active = stickers.selected_signal(index.clone())
            => {
                !*is_active || !*is_editing
            }
    };

    let sticker_menu_signal = || {
        map_ref! {
            let is_editing = text.is_editing.signal(),
            let is_active = stickers.selected_signal(index.clone()),
            let is_editable = text.is_editable.signal()
                => {
                    (*is_active && !*is_editing, *is_editable)
                }
        }
    };

    let highlight_signal = map_ref! {
        let sticker_menu = sticker_menu_signal(),
        let is_editing = text.is_editing.signal(),
        let highlight = text.highlight.signal()
            => {
                !sticker_menu.0 && *highlight && !is_editing
            }
    };

    fn apply_transform<A: AsRef<HtmlElement>>(
        dom: DomBuilder<A>,
        transform: &TransformState,
    ) -> DomBuilder<A> {
        dom.style("position", "absolute")
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
                .prop_signal("valueAsString", text.value.signal_cloned())
                .prop_signal("theme", text.editor.theme_id.signal_cloned().map(|theme_id| theme_id.as_str_id()))
                .apply(|dom| mixin_measured_text(dom, clone!(text => move |(width, height)| {
                    if width > 0.0 && height > 0.0 {
                        text.transform.size.set(Some((width, height)));
                    } else {
                        text.transform.size.set(None);
                    }
                })))
                .after_inserted(clone!(text => move |elem| {
                    text.measurer_ref.set(Some(elem));
                }))
                .after_removed(clone!(text => move |_elem| {
                    text.measurer_ref.set(None);
                }))
            })
        )
        .child_signal(show_renderer_signal.dedupe().map(clone!(stickers, text, index => move |show_renderer| {
            if show_renderer {
                //non-interactive rendering of wysiwyg text
                Some(html!("wysiwyg-output-renderer", {
                    .prop_signal("valueAsString", text.value.signal_cloned())
                    .prop_signal("theme", text.editor.theme_id.signal_cloned().map(|theme_id| theme_id.as_str_id()))
                    .style("cursor", "pointer")
                    .apply(|dom| apply_transform(dom, &text.transform))
                    .event(clone!(index, stickers, text => move |_evt:events::Click| {
                        if let Some(index) = index.get_cloned() {
                            let value = text.value.get_cloned();

                            text.editor.set_value(if value.is_empty() { None } else { Some(value) });
                            stickers.select_index(index);
                        }
                    }))
                    .after_inserted(clone!(text => move |elem| {
                        text.renderer_ref.set(Some(elem));
                    }))
                    .after_removed(clone!(text => move |_elem| {
                        text.renderer_ref.set(None);
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
                    .child(text.editor.render_wysiwyg())
                }))
            }
        })))
        .child_signal(sticker_menu_signal().map(clone!(stickers, text, index => move |(should_show, is_editable)| {
            if should_show {
                let menu = if is_editable {
                    Some(clone!(stickers, index, text => move || render_sticker_text_menu(stickers.clone(), index.clone(), text.clone())))
                } else {
                    None
                };
                Some(render_transform(
                    text.transform.clone(),
                    ResizeLevel::None,
                    menu,
                ))
            } else {
                None
            }
        })))
        .child_signal(highlight_signal.map(clone!(text => move |highlight| {
            if highlight {
                Some(html!("highlight-box", {
                    .style("display", "block")
                    .style("position", "absolute")
                    .style_signal("transform", text.transform.rotation_matrix_string_signal())
                    .style_signal("top", text.transform.y_px_signal().map(|x| format!("{}px", x)))
                    .style_signal("left", text.transform.x_px_signal().map(|x| format!("{}px", x)))
                    .style_signal("width", text.transform.width_px_signal().map(|x| format!("{}px", x)))
                    .style_signal("height", text.transform.height_px_signal().map(|x| format!("{}px", x)))
                    .prop_signal("width", text.transform.width_px_signal())
                    .prop_signal("height", text.transform.height_px_signal())
                    .prop_signal("screenScale", resize_info_signal().map(|resize| resize.scale))
                }))
            } else {
                None
            }
        }))
        )
    })
}

//The parent part is a bit weird, but helpful for creating generic containers like StickerOutline
//The idea is that the sticker sets styles on the parent and then appends itself
//So the parent gets transformed etc.
pub fn render_sticker_text_raw(
    text: &RawText,
    theme_id: ThemeId,
    opts: Option<TextRawRenderOptions>,
) -> Dom {
    const COORDS_IN_CENTER: bool = true;

    let opts = opts.unwrap_or_default();

    let parent = opts
        .base
        .parent
        .unwrap_or_else(|| DomBuilder::new_html("empty-fragment"));

    let size = opts.base.size.unwrap_or_else(|| Mutable::new(None));

    let transform = text.transform.clone();

    let transform_override = opts.base.transform_override;

    let get_transform_signal = clone!(transform, transform_override => move || {
        DefaultSignal::new(
            transform.clone(),
            transform_override.clone().map(clone!(transform => move |t| t.get_signal(transform)))
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

    let mixin = opts.base.mixin;
    let measurer_mixin = opts.measurer_mixin;

    html!("empty-fragment", {
        .child(
            //invisible element for measuring text size
            //required so that the transform will match
            html!("wysiwyg-output-renderer" => HtmlElement, {
                .prop("valueAsString", &text.value)
                .prop("theme", theme_id.as_str_id())
                .apply(|dom| mixin_measured_text(dom, clone!(size => move |(width, height)| {
                    if width > 0.0 && height > 0.0 {
                        size.set(Some((width, height)));
                    } else {
                        size.set(None);
                    }
                })))
                .apply_if(measurer_mixin.is_some(), move |dom| {
                    dom.apply(measurer_mixin.unwrap_ji())
                })
            })
        )
        .child(
            parent
                .style("user-select", "none")
                .style("position", "absolute")
                .style("transform", text.transform.rotation_matrix_string())
                //the text determines transform size, not the other way around
                //.style_signal("width", width_signal.map(|x| format!("{}px", x)))
                //.style_signal("height", height_signal.map(|x| format!("{}px", x)))
                .style_signal("left", x_signal.map(|x| format!("{}px", x)))
                .style_signal("top", y_signal.map(|x| format!("{}px", x)))
                .child(
                    html!("wysiwyg-output-renderer", {
                        .prop("valueAsString", &text.value)
                        .prop("theme", theme_id.as_str_id())
                        // Prevent text from being selected if a student attempts to drag
                        // a non-interactive text sticker.
                        .style("user-select", "none")
                        .apply_if(mixin.is_some(), move |dom| {
                            dom.apply(mixin.unwrap_ji())
                        })
                    })
            ).into_dom()
        )
    })
}

fn mixin_measured_text(
    dom: DomBuilder<HtmlElement>,
    mut on_size: impl FnMut((f64, f64)) + 'static,
) -> DomBuilder<HtmlElement> {
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
