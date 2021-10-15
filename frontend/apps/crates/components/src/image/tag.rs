use strum_macros::EnumIter;

#[repr(i16)]
#[derive(EnumIter, Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub enum ImageTag {
    BackgroundLayer1 = 0,
    BackgroundLayer2 = 1,
    Ispy = 2,
    MultipleChoice = 3,
    DragAndDrop = 4,
    Video = 5,
    NavigationButton = 6,
}

impl ImageTag {
    pub const fn STR_DISPLAY_NAME(&self) -> &'static str {
        match self {
            Self::BackgroundLayer1 => "Background Layer 1",
            Self::BackgroundLayer2 => "Background Layer 2 (a.k.a. \"Overlay\")",
            Self::Ispy => "I Spy",
            Self::MultipleChoice => "Multiple Choice",
            Self::DragAndDrop => "Drag and Drop",
            Self::Video => "Video",
            Self::NavigationButton => "Navigation Button",
        }
    }

    pub const fn as_index(&self) -> i16 {
        *self as i16
    }
}

//it's up to the caller to ensure a valid value!
impl From<i16> for ImageTag {
    fn from(value: i16) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
