use super::common::*;
use crate::module::_groups::cards::lookup::Side;
use dominator::{html, Dom, DomBuilder};
use futures_signals::signal::{Always, Signal, SignalExt};
use shared::domain::jig::module::body::{
    ModeExt,
    _groups::cards::{Card, Mode},
};
use utils::prelude::*;
use web_sys::HtmlElement;

//For the use case when things are driven by Signals
pub struct DynamicCardOptions<'a, F, T, H, S, SOut>
where
    F: Signal<Item = bool> + 'static,
    T: Signal<Item = bool> + 'static,
    H: Signal<Item = bool> + 'static,
    S: Fn() -> SOut + 'static,
    SOut: Signal<Item = Option<SimpleTransform>> + 'static,
{
    pub card: &'a Card,
    pub back_card: Option<&'a Card>,
    pub flip_on_hover: bool,
    pub flipped: F,
    pub transparent: T,
    pub hidden: H,
    pub get_simple_transform: Option<S>,
    pub theme_id: ThemeId,
    pub size: Size,
    pub mode: Mode,
    pub style_kind: StyleKind,
    //should be set to match card and back_card will automatically
    //use the opposite
    pub side: Side,
    pub slot: Option<&'a str>,
}

//To make it easier to pass None::<NoTransform> for the get_simple_transform arg
//others can simply use always()
pub type NoTransform = fn() -> Always<Option<SimpleTransform>>;

impl<'a, F, T, H, S, SOut> DynamicCardOptions<'a, F, T, H, S, SOut>
where
    F: Signal<Item = bool> + 'static,
    T: Signal<Item = bool> + 'static,
    H: Signal<Item = bool> + 'static,
    S: Fn() -> SOut + 'static,
    SOut: Signal<Item = Option<SimpleTransform>> + 'static,
{
    pub fn new(
        card: &'a Card,
        theme_id: ThemeId,
        mode: Mode,
        side: Side,
        size: Size,
        flipped: F,
        transparent: T,
        hidden: H,
        get_simple_transform: Option<S>,
    ) -> Self {
        Self {
            card,
            theme_id,
            mode,
            side,
            size,
            flipped,
            transparent,
            hidden,
            get_simple_transform,
            //mimic default derive
            back_card: None,
            flip_on_hover: false,
            slot: None,
            style_kind: StyleKind::Theme,
        }
    }
}

pub fn render_dynamic_card<F, T, H, S, SOut>(options: DynamicCardOptions<F, T, H, S, SOut>) -> Dom
where
    F: Signal<Item = bool> + 'static,
    T: Signal<Item = bool> + 'static,
    H: Signal<Item = bool> + 'static,
    S: Fn() -> SOut + 'static,
    SOut: Signal<Item = Option<SimpleTransform>> + 'static,
{
    _render_dynamic_card(
        options,
        None::<fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>,
    )
}

pub fn render_dynamic_card_mixin<F, T, H, S, SOut, M>(
    options: DynamicCardOptions<F, T, H, S, SOut>,
    mixin: M,
) -> Dom
where
    F: Signal<Item = bool> + 'static,
    T: Signal<Item = bool> + 'static,
    H: Signal<Item = bool> + 'static,
    S: Fn() -> SOut + 'static,
    SOut: Signal<Item = Option<SimpleTransform>> + 'static,
    M: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    _render_dynamic_card(options, Some(mixin))
}

fn _render_dynamic_card<F, T, H, S, SOut, M>(
    options: DynamicCardOptions<F, T, H, S, SOut>,
    mixin: Option<M>,
) -> Dom
where
    F: Signal<Item = bool> + 'static,
    T: Signal<Item = bool> + 'static,
    H: Signal<Item = bool> + 'static,
    S: Fn() -> SOut + 'static,
    SOut: Signal<Item = Option<SimpleTransform>> + 'static,
    M: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let DynamicCardOptions {
        card,
        back_card,
        flip_on_hover,
        flipped,
        transparent,
        hidden,
        get_simple_transform,
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
        .property_signal("flipped", flipped)
        .property("theme", theme_id.as_str_id())
        .property("mode", mode.as_str_id())
        .property("side", side.as_str_id())
        .style_signal("visibility", transparent.map(|transparent| {
            if transparent {
                "hidden"
            } else {
                "visible"
            }
        }))
        .style_signal("display", hidden.map(|hidden| {
            if hidden {
                "none"
            } else {
                "block"
            }
        }))
        .apply_if(get_simple_transform.is_some(), move |dom| {
            let get_simple_transform = get_simple_transform.unwrap_ji();

            dom
                .style_signal("transform", {
                    get_simple_transform().map(|t| match t {
                        Some(t) => format!("scale({}) translate({}rem, {}rem)", t.scale, t.x, t.y),
                        None => String::from("none")
                    })
                })
                .property("hasTransform", true)
        })
        .child(render_media(&card, mode, theme_id, None))
        .apply_if(back_card.is_some(), |dom| {
            dom
                .property("doubleSided", true)
                .child(render_media(&back_card.unwrap_ji(), mode, theme_id, Some("backSideContent")))
        })
        .apply_if(mixin.is_some(), |dom| {
            (mixin.unwrap_ji()) (dom)
        })
    })
}
