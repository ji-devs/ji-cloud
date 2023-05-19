use super::ParseUrlExt;
use anyhow::Context;
use futures_signals::signal::Mutable;
use regex::Regex;
use shared::domain::module::body::_groups::design::{
    GoogleSlideId, GoogleSlidesEmbed as RawGoogleSlidesEmbed,
};
use utils::unwrap::UnwrapJiExt;

#[derive(Clone, Debug, Default)]
pub struct PartialGoogleSlidesEmbed {
    pub url: Mutable<Option<GoogleSlideId>>,
}
impl PartialGoogleSlidesEmbed {
    pub fn full(&self) -> anyhow::Result<GoogleSlidesEmbed> {
        Ok(GoogleSlidesEmbed {
            url: Mutable::new(self.url.get_cloned().context("")?),
        })
    }
}

#[derive(Clone, Debug)]
pub struct GoogleSlidesEmbed {
    pub url: Mutable<GoogleSlideId>,
}
impl GoogleSlidesEmbed {
    pub fn partial(&self) -> PartialGoogleSlidesEmbed {
        PartialGoogleSlidesEmbed {
            url: Mutable::new(Some(self.url.get_cloned())),
        }
    }
    pub fn update_from_partial(&self, partial: &PartialGoogleSlidesEmbed) -> anyhow::Result<()> {
        self.url.set_neq(partial.url.get_cloned().context("")?);
        Ok(())
    }
}
impl From<RawGoogleSlidesEmbed> for GoogleSlidesEmbed {
    fn from(value: RawGoogleSlidesEmbed) -> Self {
        Self {
            url: Mutable::new(value.url),
        }
    }
}
impl From<&GoogleSlidesEmbed> for RawGoogleSlidesEmbed {
    fn from(value: &GoogleSlidesEmbed) -> Self {
        Self {
            url: value.url.get_cloned(),
        }
    }
}

impl ParseUrlExt for GoogleSlideId {
    fn try_parse(text: String) -> anyhow::Result<Self> {
        match get_id_from_url(&text) {
            Some(_) => Ok(Self(text)),
            None => Err(anyhow::anyhow!("")),
        }
    }

    fn get_id(&self) -> &str {
        get_id_from_url(&self.0).expect_ji("Not a valid Google Slide url")
    }
}

const SHARE_URL_BASE: &str = "https://docs.google.com/presentation/d/e/";
const EMBED_IFRAME_BASE: &str = "<iframe ";
const ID_LENGTH: usize = 86;

fn get_id_from_url(url: &str) -> Option<&str> {
    let id;

    if is_id(url) {
        return Some(url);
    } else if url.starts_with(SHARE_URL_BASE) && url.len() >= SHARE_URL_BASE.len() + ID_LENGTH {
        id = extract_id_share(url)?;
    } else if url.starts_with(EMBED_IFRAME_BASE) && url.len() >= EMBED_IFRAME_BASE.len() + ID_LENGTH
    {
        id = extract_id_iframe(url)?;
    } else {
        return None;
    };

    if is_id(id) {
        Some(id)
    } else {
        None
    }
}

fn extract_id_share(url: &str) -> Option<&str> {
    let base_length = SHARE_URL_BASE.len();
    url.get(base_length..(base_length + ID_LENGTH))
}

fn extract_id_iframe(code: &str) -> Option<&str> {
    let id_index = code.find(SHARE_URL_BASE)? + SHARE_URL_BASE.len();
    code.get(id_index..(id_index + ID_LENGTH))
}

fn is_id(id: &str) -> bool {
    let regex = Regex::new(r"^2PACX-[\w|_|\-]{80}$").unwrap_ji();
    regex.is_match(id)
}

#[cfg(test)]
mod tests {
    use shared::domain::module::body::_groups::design::GoogleSlideId;

    use crate::stickers::embed::types::ParseUrlExt;

    #[test]
    fn can_get_id_from_url() {
        let valid_url_vec = vec![
            r#"<iframe src="https://docs.google.com/presentation/d/e/2PACX-1vRZjr3og_4bSCwmudg9n3Ulpz1KwobzA8y47tChdrZ2RXT1vwuw4eZjJj2e30esmQErbbTerJ4GF4Xc/embed?start=false&loop=true&delayms=3000" frameborder="0" width="960" height="569" allowfullscreen="true" mozallowfullscreen="true" webkitallowfullscreen="true"></iframe>"#,
            "https://docs.google.com/presentation/d/e/2PACX-1vRZjr3og_4bSCwmudg9n3Ulpz1KwobzA8y47tChdrZ2RXT1vwuw4eZjJj2e30esmQErbbTerJ4GF4Xc/pub?start=true&loop=true&delayms=3000",
            "2PACX-1vRZjr3og_4bSCwmudg9n3Ulpz1KwobzA8y47tChdrZ2RXT1vwuw4eZjJj2e30esmQErbbTerJ4GF4Xc",
        ];

        for url in valid_url_vec {
            let id = GoogleSlideId::try_parse(url.to_string());

            assert!(id.is_ok());
        }
    }
}
