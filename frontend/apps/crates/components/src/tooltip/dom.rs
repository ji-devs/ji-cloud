use dominator::{Dom, html, clone};
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use crate::module::history::state::HistoryState;
use web_sys::HtmlElement;

pub struct TooltipDom {
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Placement {
    Auto,
    AutoStart,
    AutoEnd,
    Top,
    TopStart,
    TopEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Right,
    RightStart,
    RightEnd,
    Left,
    LeftStart,
    LeftEnd
}

impl Placement {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::AutoStart => "auto-start",
            Self::AutoEnd=> "auto-end",
            Self::Top => "top",
            Self::TopStart => "top-start",
            Self::TopEnd=> "top-end",
            Self::Bottom => "bottom",
            Self::BottomStart => "bottom-start",
            Self::BottomEnd=> "bottom-end",
            Self::Right => "right",
            Self::RightStart => "right-start",
            Self::RightEnd=> "rightend",
            Self::Left => "left",
            Self::LeftStart => "left-start",
            Self::LeftEnd=> "left-end",
        }
    }
}
//TODO - move on_undoredo into HistoryState itself
impl TooltipDom {
    pub fn render_error(elem:&HtmlElement, placement:Placement, slot: Option<&str>, body:&str, on_close: Option<Rc<impl Fn() + 'static>>) -> Dom {
        html!("tooltip-error", {
            .text(body)
            .apply_if(slot.is_some(), |dom| dom.property("slot", slot.unwrap_ji()))
            .property("maxWidth", 182)
            .property("target", elem)
            .property("placement", placement.as_str())
            .event(clone!(on_close => move |evt:events::Close| {
                if let Some(on_close) = &on_close {
                    on_close();
                }
            }))
        })
    }
    pub fn render_confirm(elem:&HtmlElement, placement:Placement, slot: Option<&str>, header:&str, confirm_label:&str, cancel_label:&str, on_confirm: Rc<impl Fn() + 'static>, on_cancel: Rc<impl Fn() + 'static>) -> Dom {
        html!("tooltip-confirm", {
            .apply_if(slot.is_some(), |dom| dom.property("slot", slot.unwrap_ji()))
            .property("header", header)
            .property("confirmLabel", confirm_label)
            .property("cancelLabel", cancel_label)
            .property("maxWidth", 332)
            .property("target", elem)
            .property("placement", placement.as_str())
            .event(clone!(on_confirm => move |evt:events::Accept| {
                on_confirm();
            }))
            .event(clone!(on_cancel => move |evt:events::Close| {
                on_cancel();
            }))
        })
    }
}
