use dominator::{clone, html, Dom};
use shared::domain::module::body::{HoverAnimation, ShowHideAnimation, StickerHidden};
use std::rc::Rc;
use strum_macros::EnumIs;
use utils::prelude::*;

use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable, SignalExt},
};

use crate::stickers::text::dom::TextRenderOptions;

use super::{
    super::super::state::{AsSticker, Stickers},
    super::state::Text,
};

#[derive(Clone, Copy, EnumIs)]
enum MenuPhase {
    Main,
    Animations,
    HoverAnimation,
    OnClick,
    UntilClick,
}

pub fn render_sticker_text_menu<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    index: ReadOnlyMutable<Option<usize>>,
    text: Rc<Text>,
    opts: Rc<TextRenderOptions>,
) -> Dom {
    let menu_phase = Mutable::new(MenuPhase::Main);
    html!("div", {
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("gap", "10px")
        .style("align-items", "start")
        .children_signal_vec(menu_phase.signal().map(clone!(stickers, index, text, menu_phase => move |phase| {
            match phase {
                MenuPhase::Main => render_main(Rc::clone(&stickers), index.clone(), Rc::clone(&text), menu_phase.clone(), opts.clone()),
                MenuPhase::Animations => render_animations(Rc::clone(&stickers), Rc::clone(&text), menu_phase.clone()),
                MenuPhase::HoverAnimation => render_hover_animations(Rc::clone(&stickers), Rc::clone(&text), menu_phase.clone()),
                MenuPhase::OnClick => render_hide_on_click(Rc::clone(&stickers), Rc::clone(&text), menu_phase.clone()),
                MenuPhase::UntilClick => render_hide_until_click(Rc::clone(&stickers), Rc::clone(&text), menu_phase.clone()),
            }
        })).to_signal_vec())
        .child_signal(map_ref! {
            let sticker_index = index.signal(),
            let menu_phase = menu_phase.signal() => move {
                if menu_phase.is_main() {
                    let can_delete = match sticker_index {
                        Some(sticker_index) => match stickers.get_as_text(*sticker_index) {
                            Some(sticker) => sticker.can_delete.get(),
                            None => true,
                        }
                        None => true,
                    };

                    if can_delete {
                        Some(html!("menu-line", {
                            .prop("icon", "delete")
                            .event(clone!(stickers, index, text => move |_evt:events::Click| {
                                text.transform.close_menu();
                                if let Some(index) = index.get() {
                                    stickers.delete_index(index);
                                }
                            }))
                        }))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        })
    })
}

fn render_main<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    index: ReadOnlyMutable<Option<usize>>,
    text: Rc<Text>,
    menu_phase: Mutable<MenuPhase>,
    opts: Rc<TextRenderOptions>,
) -> Vec<Dom> {
    vec![
        html!("menu-line", {
            .prop("icon", "edit")
            .event(clone!(stickers, index, text => move |_evt:events::Click| {
                text.transform.close_menu();
                if let Some(index) = index.get() {
                    if let Some(text_sticker) = stickers.get_as_text(index) {
                        text_sticker.is_editing.set_neq(true);
                    }
                }
            }))
        }),
        html!("menu-line", {
            .prop("icon", "duplicate")
            .event(clone!(stickers, index, text => move |_evt:events::Click| {
                text.transform.close_menu();
                if let Some(index) = index.get() {
                    stickers.duplicate(index);
                }
            }))
        }),
        html!("menu-line", {
            .prop("icon", "move-to-front")
            .event(clone!(stickers, index, text => move |_ :events::Click| {
                text.transform.close_menu();
                if let Some(index) = index.get() {
                    stickers.move_to_front(index);
                }
            }))
        }),
        html!("menu-line", {
            .prop("icon", "move-forward")
            .event(clone!(stickers, index, text => move |_evt:events::Click| {
                text.transform.close_menu();
                if let Some(index) = index.get() {
                    stickers.move_forward(index);
                }
            }))
        }),
        html!("menu-line", {
            .prop("icon", "move-backward")
            .event(clone!(stickers, index, text => move |_evt:events::Click| {
                text.transform.close_menu();
                if let Some(index) = index.get() {
                    stickers.move_backward(index);
                }
            }))
        }),
        html!("menu-line", {
            .prop("icon", "move-to-back")
            .event(clone!(stickers, index, text => move |_ :events::Click| {
                text.transform.close_menu();
                if let Some(index) = index.get() {
                    stickers.move_to_back(index);
                }
            }))
        }),
        html!("menu-line", {
            .visible(opts.base.animations)
            .prop("icon", "animations")
            .event(move |_ :events::Click| {
                menu_phase.set(MenuPhase::Animations);
            })
        }),
    ]
}

fn render_animations<T: AsSticker>(
    _stickers: Rc<Stickers<T>>,
    text: Rc<Text>,
    menu_phase: Mutable<MenuPhase>,
) -> Vec<Dom> {
    vec![
        html!("button-rect", {
            .prop("kind", "text")
            .prop("color", "blue")
            .child(html!("fa-icon", {
               .prop("icon", "fa-regular fa-chevron-left")
            }))
            .text("Back")
            .event(clone!(menu_phase => move |_ :events::Click| {
                menu_phase.set(MenuPhase::Main);
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::OnClick(_)) => true,
                    _ => false,
                }))
            }))
            .text("Hide on click")
            .event(clone!(menu_phase => move |_ :events::Click| {
                menu_phase.set(MenuPhase::OnClick);
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::UntilClick(_)) => true,
                    _ => false,
                }))
            }))
            .text("Show on click")
            .event(clone!(menu_phase => move |_ :events::Click| {
                menu_phase.set(MenuPhase::UntilClick);
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hover_animation.signal_cloned().map(|hover_animation| match hover_animation {
                    Some(_) => true,
                    _ => false,
                }))
            }))
            .text("Hover effect")
            .event(clone!(menu_phase => move |_ :events::Click| {
                menu_phase.set(MenuPhase::HoverAnimation);
            }))
        }),
    ]
}

