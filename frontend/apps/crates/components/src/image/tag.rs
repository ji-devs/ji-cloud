use strum_macros::EnumIter;

const STR_BACKGROUND_LAYER_1: &str = "Background Layer 1";
const STR_BACKGROUND_LAYER_2: &str = "Background Layer 2 (a.k.a. \"Overlay\")";
const STR_ISPY: &str = "I Spy";
const STR_MULTIPLE_CHOICE: &str = "Multiple Choice";
const STR_DRAG_AND_DROP: &str = "Drag & Drop";
const STR_VIDEO: &str = "Video";
const STR_NAVIGATION_BUTTON: &str = "Navigation Button";

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
    pub const fn display_name(&self) -> &'static str {
        match self {
            Self::BackgroundLayer1 => STR_BACKGROUND_LAYER_1,
            Self::BackgroundLayer2 => STR_BACKGROUND_LAYER_2,
            Self::Ispy => STR_ISPY,
            Self::MultipleChoice => STR_MULTIPLE_CHOICE,
            Self::DragAndDrop => STR_DRAG_AND_DROP,
            Self::Video => STR_VIDEO,
            Self::NavigationButton => STR_NAVIGATION_BUTTON,
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
