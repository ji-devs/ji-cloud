use web_sys::HtmlElement;
use std::rc::Rc;

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

#[derive(Clone)]
pub enum TooltipData {
    Error(TooltipError),
    Confirm(TooltipConfirm),
}

#[derive(Clone)]
pub struct TooltipError {
    pub elem:HtmlElement, 
    pub placement:Placement, 
    pub slot: Option<String>, 
    pub body: String, 
    pub on_close: Option<Rc<Box<dyn Fn()>>>
}

#[derive(Clone)]
pub struct TooltipConfirm {
    pub elem:HtmlElement, 
    pub placement:Placement, 
    pub slot: Option<String>, 
    pub header: String,
    pub confirm_label: String,
    pub cancel_label: String,
    pub on_confirm: Rc<Box<dyn Fn()>>,
    pub on_cancel: Rc<Box<dyn Fn()>>
}

