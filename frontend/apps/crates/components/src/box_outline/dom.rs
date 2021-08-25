use dominator::{clone, html, Dom, DomBuilder};
use super::state::*;
use std::rc::Rc;
use web_sys::HtmlElement;
use wasm_bindgen::prelude::*; 
use utils::prelude::*;

impl BoxOutline {
    pub fn render(state: Rc<Self>, slot: Option<&str>) -> Dom {
        Self::_render_mixins(
            state,
            slot,
            None::<BoxOutlineMixins<MixinStub, MixinStub>>,
        )
    }
    pub fn render_mixins<A, B>(state: Rc<Self>, slot: Option<&str>, mixins: BoxOutlineMixins<A, B>) -> Dom 
    where
        A: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
        B: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> 
        
    {
        Self::_render_mixins(state, slot, Some(mixins))
    }

    fn _render_mixins<A, B>(state: Rc<Self>, slot: Option<&str>, mixins: Option<BoxOutlineMixins<A, B>>) -> Dom
    where
        A: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
        B: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> 
    {
        html!("box-outline-absolute", {
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_ji())
            })
            .apply_if(state.get_top_right_hover_only(), |dom| {
                dom.property("top-right-hover-only", true)
            })
            .style_signal("top", state.top_style_signal()) 
            .style_signal("left", state.left_style_signal()) 
            .style_signal("width", state.width_style_signal()) 
            .style_signal("height", state.height_style_signal()) 
            .apply_if(mixins.is_some(), |dom| {
                let mixins = mixins.unwrap_ji();

                let click_area = mixins.click_area;
                let top_right = mixins.top_right;

                dom
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
            })
        })
    }

}

pub struct BoxOutlineMixins<A, B> 
    where
        A: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
        B: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    //useful for attaching click events. 
    //by default is sized to fit the box area
    pub click_area: Option<A>,

    //useful for adding positioning buttons
    //by default is nudged 16px outside the top-right corner
    pub top_right: Option<B>,
}
