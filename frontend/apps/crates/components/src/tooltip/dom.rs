use dominator::{Dom, html, clone};
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use utils::events;
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
    pub fn render_error(elem:&HtmlElement, placement:Placement, body:&str) -> Dom {
        html!("tooltip-error", {
            .property("body", body)
            .property("target", elem)
            .property("placement", placement.as_str())
        })
    }
}
