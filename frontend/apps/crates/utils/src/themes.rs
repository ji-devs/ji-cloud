use crate::unwrap::UnwrapJiExt;
use once_cell::sync::Lazy;
use rgb::RGBA8;
use serde::{
    de::{self, Deserializer},
    Deserialize,
};
pub use shared::domain::jig::module::body::ThemeId;
use std::{fmt, marker::PhantomData};

//Set lazily, first time as-needed
static THEMES: Lazy<Themes> = Lazy::new(|| {
    let themes: Themes = serde_json::from_str(include_str!("../../../../config/themes.json"))
        .expect_ji("Invalid Themes");

    themes
});

pub const THEME_IDS: [ThemeId; 52] = [
    ThemeId::Blank,
    ThemeId::Jigzi,
    ThemeId::Chalkboard,
    ThemeId::MyNotebook,
    ThemeId::BackToSchool,
    ThemeId::MyWorkspace,
    ThemeId::Comix,
    ThemeId::Surreal,
    ThemeId::Abstract,
    ThemeId::Denim,
    ThemeId::HappyBrush,
    ThemeId::Graffiti,
    ThemeId::JewishText,
    ThemeId::ShabbatShalom,
    ThemeId::RoshHashana,
    ThemeId::AppleWithHoney,
    ThemeId::Pomegranate,
    ThemeId::YomKippur,
    ThemeId::HappySukkot,
    ThemeId::Sukkot,
    ThemeId::IlluminatingHanukkah,
    ThemeId::Chanukah,
    ThemeId::ChanukahLights,
    ThemeId::Purim,
    ThemeId::PurimFeast,
    ThemeId::PurimSweets,
    ThemeId::HappyPassover,
    ThemeId::PassoveMatza,
    ThemeId::PassoverSeder,
    ThemeId::HappyShavuot,
    ThemeId::ShavuotDishes,
    ThemeId::ShavuotFields,
    ThemeId::OurIsrael,
    ThemeId::Israel,
    ThemeId::JerusalemCity,
    ThemeId::JerusalemWall,
    ThemeId::LovelySpring,
    ThemeId::Spring,
    ThemeId::WatermelonSummer,
    ThemeId::SummerPool,
    ThemeId::ExcitingFall,
    ThemeId::Autumn,
    ThemeId::WinterSnow,
    ThemeId::IceAge,
    ThemeId::LostInSpace,
    ThemeId::Space,
    ThemeId::Camping,
    ThemeId::HappyBirthday,
    ThemeId::Jungle,
    ThemeId::OurPlanet,
    ThemeId::Theater,
    ThemeId::Travel,
];

pub trait ThemeIdExt {
    fn get_colors(self) -> &'static [RGBA8];

    fn get_text_editor_fonts(self) -> &'static [String];

    fn as_str_id(&self) -> &'static str;

    //It's safe to just call this whenever, it will lazily init the config
    fn map_theme<F, A>(self, mapper: F) -> A
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

