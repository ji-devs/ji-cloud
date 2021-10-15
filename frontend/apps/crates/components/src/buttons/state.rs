use std::rc::Rc;

pub struct Button {
    pub style: ButtonStyle,
    pub label: Option<String>,
    pub on_click: Option<Box<dyn Fn()>>,
}

impl Button {
    pub const fn icon_str(&self) -> Option<&'static str> {
        match &self.style {
            ButtonStyle::Icon(icon) | ButtonStyle::IconSized(icon, _) => Some(icon.as_str()),
        }
    }

    pub const fn element_str(&self) -> &'static str {
        match &self.style {
            ButtonStyle::Icon(_) | ButtonStyle::IconSized(_, _) => {
                if self.label.is_none() {
                    "button-icon"
                } else {
                    "button-icon-label"
                }
            }
        }
    }
}

pub enum ButtonStyle {
    Icon(ButtonStyleIcon),
    IconSized(ButtonStyleIcon, f64),
}

pub enum ButtonStyleIcon {
    BlueX,
    BluePlus,
    Audio,
    AudioStop,
    GreyKebab,
}

impl ButtonStyleIcon {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::BlueX => "circle-x-blue",
            Self::BluePlus => "circle-+-blue",
            Self::Audio => "audio",
            Self::AudioStop => "audio-stop",
            Self::GreyKebab => "circle-kebab-grey",
        }
    }
}

impl Button {
    pub fn new(style: ButtonStyle, on_click: impl Fn() + 'static) -> Rc<Self> {
        Rc::new(Self {
            style,
            label: None,
            on_click: Some(Box::new(on_click)),
        })
    }
    pub fn new_label(style: ButtonStyle, label: String, on_click: impl Fn() + 'static) -> Rc<Self> {
        Rc::new(Self {
            style,
            label: Some(label),
            on_click: Some(Box::new(on_click)),
        })
    }
}
