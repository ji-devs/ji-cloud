use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum_macros::EnumIter;

const STR_BACKGROUND_LAYER_1: &str = "Background Layer 1";
const STR_BACKGROUND_LAYER_2: &str = "Background Layer 2 (a.k.a. \"Overlay\")";
const STR_ISPY: &str = "I Spy";
const STR_MULTIPLE_CHOICE: &str = "Multiple Choice";
const STR_DRAG_AND_DROP: &str = "Drag & Drop";
const STR_VIDEO: &str = "Video";
const STR_NAVIGATION_BUTTON: &str = "Navigation Button";
const STR_TABLE: &str = "Table";
const STR_MAP: &str = "Map";
const STR_BOARDS: &str = "Boards";
const STR_TIMELINE: &str = "Timeline";
const STR_BOOK: &str = "Book";
const STR_PRINTABLES: &str = "Printables";
const STR_PHOTO_ALBUM: &str = "Photo Album";
const STR_MUSIC: &str = "Music";
const STR_COMIX: &str = "Comix";
const STR_WARDROBE: &str = "Wardrobe";
const STR_STAGE: &str = "Stage";

#[repr(i16)]
#[derive(EnumIter, Debug, PartialEq, Copy, Clone, Eq, Hash, FromPrimitive)]
pub enum ImageTag {
    BackgroundLayer1 = 0,
    BackgroundLayer2 = 1,
    Ispy = 2,
    MultipleChoice = 3,
    DragAndDrop = 4,
    Video = 5,
    NavigationButton = 6,
    Table = 7,
    Map = 8,
    Boards = 9,
    Timeline = 10,
    Book = 11,
    Printables = 12,
    PhotoAlbum = 13,
    Music = 14,
    Comix = 15,
    Wardrobe = 16,
    Stage = 17,
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
            Self::Table => STR_TABLE,
            Self::Map => STR_MAP,
            Self::Boards => STR_BOARDS,
            Self::Timeline => STR_TIMELINE,
            Self::Book => STR_BOOK,
            Self::Printables => STR_PRINTABLES,
            Self::PhotoAlbum => STR_PHOTO_ALBUM,
            Self::Music => STR_MUSIC,
            Self::Comix => STR_COMIX,
            Self::Wardrobe => STR_WARDROBE,
            Self::Stage => STR_STAGE,
        }
    }

    pub const fn as_index(&self) -> i16 {
        *self as i16
    }
}

impl TryFrom<i16> for ImageTag {
    type Error = ();
    fn try_from(value: i16) -> Result<Self, ()> {
        FromPrimitive::from_i16(value).ok_or(())
    }
}