    fn get_text_editor_fonts(self) -> &'static [String] {
        self.map_theme(|theme| theme.text_editor.font_list.as_slice())
    }

    fn as_str_id(&self) -> &'static str {
        match self {
            ThemeId::Blank => "blank",
            ThemeId::Jigzi => "jigzi",
            ThemeId::Chalkboard => "chalkboard",
            ThemeId::MyNotebook => "my-notebook",
            ThemeId::BackToSchool => "back-to-school",
            ThemeId::MyWorkspace => "my-workspace",
            ThemeId::Comix => "comix",
            ThemeId::Surreal => "surreal",
            ThemeId::Abstract => "abstract",
            ThemeId::Denim => "denim",
            ThemeId::HappyBrush => "happy-brush",
            ThemeId::Graffiti => "graffiti",
            ThemeId::JewishText => "jewish-text",
            ThemeId::ShabbatShalom => "shabbat-shalom",
            ThemeId::RoshHashana => "rosh-hashanah",
            ThemeId::AppleWithHoney => "apple-with-honey",
            ThemeId::Pomegranate => "pomegranate",
            ThemeId::YomKippur => "yom-kippur",
            ThemeId::HappySukkot => "happy-sukkot",
            ThemeId::Sukkot => "sukkot",
            ThemeId::IlluminatingHanukkah => "illuminating-hanukkah",
            ThemeId::Chanukah => "chanukah",
            ThemeId::ChanukahLights => "chanukah-lights",
            ThemeId::Purim => "purim",
            ThemeId::PurimFeast => "purim-feast",
            ThemeId::PurimSweets => "purim-sweets",
            ThemeId::HappyPassover => "happy-passover",
            ThemeId::PassoveMatza => "passover-matza",
            ThemeId::PassoverSeder => "passover-seder",
            ThemeId::HappyShavuot => "happy-shavuot",
            ThemeId::ShavuotDishes => "shavuot-dishes",
            ThemeId::ShavuotFields => "shavuot-fields",
            ThemeId::OurIsrael => "our-israel",
            ThemeId::Israel => "israel",
            ThemeId::JerusalemCity => "jerusalem-city",
            ThemeId::JerusalemWall => "jerusalem-wall",
            ThemeId::LovelySpring => "lovely-spring",
            ThemeId::Spring => "spring",
            ThemeId::WatermelonSummer => "watermelon-summer",
            ThemeId::SummerPool => "summer-pool",
            ThemeId::ExcitingFall => "exciting-fall",
            ThemeId::Autumn => "autumn",
            ThemeId::WinterSnow => "winter-snow",
            ThemeId::IceAge => "ice-age",
            ThemeId::LostInSpace => "lost-in-space",
            ThemeId::Space => "space",
            ThemeId::Camping => "camping",
            ThemeId::HappyBirthday => "happy-birthday",
            ThemeId::Jungle => "jungle",
            ThemeId::OurPlanet => "our-planet",
            ThemeId::Theater => "theater",
            ThemeId::Travel => "travel",
        }
    }

    //It's safe to just call this whenever, it will lazily init the config
    fn map_theme<F, A>(self, mapper: F) -> A
    where
        F: FnOnce(&'static Theme) -> A,
    {
        let theme = THEMES.get(self.as_str_id()).unwrap_ji();

        mapper(theme)
    }
}

//These are for storing the config statically
//access is via the public ThemeId getters
//TODO - would be cool to change this to HashMap<ThemeId, Theme>
type Themes = std::collections::HashMap<String, Theme>;
/*
#[derive(Debug, Deserialize)]
struct Themes {
    pub blank: Theme,
    pub chalkboard: Theme,
    pub happy_brush: Theme,
}
*/

#[derive(Debug, Deserialize)]
pub struct Theme {
    pub label: ThemeLabel,

    pub id: String,

    #[serde(rename(deserialize = "fontFamilies"))]
    pub font_familes: Vec<String>,
    /// 3 values for now
    #[serde(deserialize_with = "hex_to_rgba8")]
    pub colors: Vec<RGBA8>,

    #[serde(rename(deserialize = "textEditor"))]
    pub text_editor: TextEditor,

    #[serde(rename(deserialize = "cards"))]
    pub cards: Cards,
}

#[derive(Debug, Deserialize)]
pub struct ThemeLabel {
    pub en: String,
}

#[derive(Debug, Deserialize)]
pub struct TextEditor {
    pub h1: TextEditorVariant,
    pub h2: TextEditorVariant,
    pub p1: TextEditorVariant,
    pub p2: TextEditorVariant,
    #[serde(rename(deserialize = "fontList"))]
    pub font_list: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct TextEditorVariant {
    #[serde(rename(deserialize = "fontFamily"))]
    pub font_family: FontFamilyMapping,
    #[serde(rename(deserialize = "fontColor"))]
    pub font_color: ColorMapping,
    #[serde(rename(deserialize = "fontSize"))]
    pub font_size: f64,
}

#[derive(Debug, Deserialize)]
pub struct Cards {
    #[serde(rename(deserialize = "fontColor"))]
    pub font_color: ColorMapping,
    #[serde(rename(deserialize = "fillColor"))]
    pub fill_color: ColorMapping,
    #[serde(rename(deserialize = "borderColor"))]
    pub border_color: ColorMapping,
    #[serde(rename(deserialize = "fontFamily"))]
    pub font_family: FontFamilyMapping,
    #[serde(rename(deserialize = "fontFamilyLetteringLeft"))]
    pub font_family_lettering_left: FontFamilyMapping,
    #[serde(rename(deserialize = "fontFamilyLetteringRight"))]
    pub font_family_lettering_right: FontFamilyMapping,
}

pub type FontFamilyMapping = usize;
pub type ColorMapping = usize;

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
                let value = value.trim_start_matches('#');
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
