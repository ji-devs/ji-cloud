use crate::unwrap::UnwrapJiExt;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

pub fn font_families_iter() -> impl Iterator<Item = &'static str> {
    FONTS.keys().map(|x| x.as_ref())
}

static FONTS: Lazy<Fonts> = Lazy::new(|| {
    let fonts: Fonts = serde_json::from_str(include_str!("../../../../config/fonts.json"))
        .expect_ji("Invalid Fonts");

    fonts
});

type Fonts = HashMap<String, FontInfo>;

#[derive(Debug, Deserialize)]
pub struct FontInfo {
    pub file: String,
    pub format: String,
    pub range: Option<String>,
}
