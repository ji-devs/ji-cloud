use utils::math::BoundsF64;
use web_sys::HtmlElement;
use std::rc::Rc;
use super::callbacks::*;

pub struct State {
    pub data: TooltipData,
    pub target: TooltipTarget
}

impl State {
    pub fn new(target: TooltipTarget, data: TooltipData) -> Self {
        Self {
            data,
            target
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Placement {
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

pub enum TooltipTarget {
    Element(HtmlElement, MoveStrategy),
    NormalizedBounds(BoundsF64, MoveStrategy),
}

pub enum TooltipData {
    Error(Rc<TooltipError>),
    Confirm(Rc<TooltipConfirm>),
    Bubble(Rc<TooltipBubble>),
}


pub struct TooltipError {
    pub placement:Placement, 
    pub slot: Option<String>, 
    pub body: String, 
    pub max_width: Option<f64>,
    pub callbacks: TooltipErrorCallbacks,
}

pub struct TooltipConfirm {
    pub placement:Placement, 
    pub slot: Option<String>, 
    pub header: String,
    pub confirm_label: String,
    pub cancel_label: String,
    pub max_width: Option<f64>,
    pub callbacks: TooltipConfirmCallbacks,
}

pub struct TooltipBubble {
    pub placement:Placement, 
    pub slot: Option<String>, 
    pub body: String, 
    pub max_width: Option<f64>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MoveStrategy {
    None,
    Destroy,
    Track
}

impl MoveStrategy {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Destroy => "destroy",
            Self::Track => "track"
        }
    }
}
