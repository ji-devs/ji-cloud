use super::ParseUrlExt;
use anyhow::Context;
use futures_signals::signal::Mutable;
use regex::Regex;
use shared::domain::module::body::_groups::design::{
    GoogleSheetId, GoogleSheetsEmbed as RawGoogleSheetsEmbed,
};
use utils::unwrap::UnwrapJiExt;

#[derive(Clone, Debug, Default)]
pub struct PartialGoogleSheetsEmbed {
    pub url: Mutable<Option<GoogleSheetId>>,
}
impl PartialGoogleSheetsEmbed {
    pub fn full(&self) -> anyhow::Result<GoogleSheetsEmbed> {
        Ok(GoogleSheetsEmbed {
            url: Mutable::new(self.url.get_cloned().context("")?),
        })
    }
}

#[derive(Clone, Debug)]
pub struct GoogleSheetsEmbed {
    pub url: Mutable<GoogleSheetId>,
}
impl GoogleSheetsEmbed {
    pub fn partial(&self) -> PartialGoogleSheetsEmbed {
        PartialGoogleSheetsEmbed {
            url: Mutable::new(Some(self.url.get_cloned())),
        }
    }
    pub fn update_from_partial(&self, partial: &PartialGoogleSheetsEmbed) -> anyhow::Result<()> {
        self.url.set_neq(partial.url.get_cloned().context("")?);
        Ok(())
    }
}
impl From<RawGoogleSheetsEmbed> for GoogleSheetsEmbed {
    fn from(value: RawGoogleSheetsEmbed) -> Self {
        Self {
            url: Mutable::new(value.url),
        }
    }
}
impl From<&GoogleSheetsEmbed> for RawGoogleSheetsEmbed {
    fn from(value: &GoogleSheetsEmbed) -> Self {
        Self {
            url: value.url.get_cloned(),
        }
    }
}

impl ParseUrlExt for GoogleSheetId {
    fn try_parse(text: String) -> anyhow::Result<Self> {
        match get_id_from_url(&text) {
            Ok(_) => Ok(Self(text)),
            Err(_) => Err(anyhow::anyhow!("")),
        }
    }

    fn get_id(&self) -> &str {
        get_id_from_url(&self.0).expect_ji("Not a valid Google Sheet url")
    }
}

const SHARE_URL_BASE: &str = "https://docs.google.com/spreadsheets/d/e/";
const EMBED_IFRAME_BASE: &str = "<iframe ";
const ID_LENGTH: usize = 86;

fn get_id_from_url(url: &str) -> Result<&str, ()> {
    let id;

    if is_id(url) {
        return Ok(url);
    } else if url.starts_with(SHARE_URL_BASE) && url.len() >= SHARE_URL_BASE.len() + ID_LENGTH {
        id = extract_id_share(url);
    } else if url.starts_with(EMBED_IFRAME_BASE) && url.len() >= EMBED_IFRAME_BASE.len() + ID_LENGTH
    {
        id = extract_id_iframe(url);
    } else {
        return Err(());
    };

    if is_id(id) {
        Ok(id)
    } else {
        Err(())
    }
}

fn extract_id_share(url: &str) -> &str {
    let base_length = SHARE_URL_BASE.len();
    &url[base_length..(base_length + ID_LENGTH)]
}

fn extract_id_iframe(code: &str) -> &str {
    let id_index = code.find(SHARE_URL_BASE).unwrap_ji() + SHARE_URL_BASE.len();
    &code[id_index..(id_index + ID_LENGTH)]
}

fn is_id(id: &str) -> bool {
    let regex = Regex::new(r"^2PACX-[\w|_|\-]{80}$").unwrap_ji();
    regex.is_match(id)
}

#[cfg(test)]
mod tests {
    use shared::domain::module::body::_groups::design::GoogleSheetId;

    use crate::stickers::embed::types::ParseUrlExt;

    #[test]
    fn can_get_id_from_url() {
        let valid_url_vec = vec![
            r#"<iframe src="https://docs.google.com/spreadsheets/d/e/2PACX-1vQJ_SKKAS4a6sZ8Qyb7SQk1EzXinWx00EBmDciSVTgNoOiYtRCKa3Gifgvfp715PgawB6q_tTc3E5u-/pubhtml?widget=true&amp;headers=false"></iframe>"#,
            "https://docs.google.com/spreadsheets/d/e/2PACX-1vQJ_SKKAS4a6sZ8Qyb7SQk1EzXinWx00EBmDciSVTgNoOiYtRCKa3Gifgvfp715PgawB6q_tTc3E5u-/pubhtml?widget=true&amp;headers=false",
            "2PACX-1vQJ_SKKAS4a6sZ8Qyb7SQk1EzXinWx00EBmDciSVTgNoOiYtRCKa3Gifgvfp715PgawB6q_tTc3E5u-",
        ];

        for url in valid_url_vec {
            let id = GoogleSheetId::try_parse(url.to_string());

            assert!(id.is_ok());
        }
    }
}
