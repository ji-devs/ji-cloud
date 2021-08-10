use strum_macros::EnumIter;

#[repr(i16)]
#[derive(EnumIter, Debug, PartialEq, Copy, Clone, Eq, Hash)]
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

//it's up to the caller to ensure a valid value!
impl From<i16> for ImageTag {
    fn from(value:i16) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
