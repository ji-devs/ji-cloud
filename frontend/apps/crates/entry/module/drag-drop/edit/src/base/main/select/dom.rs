use components::module::_common::edit::prelude::*;
use dominator::{html, Dom, DomBuilder, clone, apply_methods};
use web_sys::HtmlElement;
use std::rc::Rc;
use crate::base::state::*;
use super::state::*;
use utils::prelude::*;
use components::{
    buttons::{Button, ButtonStyle, ButtonStyleIcon},
    box_outline::{BoxOutline,BoxOutlineMixins},
    backgrounds::dom::render_backgrounds, 
    stickers::dom::{render_stickers, mixin_sticker_button, StickerRawRenderOptions, BaseRawRenderOptions, render_sticker_raw},
};

use futures_signals::{
    signal_vec::{always, SignalVecExt},
    signal::{Mutable, SignalExt}
};

impl MainSelect {
    pub fn render(state: Rc<Self>) -> Dom {
        let theme_id = state.base.theme_id.get();
        
        html!("empty-fragment", {
            .children_signal_vec(
                always(
                    state.clone().items
                        .iter()
                        .map(|item| {
                            item.kind_signal_cloned()
                                .map(clone!(state, item => move |kind| {
                                    match kind {
                                        ItemKind::Interactive(data) => {
                                            Self::render_interactive(state.clone(), theme_id, item.clone(), data)
                                        },
                                        ItemKind::Static => {
                                            Self::render_static(state.clone(), theme_id, item.clone())
                                        }
                                    }
                                }))
                        })
                        .collect()
                )
                .map_signal(|x| x)
            )
        })
    }

    pub fn render_interactive(state: Rc<Self>, theme_id: ThemeId, item: SelectItem, data: Interactive) -> Dom {
        let mut opts = BaseRawRenderOptions::default();
        let size:Mutable<Option<(f64, f64)>> = Mutable::new(None);
        let index = item.index;

        opts.set_transform_override(item.get_transform_override());

        opts.set_mixin(clone!(state, index, item => move |dom| {
            dom
                .apply(Self::mixin_click(state.clone(), index))
        }));

        opts.set_size(size.clone());

        let raw_sticker = &item.raw_sticker();

        let opts = StickerRawRenderOptions::new(&raw_sticker, Some(opts));

        let transform = raw_sticker.transform().clone();
        let transform_override = item.get_transform_override();
        html!("empty-fragment", {
            .child(render_sticker_raw(&raw_sticker, theme_id, Some(opts)))
            .child(BoxOutline::render_mixins(
                    {
                        let mut box_outline = BoxOutline::new_transform_size(
                            move || transform_override.get_signal(transform.clone()),
                            move || size.signal_cloned()
                        );

                        box_outline.set_top_right_hover_only(true);

                        box_outline
                    },
                    None,
                    BoxOutlineMixins {
                        click_area: Some(clone!(state, index, item => move |dom:DomBuilder<HtmlElement>| {
                            dom
                                .event(clone!(item => move |evt:events::MouseDown| {
                                    item.start_drag(evt.x() as i32, evt.y() as i32);
                                }))
                                .global_event_preventable(clone!(item => move |evt:events::MouseUp| {
                                    item.try_end_drag(evt.x() as i32, evt.y() as i32);

                                }))
                                .global_event_preventable(clone!(item => move |evt:events::MouseMove| {
                                    item.try_move_drag(evt.x() as i32, evt.y() as i32);
                                }))
                        })),

                        //adds a close button to the top-right corner 
                        top_right: Some(clone!(state, index => move |dom:DomBuilder<HtmlElement>| {
                            dom.child(Button::render(
                                Button::new(
                                    ButtonStyle::Icon(ButtonStyleIcon::BlueX),
                                    clone!(state, index => move || { 
                                        state.base.set_drag_item_deselected(index)
                                    })
                                ),
                                None
                            ))
                        }))
                    }
            ))
        })

    }

    pub fn render_static(state: Rc<Self>, theme_id: ThemeId, item: SelectItem) -> Dom {
        let mut opts = BaseRawRenderOptions::default();
        opts.set_mixin(Self::mixin_click(state.clone(), item.index));

        let raw_sticker = &item.raw_sticker();

        let opts = StickerRawRenderOptions::new(&raw_sticker, Some(opts));

        render_sticker_raw(&raw_sticker, theme_id, Some(opts))
    }

    pub fn mixin_click(state:Rc<Self>, index: usize) -> impl Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        move |dom| {
            dom
                .apply(mixin_sticker_button)
                .event(clone!(state, index => move |evt:events::Click| {
                    state.base.set_drag_item_selected(index)
                }))
        }
    }
}
