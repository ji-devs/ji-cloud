use crate::module::_groups::cards::lookup::Side;
use dominator::{html, Dom, DomBuilder};
use shared::domain::jig::module::body::{
    ModeExt,
    _groups::cards::{Card, Mode},
};
use utils::prelude::*;
use web_sys::HtmlElement;

use super::common::*;

pub struct CardOptions<'a> {
    pub card: &'a Card,
    pub back_card: Option<&'a Card>,
    pub flip_on_hover: bool,
    pub flipped: bool,
    pub transparent: bool,
    pub hidden: bool,
    pub simple_transform: Option<SimpleTransform>,
    pub theme_id: ThemeId,
    pub size: Size,
    pub mode: Mode,
    pub style_kind: StyleKind,
    //should be set to match card and back_card will automatically
    //use the opposite
    pub side: Side,
    pub slot: Option<&'a str>,
}

/*
 * flipped
 * opaque (visibility style)
 * hidden  (display style block vs none)
 * transform (Option)
 */
impl<'a> CardOptions<'a> {
    pub fn new(card: &'a Card, theme_id: ThemeId, mode: Mode, side: Side, size: Size) -> Self {
        Self {
            card,
            theme_id,
            mode,
            side,
            size,
            //mimic default derive
            back_card: None,
            flip_on_hover: false,
            flipped: false,
            transparent: false,
            hidden: false,
            simple_transform: None,
            slot: None,
            style_kind: StyleKind::Theme,
        }
    }
}

pub fn render_card(options: CardOptions) -> Dom {
    _render_card(
        options,
        None::<fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>,
    )
}

pub fn render_card_mixin<F>(options: CardOptions, mixin: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    _render_card(options, Some(mixin))
}

fn _render_card<F>(options: CardOptions, mixin: Option<F>) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let CardOptions {
        card,
        back_card,
        flip_on_hover,
        flipped,
        transparent,
        hidden,
        simple_transform,
        theme_id,
        mode,
        size,
        side,
        slot,
        style_kind,
    } = options;

    html!("play-card", {
        .style("visibility", "visible")
        .apply_if(slot.is_some(), |dom|
            dom.property("slot", slot.unwrap_ji())
        )
        .property("styleKind", style_kind.as_str_id())
        .property("size", size.as_str_id())
        .property("flipOnHover", flip_on_hover)
        .property("flipped", flipped)
        .property("theme", theme_id.as_str_id())
        .property("mode", mode.as_str_id())
        .property("side", side.as_str_id())
        .style("visibility", {
            if transparent {
                "hidden"
            } else {
                "visible"
            }
        })
        .style("display", {
            if hidden {
                "none"
            } else {
                "block"
            }
        })
        .apply_if(simple_transform.is_some(), |dom| {

            let t = simple_transform.unwrap_ji();

            dom
                .property("translateX", t.x)
                .property("translateY", t.y)
                .property("scale", t.scale)
                .property("hasTransform", true)
        })
        .child(render_media(card, mode, theme_id, None))
        .apply_if(back_card.is_some(), |dom| {
            dom
                .property("doubleSided", true)
                .child(render_media(back_card.unwrap_ji(), mode, theme_id, Some("backSideContent")))
        })
        .apply_if(mixin.is_some(), |dom| {
            (mixin.unwrap_ji()) (dom)
        })
    })
}
