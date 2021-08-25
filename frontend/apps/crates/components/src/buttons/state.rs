use std::rc::Rc;
use utils::prelude::*;

pub struct Button {
    pub style: ButtonStyle,
    pub on_click: Option<Box<dyn Fn()>>
}

pub enum ButtonStyle {
    Icon(ButtonStyleIcon)
}


pub enum ButtonStyleIcon {
    BlueX ,
    GreyKebab,
}

impl ButtonStyleIcon {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::BlueX => "circle-x-blue",
            Self::GreyKebab => "circle-kebab-grey",
        }
    }
}

impl ButtonStyle {
    pub const fn icon_str(&self) -> Option<&'static str> {
        match self {
            Self::Icon(icon) => Some(icon.as_str()),
            _ => None
        }
    }

    pub const fn element_str(&self) -> &'static str {
        match self {
            Self::Icon(_) => "button-icon"
        }
    }

}

impl Button {
    pub fn new(style: ButtonStyle, on_click: impl Fn() + 'static) -> Rc<Self> {
        Rc::new(Self {
            style,
            on_click: Some(Box::new(on_click))
        })
    }
}