fn render_hover_animations<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    text: Rc<Text>,
    menu_phase: Mutable<MenuPhase>,
) -> Vec<Dom> {
    vec![
        html!("button-rect", {
            .prop("kind", "text")
            .prop("color", "blue")
            .child(html!("fa-icon", {
                .prop("icon", "fa-regular fa-chevron-left")
            }))
            .text("Back")
            .event(move |_ :events::Click| {
                menu_phase.set(MenuPhase::Animations);
            })
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hover_animation.signal().map(|hover_animation| match hover_animation {
                    Some(HoverAnimation::Grow) => true,
                    _ => false,
                }))
            }))
            .text("Hover grow")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hover_animation.set_neq(Some(HoverAnimation::Grow));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hover_animation.signal().map(|hover_animation| match hover_animation {
                    Some(HoverAnimation::Tilt) => true,
                    _ => false,
                }))
            }))
            .text("Hover tilt")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hover_animation.set_neq(Some(HoverAnimation::Tilt));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hover_animation.signal().map(|hover_animation| match hover_animation {
                    Some(HoverAnimation::Buzz) => true,
                    _ => false,
                }))
            }))
            .text("Hover buzz")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hover_animation.set_neq(Some(HoverAnimation::Buzz));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hover_animation.signal().map(|hover_animation| match hover_animation {
                    None => true,
                    Some(_) => false,
                }))
            }))
            .text("No effect")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hover_animation.set_neq(None);
                stickers.call_change();
            }))
        }),
    ]
}

fn render_hide_on_click<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    text: Rc<Text>,
    menu_phase: Mutable<MenuPhase>,
) -> Vec<Dom> {
    vec![
        html!("button-rect", {
            .prop("kind", "text")
            .prop("color", "blue")
            .child(html!("fa-icon", {
               .prop("icon", "fa-regular fa-chevron-left")
            }))
            .text("Back")
            .event(move |_ :events::Click| {
                menu_phase.set(MenuPhase::Animations);
            })
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::OnClick(ShowHideAnimation::Appear)) => true,
                    _ => false,
                }))
            }))
            .text("Disappear")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hidden.set_neq(Some(StickerHidden::OnClick(Default::default())));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::OnClick(ShowHideAnimation::FadeInBottom)) => true,
                    _ => false,
                }))
            }))
            .text("Exit bottom")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hidden.set_neq(Some(StickerHidden::OnClick(ShowHideAnimation::FadeInBottom)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::OnClick(ShowHideAnimation::FadeInTop)) => true,
                    _ => false,
                }))
            }))
            .text("Exit top")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hidden.set_neq(Some(StickerHidden::OnClick(ShowHideAnimation::FadeInTop)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::OnClick(ShowHideAnimation::FadeInLeft)) => true,
                    _ => false,
                }))
            }))
            .text("Exit left")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hidden.set_neq(Some(StickerHidden::OnClick(ShowHideAnimation::FadeInLeft)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::OnClick(ShowHideAnimation::FadeInRight)) => true,
                    _ => false,
                }))
            }))
            .text("Exit right")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hidden.set_neq(Some(StickerHidden::OnClick(ShowHideAnimation::FadeInRight)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hidden.signal_cloned().map(|hidden| match hidden {
                    None => true,
                    Some(_) => false,
                }))
            }))
            .text("No effect")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hidden.set_neq(None);
                stickers.call_change();
            }))
        }),
    ]
}

fn render_hide_until_click<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    text: Rc<Text>,
    menu_phase: Mutable<MenuPhase>,
) -> Vec<Dom> {
    vec![
        html!("button-rect", {
            .prop("kind", "text")
            .prop("color", "blue")
            .child(html!("fa-icon", {
               .prop("icon", "fa-regular fa-chevron-left")
            }))
            .text("Back")
            .event(move |_ :events::Click| {
                menu_phase.set(MenuPhase::Animations);
            })
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::UntilClick(ShowHideAnimation::Appear)) => true,
                    _ => false,
                }))
            }))
            .text("Appear")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hidden.set_neq(Some(StickerHidden::UntilClick(Default::default())));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInBottom)) => true,
                    _ => false,
                }))
            }))
            .text("Enter bottom")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hidden.set_neq(Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInBottom)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInTop)) => true,
                    _ => false,
                }))
            }))
            .text("Enter top")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hidden.set_neq(Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInTop)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInLeft)) => true,
                    _ => false,
                }))
            }))
            .text("Enter left")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hidden.set_neq(Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInLeft)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInRight)) => true,
                    _ => false,
                }))
            }))
            .text("Enter right")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hidden.set_neq(Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInRight)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", text.hidden.signal_cloned().map(|hidden| match hidden {
                    None => true,
                    Some(_) => false,
                }))
            }))
            .text("No effect")
            .event(clone!(text, stickers => move |_ :events::Click| {
                text.hidden.set_neq(None);
                stickers.call_change();
            }))
        }),
    ]
}
