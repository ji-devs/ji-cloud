use once_cell::sync::OnceCell;
use rgb::RGBA8;
use serde::{
    de::{self, Deserializer},
    Deserialize,
};
use std::{fmt, marker::PhantomData};

use crate::unwrap::UnwrapJiExt;

static THEMES: OnceCell<Themes> = OnceCell::new();

#[derive(Debug, Deserialize)]
struct Themes {
    pub chalkboard: Theme,
    pub happy_brush: Theme,
}
#[derive(Debug, Deserialize)]
struct Theme {
    #[serde(deserialize_with = "hex_to_rgba8")]
    pub colors: Vec<RGBA8>,
}

pub fn init() {
    let themes: Themes = serde_json::from_str(include_str!("../../../../config/themes.json")).expect("Invalid Themes");

    THEMES.set(themes).unwrap_ji()
}


pub fn hex_to_rgba8<'de, D>(deserializer: D) -> Result<Vec<RGBA8>, D::Error>
where
    D: Deserializer<'de>,
{
    struct ColorVec(PhantomData<Vec<RGBA8>>);

    impl<'de> de::Visitor<'de> for ColorVec {
        type Value = Vec<RGBA8>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("List of Colors as hex values")
        }

        fn visit_seq<S>(self, mut visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            let mut out: Vec<RGBA8> = Vec::with_capacity(visitor.size_hint().unwrap_or(0));

            // While there are entries remaining in the input, add them
            // into our vec.
            while let Some(value) = visitor.next_element::<String>()? {
                let value = value.trim_start_matches("0x");
                let value = u32::from_str_radix(value, 16)
                    .map_err(|_| serde::de::Error::custom(format!("invalid color [{}]!", value)))?;

                let r = ((value & 0xFF0000) >> 16) as u8;
                let g = ((value & 0x00FF00) >> 8) as u8;
                let b = (value & 0x0000FF) as u8;
                out.push(RGBA8::new(r, g, b, 255));
            }

            Ok(out)
        }
    }

    deserializer.deserialize_any(ColorVec(PhantomData))
}

pub enum ThemeId {
    Chalkboard,
    HappyBrush,
}
impl ThemeId {
    pub fn get_colors(&self) -> &'static [RGBA8] {

        if THEMES.get().is_none() {
            init();
        }

        THEMES
            .get()
            .map(|themes| match self {
                Self::Chalkboard => themes.chalkboard.colors.as_slice(),
                Self::HappyBrush => themes.happy_brush.colors.as_slice(),
            })
            .unwrap_ji()
    }
}
