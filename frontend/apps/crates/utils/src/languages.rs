use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct Language (pub &'static str, pub &'static str);

pub const LANGUAGE_CODE_EN: &'static str = "en";
pub const LANGUAGE_CODE_HE: &'static str = "he";
pub const LANGUAGE_CODE_FR: &'static str = "fr";
const STR_LANGUAGE_ENGLISH: &'static str = "English";
const STR_LANGUAGE_HEBREW: &'static str = "Hebrew";
const STR_LANGUAGE_FRENCH: &'static str = "French";


lazy_static! {
    pub static ref LANGUAGES: Vec<Language> = vec![
        Language(LANGUAGE_CODE_EN, STR_LANGUAGE_ENGLISH),
        Language(LANGUAGE_CODE_HE, STR_LANGUAGE_HEBREW),
        Language(LANGUAGE_CODE_FR, STR_LANGUAGE_FRENCH),
    ];
}

impl Language {
    pub fn code_to_display_name(code: &str) -> &'static str {
        match code {
            LANGUAGE_CODE_EN => STR_LANGUAGE_ENGLISH,
            LANGUAGE_CODE_HE => STR_LANGUAGE_HEBREW,
            LANGUAGE_CODE_FR => STR_LANGUAGE_FRENCH,
            _ => "?"
        }
    }

    pub fn code(&self) -> &'static str {
        match self.1 {
            STR_LANGUAGE_ENGLISH => LANGUAGE_CODE_EN,
            STR_LANGUAGE_HEBREW => LANGUAGE_CODE_HE,
            STR_LANGUAGE_FRENCH => LANGUAGE_CODE_FR,
            _ => "?"
        }
    }

    pub fn display_name(&self) -> &'static str {
        Self::code_to_display_name(self.0)
    }
}
