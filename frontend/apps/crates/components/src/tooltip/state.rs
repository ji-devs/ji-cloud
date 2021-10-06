use super::callbacks::*;
use std::rc::Rc;
use utils::math::BoundsF64;
use web_sys::HtmlElement;

pub struct State {
    pub data: TooltipData,
    pub target: TooltipTarget,
}

impl State {
    pub fn new(target: TooltipTarget, data: TooltipData) -> Self {
        Self { data, target }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Anchor {
    Top,
    TopLeft,
    TopRight,
    Bottom,
    BottomLeft,
    BottomRight,
    Middle,
    MiddleLeft,
    MiddleRight,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ContentAnchor {
    Top,
    TopLeft,
    TopRight,
    Bottom,
    BottomLeft,
    BottomRight,
    Middle,
    MiddleLeft,
    MiddleRight,
    OppositeH,
    OppositeV,
    OppositeVH,
}

impl Anchor {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Top => "tm",
            Self::TopLeft => "tl",
            Self::TopRight => "tr",
            Self::Bottom => "bm",
            Self::BottomLeft => "bl",
            Self::BottomRight => "br",
            Self::Middle => "mm",
            Self::MiddleLeft => "ml",
            Self::MiddleRight => "mr",
        }
    }
}

impl ContentAnchor {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Top => "tm",
            Self::TopLeft => "tl",
            Self::TopRight => "tr",
            Self::Bottom => "bm",
            Self::BottomLeft => "bl",
            Self::BottomRight => "br",
            Self::Middle => "mm",
            Self::MiddleLeft => "ml",
            Self::MiddleRight => "mr",
            Self::OppositeH => "oppositeH",
            Self::OppositeV => "oppositeV",
            Self::OppositeVH => "oppositeVH",
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
    pub target_anchor: Anchor,
    pub content_anchor: ContentAnchor,
    pub body: String,
    pub max_width: Option<f64>,
    pub callbacks: TooltipErrorCallbacks,
}

pub struct TooltipConfirm {
    pub target_anchor: Anchor,
    pub content_anchor: ContentAnchor,
    pub header: String,
    pub confirm_label: String,
    pub cancel_label: String,
    pub max_width: Option<f64>,
    pub callbacks: TooltipConfirmCallbacks,
}

pub struct TooltipBubble {
    pub target_anchor: Anchor,
    pub content_anchor: ContentAnchor,
    pub body: String,
    pub max_width: Option<f64>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MoveStrategy {
    None,
    Destroy,
    Track,
}

impl MoveStrategy {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Destroy => "destroy",
            Self::Track => "track",
        }
    }
}
