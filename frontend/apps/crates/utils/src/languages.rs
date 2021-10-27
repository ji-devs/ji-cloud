use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct Language(pub &'static str, pub &'static str);

const LANGUAGE_ENGLISH_CODE: &str = "en";
const STR_LANGUAGE_ENGLISH: &str = "English";
const LANGUAGE_HEBREW_CODE: &str = "he";
const STR_LANGUAGE_HEBREW: &str = "Hebrew";
const LANGUAGE_SPANISH_CODE: &str = "es";
const STR_LANGUAGE_SPANISH: &str = "Spanish";
const LANGUAGE_PORTUGUESE_CODE: &str = "pt	";
const STR_LANGUAGE_PORTUGUESE: &str = "Portuguese";
const LANGUAGE_RUSSIAN_CODE: &str = "ru";
const STR_LANGUAGE_RUSSIAN: &str = "Russian";
const LANGUAGE_FRENCH_CODE: &str = "fr";
const STR_LANGUAGE_FRENCH: &str = "French";
const LANGUAGE_DUTCH_CODE: &str = "nl";
const STR_LANGUAGE_DUTCH: &str = "Dutch";
const LANGUAGE_SWEDISH_CODE: &str = "sv";
const STR_LANGUAGE_SWEDISH: &str = "Swedish";
const LANGUAGE_ARABIC_CODE: &str = "ar";
const STR_LANGUAGE_ARABIC: &str = "Arabic";
const LANGUAGE_GERMAN_CODE: &str = "de";
const STR_LANGUAGE_GERMAN: &str = "German";
const LANGUAGE_HUNGARIAN_CODE: &str = "hu";
const STR_LANGUAGE_HUNGARIAN: &str = "Hungarian";
const LANGUAGE_ITALIAN_CODE: &str = "it";
const STR_LANGUAGE_ITALIAN: &str = "Italian";
const LANGUAGE_YIDDISH_CODE: &str = "yi";
const STR_LANGUAGE_YIDDISH: &str = "Yiddish";

lazy_static! {
    pub static ref PLATFORM_LANGUAGES: Vec<Language> =
        vec![Language(LANGUAGE_ENGLISH_CODE, STR_LANGUAGE_ENGLISH),];
    pub static ref EMAIL_LANGUAGES: Vec<Language> = vec![
        Language(LANGUAGE_ENGLISH_CODE, STR_LANGUAGE_ENGLISH),
        Language(LANGUAGE_HEBREW_CODE, STR_LANGUAGE_HEBREW),
        Language(LANGUAGE_FRENCH_CODE, STR_LANGUAGE_FRENCH),
    ];
    pub static ref JIG_LANGUAGES: Vec<Language> = vec![
        Language(LANGUAGE_ENGLISH_CODE, STR_LANGUAGE_ENGLISH),
        Language(LANGUAGE_HEBREW_CODE, STR_LANGUAGE_HEBREW),
        Language(LANGUAGE_SPANISH_CODE, STR_LANGUAGE_SPANISH),
        Language(LANGUAGE_PORTUGUESE_CODE, STR_LANGUAGE_PORTUGUESE),
        Language(LANGUAGE_RUSSIAN_CODE, STR_LANGUAGE_RUSSIAN),
        Language(LANGUAGE_FRENCH_CODE, STR_LANGUAGE_FRENCH),
        Language(LANGUAGE_DUTCH_CODE, STR_LANGUAGE_DUTCH),
        Language(LANGUAGE_SWEDISH_CODE, STR_LANGUAGE_SWEDISH),
        Language(LANGUAGE_ARABIC_CODE, STR_LANGUAGE_ARABIC),
        Language(LANGUAGE_GERMAN_CODE, STR_LANGUAGE_GERMAN),
        Language(LANGUAGE_HUNGARIAN_CODE, STR_LANGUAGE_HUNGARIAN),
        Language(LANGUAGE_ITALIAN_CODE, STR_LANGUAGE_ITALIAN),
        Language(LANGUAGE_YIDDISH_CODE, STR_LANGUAGE_YIDDISH),
    ];
}

impl Language {
    pub fn code_to_display_name(code: &str) -> &'static str {
        match code {
            LANGUAGE_ENGLISH_CODE => STR_LANGUAGE_ENGLISH,
            LANGUAGE_HEBREW_CODE => STR_LANGUAGE_HEBREW,
            LANGUAGE_SPANISH_CODE => STR_LANGUAGE_SPANISH,
            LANGUAGE_PORTUGUESE_CODE => STR_LANGUAGE_PORTUGUESE,
            LANGUAGE_RUSSIAN_CODE => STR_LANGUAGE_RUSSIAN,
            LANGUAGE_FRENCH_CODE => STR_LANGUAGE_FRENCH,
            LANGUAGE_DUTCH_CODE => STR_LANGUAGE_DUTCH,
            LANGUAGE_SWEDISH_CODE => STR_LANGUAGE_SWEDISH,
            LANGUAGE_ARABIC_CODE => STR_LANGUAGE_ARABIC,
            LANGUAGE_GERMAN_CODE => STR_LANGUAGE_GERMAN,
            LANGUAGE_HUNGARIAN_CODE => STR_LANGUAGE_HUNGARIAN,
            LANGUAGE_ITALIAN_CODE => STR_LANGUAGE_ITALIAN,
            LANGUAGE_YIDDISH_CODE => STR_LANGUAGE_YIDDISH,
            _ => "?",
        }
    }

    pub fn code(&self) -> &'static str {
        match self.1 {
            STR_LANGUAGE_ENGLISH => LANGUAGE_ENGLISH_CODE,
            STR_LANGUAGE_HEBREW => LANGUAGE_HEBREW_CODE,
            STR_LANGUAGE_SPANISH => LANGUAGE_SPANISH_CODE,
            STR_LANGUAGE_PORTUGUESE => LANGUAGE_PORTUGUESE_CODE,
            STR_LANGUAGE_RUSSIAN => LANGUAGE_RUSSIAN_CODE,
            STR_LANGUAGE_FRENCH => LANGUAGE_FRENCH_CODE,
            STR_LANGUAGE_DUTCH => LANGUAGE_DUTCH_CODE,
            STR_LANGUAGE_SWEDISH => LANGUAGE_SWEDISH_CODE,
            STR_LANGUAGE_ARABIC => LANGUAGE_ARABIC_CODE,
            STR_LANGUAGE_GERMAN => LANGUAGE_GERMAN_CODE,
            STR_LANGUAGE_HUNGARIAN => LANGUAGE_HUNGARIAN_CODE,
            STR_LANGUAGE_ITALIAN => LANGUAGE_ITALIAN_CODE,
            STR_LANGUAGE_YIDDISH => LANGUAGE_YIDDISH_CODE,
            _ => "?",
        }
    }

    pub fn display_name(&self) -> &'static str {
        Self::code_to_display_name(self.0)
    }
}

impl Default for Language {
    fn default() -> Self {
        Self(LANGUAGE_ENGLISH_CODE, STR_LANGUAGE_ENGLISH)
    }
}
