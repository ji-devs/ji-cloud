use dominator::{clone, html, Dom};
use shared::domain::module::body::{HoverAnimation, ShowHideAnimation, StickerHidden};
use std::rc::Rc;
use strum_macros::EnumIs;
use utils::prelude::*;

use super::{
    super::super::state::{AsSticker, Stickers},
    super::state::Sprite,
};
use futures_signals::signal::{Mutable, ReadOnlyMutable, SignalExt};

#[derive(Clone, Copy, EnumIs)]
enum MenuPhase {
    Main,
    Animations,
    HoverAnimation,
    OnClick,
    UntilClick,
}

pub fn render_sticker_sprite_menu<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    index: ReadOnlyMutable<Option<usize>>,
    sprite: Rc<Sprite>,
) -> Dom {
    let menu_phase = Mutable::new(MenuPhase::Main);
    html!("div", {
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("gap", "10px")
        .style("align-items", "start")
        .children_signal_vec(menu_phase.signal().map(clone!(stickers, index, sprite, menu_phase => move |phase| {
            match phase {
                MenuPhase::Main => render_main(Rc::clone(&stickers), index.clone(), Rc::clone(&sprite), menu_phase.clone()),
                MenuPhase::Animations => render_animations(Rc::clone(&stickers), Rc::clone(&sprite), menu_phase.clone()),
                MenuPhase::HoverAnimation => render_hover_animations(Rc::clone(&stickers), Rc::clone(&sprite), menu_phase.clone()),
                MenuPhase::OnClick => render_hide_on_click(Rc::clone(&stickers), Rc::clone(&sprite), menu_phase.clone()),
                MenuPhase::UntilClick => render_hide_until_click(Rc::clone(&stickers), Rc::clone(&sprite), menu_phase.clone()),
            }
        })).to_signal_vec())
    })
}

fn render_main<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    index: ReadOnlyMutable<Option<usize>>,
    sprite: Rc<Sprite>,
    menu_phase: Mutable<MenuPhase>,
) -> Vec<Dom> {
    vec![
        html!("menu-line", {
            .prop("icon", "duplicate")
            .event(clone!(stickers, index, sprite => move |_evt:events::Click| {
                sprite.transform.close_menu();
                if let Some(index) = index.get() {
                    stickers.duplicate(index);
                }
            }))
        }),
        html!("menu-line", {
            .prop("icon", "move-to-front")
            .event(clone!(stickers, index, sprite => move |_ :events::Click| {
                sprite.transform.close_menu();
                if let Some(index) = index.get() {
                    stickers.move_to_front(index);
                }
            }))
        }),
        html!("menu-line", {
            .prop("icon", "move-forward")
            .event(clone!(stickers, index, sprite => move |_evt:events::Click| {
                sprite.transform.close_menu();
                if let Some(index) = index.get() {
                    stickers.move_forward(index);
                }
            }))
        }),
        html!("menu-line", {
            .prop("icon", "move-backward")
            .event(clone!(stickers, index, sprite => move |_evt:events::Click| {
                sprite.transform.close_menu();
                if let Some(index) = index.get() {
                    stickers.move_backward(index);
                }
            }))
        }),
        html!("menu-line", {
            .prop("icon", "move-to-back")
            .event(clone!(stickers, index, sprite => move |_evt:events::Click| {
                sprite.transform.close_menu();
                if let Some(index) = index.get() {
                    stickers.move_to_back(index);
                }
            }))
        }),
        /* post-beta
        html!("menu-line", {
            .prop("icon", "crop")
            .event(clone!(stickers, sprite => move |evt:events::Click| {
                log::info!("TODO!");
            }))
        }),
        */
        html!("menu-line", {
            .prop("icon", "remove-white")
            .event(clone!(stickers, sprite => move |_evt:events::Click| {
                sprite.transform.close_menu();
                sprite.remove_white();
                stickers.call_change();
            }))
        }),
        html!("menu-line", {
            .prop("icon", "flip-horizontal")
            .event(clone!(stickers, sprite => move |_evt:events::Click| {
                sprite.transform.close_menu();
                sprite.toggle_flip_horizontal();
                stickers.call_change();
            }))
        }),
        html!("menu-line", {
            .prop("icon", "flip-vertical")
            .event(clone!(stickers, sprite => move |_evt:events::Click| {
                sprite.transform.close_menu();
                sprite.toggle_flip_vertical();
                stickers.call_change();
            }))
        }),
        html!("menu-line", {
            .prop("icon", "animations")
            .event(move |_ :events::Click| {
                menu_phase.set(MenuPhase::Animations);
            })
        }),
        html!("menu-line", {
            .prop("icon", "delete")
            .event(clone!(stickers, index => move |_evt:events::Click| {
                sprite.transform.close_menu();
                if let Some(index) = index.get() {
                    stickers.delete_index(index);
                }
            }))
        }),
    ]
}

