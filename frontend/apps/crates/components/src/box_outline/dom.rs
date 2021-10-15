use super::state::*;
use dominator::{html, Dom, DomBuilder};
use std::rc::Rc;
use web_sys::HtmlElement;

use utils::prelude::*;

impl BoxOutline {
    pub fn render(state: Rc<Self>, slot: Option<&str>) -> Dom {
        Self::_render_mixins(
            state,
            slot,
            None::<BoxOutlineMixins<MixinStub, MixinStub, MixinStub, MixinStub>>,
        )
    }

    pub fn render_mixins<A, B, C, D>(
        state: Rc<Self>,
        slot: Option<&str>,
        mixins: BoxOutlineMixins<A, B, C, D>,
    ) -> Dom
    where
        A: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
        B: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
        C: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
        D: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    {
        Self::_render_mixins(state, slot, Some(mixins))
    }

    fn _render_mixins<A, B, C, D>(
        state: Rc<Self>,
        slot: Option<&str>,
        mixins: Option<BoxOutlineMixins<A, B, C, D>>,
    ) -> Dom
    where
        A: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
        B: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
        C: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
        D: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    {
        html!("box-outline-absolute", {
            .property("lineHidden", state.style.line_hidden())
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_ji())
            })
            .apply_if(state.get_top_right_hover_only(), |dom| {
                dom.property("top-right-hover-only", true)
            })
            .apply_if(state.get_top_left_hover_only(), |dom| {
                dom.property("top-left-hover-only", true)
            })
            .style_signal("top", state.top_style_signal())
            .style_signal("left", state.left_style_signal())
            .style_signal("width", state.width_style_signal())
            .style_signal("height", state.height_style_signal())
            .apply_if(mixins.is_some(), |dom| {
                let mixins = mixins.unwrap_ji();

                let click_area = mixins.click_area;
                let top_right = mixins.top_right;
                let top_left = mixins.top_left;
                let main = mixins.main;

                dom
                    .apply_if(main.is_some(), |dom| {
                        dom.apply(main.unwrap_ji())
                    })
                    .apply_if(click_area.is_some(), |dom| {
                        dom
                            .property("click-area", true)
                            .child(html!("div", {
                                .property("slot", "click-area")
                                .apply(click_area.unwrap_ji())
                            }))
                    })
                    .apply_if(top_right.is_some(), |dom| {
                        dom.child(html!("div", {
                            .property("slot", "top-right")
                            .apply(top_right.unwrap_ji())
                        }))
                    })
                    .apply_if(top_left.is_some(), |dom| {
                        dom.child(html!("div", {
                            .property("slot", "top-left")
                            .apply(top_left.unwrap_ji())
                        }))
                    })
            })
        })
    }
}

pub struct BoxOutlineMixins<A, B, C, D>
where
    A: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    B: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    C: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    D: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    //main mixin
    pub main: Option<A>,
    //useful for attaching click events.
    //by default is sized to fit the box area
    pub click_area: Option<B>,

    //useful for adding positioning buttons
    //by default is nudged 16px outside the top-right corner
    pub top_right: Option<C>,

    //useful for adding positioning buttons
    //by default is nudged 16px outside the top-left corner
    pub top_left: Option<D>,
}
