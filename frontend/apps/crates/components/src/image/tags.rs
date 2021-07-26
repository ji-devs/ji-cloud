use strum::IntoEnumIterator;
use strum_macros::EnumIter;


#[repr(i16)]
#[derive(EnumIter, Debug, PartialEq, Copy, Clone, Eq)]
pub enum ImageTag {
    BackgroundLayer1,
    BackgroundLayer2,
}

impl ImageTag {
    pub const fn STR_DISPLAY_NAME(&self) -> &'static str {
        match self {
            Self::BackgroundLayer1 => "Background Layer 1",
            Self::BackgroundLayer2 => "Background Layer 2",
        }
    }

    pub const fn as_index(&self) -> i16 {
        *self as i16
    }
}
