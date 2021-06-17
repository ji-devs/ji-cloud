use once_cell::sync::OnceCell;
use rgb::RGBA8;
use serde::{
    de::{self, Deserializer},
    Serialize,
    Deserialize,
};
use std::{fmt, marker::PhantomData};
use crate::unwrap::UnwrapJiExt;
pub use shared::domain::jig::module::body::ThemeId;

pub const THEME_IDS:[ThemeId;3] = [
    ThemeId::Blank,
    ThemeId::Chalkboard, 
    ThemeId::HappyBrush
];

pub trait ThemeIdExt {
    fn get_colors(self) -> &'static [RGBA8];

    fn get_fonts(self) -> &'static [String];

    fn display_name(self) -> &'static str;

    fn as_str_id(self) -> &'static str;

    //It's safe to just call this whenever, it will lazily init the config
    fn map_theme<F, A>(self, mapper:F) -> A
    where
        F: FnOnce(&'static Theme) -> A;


    fn css_var_font_family(self, num: usize) -> String;

    fn css_var_color(self, num: usize) -> String;

}

impl ThemeIdExt for ThemeId {

    fn css_var_font_family(self, num: usize) -> String {
        format!("var(--theme-{}-font-family-{})", self.as_str_id(), num)
    }

    fn css_var_color(self, num: usize) -> String {
        format!("var(--theme-{}-color-{})", self.as_str_id(), num)
    }

    fn get_colors(self) -> &'static [RGBA8] {
        self.map_theme(|theme| theme.colors.as_slice())
    }

    fn get_fonts(self) -> &'static [String] {
        self.map_theme(|theme| theme.fonts.as_slice())
    }

    //TODO - tie to Localization
    fn display_name(self) -> &'static str {
        match self {
            Self::Blank => "blank",
            Self::Chalkboard => "Chalkboard", 
            Self::HappyBrush => "Happy Brush", 
        }
    }

    fn as_str_id(self) -> &'static str {
        match self {
            Self::Blank => "blank",
            Self::Chalkboard => "chalkboard", 
            Self::HappyBrush => "happy-brush", 
        }
    }

    //It's safe to just call this whenever, it will lazily init the config
    fn map_theme<F, A>(self, mapper:F) -> A 
    where
        F: FnOnce(&'static Theme) -> A
    {
        match THEMES.get() {
            None => {
                init_config();
                self.map_theme(mapper)
            }
            Some(themes) => {
                mapper(match self {
                    Self::Blank => &themes.blank,
                    Self::Chalkboard => &themes.chalkboard,
                    Self::HappyBrush => &themes.happy_brush,
                })
            }
        }
    }

}

//These are for storing the config statically
//access is via the public ThemeId getters
#[derive(Debug, Deserialize)]
struct Themes {
    pub blank: Theme,
    pub chalkboard: Theme,
    pub happy_brush: Theme,
}

#[derive(Debug, Deserialize)]
pub struct Theme {
    /// 3 values for now
    #[serde(deserialize_with = "hex_to_rgba8")]
    pub colors: Vec<RGBA8>,

    /// 3 values for now
    pub fonts: Vec<String> 
}

//Set lazily, first time as-needed
static THEMES: OnceCell<Themes> = OnceCell::new();

fn init_config() {
    let themes:Themes = serde_json::from_str(include_str!("../../../../config/themes.json")).expect("Invalid Themes");

    THEMES.set(themes).unwrap_ji()
}

//Deserializes the colors from Vec<String> to Vec<RGBA8>
//currently assumes all the strings are in the format 0xRRGGBB
//in the future we can enhance that to support more string types
//without breaking the api
fn hex_to_rgba8<'de, D>(deserializer: D) -> Result<Vec<RGBA8>, D::Error>
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