fn render_animations<T: AsSticker>(
    _stickers: Rc<Stickers<T>>,
    sprite: Rc<Sprite>,
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
                .prop_signal("checked", sprite.hidden.signal_cloned().map(|hidden| match hidden {
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
                .prop_signal("checked", sprite.hidden.signal_cloned().map(|hidden| match hidden {
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
                .prop_signal("checked", sprite.hover_animation.signal_cloned().map(|hover_animation| match hover_animation {
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
    sprite: Rc<Sprite>,
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
                .prop_signal("checked", sprite.hover_animation.signal().map(|hover_animation| match hover_animation {
                    Some(HoverAnimation::Grow) => true,
                    _ => false,
                }))
            }))
            .text("Hover grow")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hover_animation.set_neq(Some(HoverAnimation::Grow));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", sprite.hover_animation.signal().map(|hover_animation| match hover_animation {
                    Some(HoverAnimation::Tilt) => true,
                    _ => false,
                }))
            }))
            .text("Hover tilt")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hover_animation.set_neq(Some(HoverAnimation::Tilt));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", sprite.hover_animation.signal().map(|hover_animation| match hover_animation {
                    Some(HoverAnimation::Buzz) => true,
                    _ => false,
                }))
            }))
            .text("Hover buzz")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hover_animation.set_neq(Some(HoverAnimation::Buzz));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", sprite.hover_animation.signal().map(|hover_animation| match hover_animation {
                    None => true,
                    Some(_) => false,
                }))
            }))
            .text("No effect")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hover_animation.set_neq(None);
                stickers.call_change();
            }))
        }),
    ]
}

fn render_hide_on_click<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    sprite: Rc<Sprite>,
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
                .prop_signal("checked", sprite.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::OnClick(ShowHideAnimation::Appear)) => true,
                    _ => false,
                }))
            }))
            .text("Disappear")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hidden.set_neq(Some(StickerHidden::OnClick(Default::default())));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", sprite.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::OnClick(ShowHideAnimation::FadeInBottom)) => true,
                    _ => false,
                }))
            }))
            .text("Exit bottom")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hidden.set_neq(Some(StickerHidden::OnClick(ShowHideAnimation::FadeInBottom)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", sprite.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::OnClick(ShowHideAnimation::FadeInTop)) => true,
                    _ => false,
                }))
            }))
            .text("Exit top")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hidden.set_neq(Some(StickerHidden::OnClick(ShowHideAnimation::FadeInTop)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", sprite.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::OnClick(ShowHideAnimation::FadeInLeft)) => true,
                    _ => false,
                }))
            }))
            .text("Exit left")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hidden.set_neq(Some(StickerHidden::OnClick(ShowHideAnimation::FadeInLeft)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", sprite.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::OnClick(ShowHideAnimation::FadeInRight)) => true,
                    _ => false,
                }))
            }))
            .text("Exit right")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hidden.set_neq(Some(StickerHidden::OnClick(ShowHideAnimation::FadeInRight)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", sprite.hidden.signal_cloned().map(|hidden| match hidden {
                    None => true,
                    Some(_) => false,
                }))
            }))
            .text("No effect")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hidden.set_neq(None);
                stickers.call_change();
            }))
        }),
    ]
}

fn render_hide_until_click<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    sprite: Rc<Sprite>,
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
                .prop_signal("checked", sprite.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::UntilClick(ShowHideAnimation::Appear)) => true,
                    _ => false,
                }))
            }))
            .text("Appear")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hidden.set_neq(Some(StickerHidden::UntilClick(Default::default())));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", sprite.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInBottom)) => true,
                    _ => false,
                }))
            }))
            .text("Enter bottom")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hidden.set_neq(Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInBottom)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", sprite.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInTop)) => true,
                    _ => false,
                }))
            }))
            .text("Enter top")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hidden.set_neq(Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInTop)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", sprite.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInLeft)) => true,
                    _ => false,
                }))
            }))
            .text("Enter left")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hidden.set_neq(Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInLeft)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", sprite.hidden.signal_cloned().map(|hidden| match hidden {
                    Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInRight)) => true,
                    _ => false,
                }))
            }))
            .text("Enter right")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hidden.set_neq(Some(StickerHidden::UntilClick(ShowHideAnimation::FadeInRight)));
                stickers.call_change();
            }))
        }),
        html!("span", {
            .style("white-space", "nowrap")
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop_signal("checked", sprite.hidden.signal_cloned().map(|hidden| match hidden {
                    None => true,
                    Some(_) => false,
                }))
            }))
            .text("No effect")
            .event(clone!(sprite, stickers => move |_ :events::Click| {
                sprite.hidden.set_neq(None);
                stickers.call_change();
            }))
        }),
    ]
}
