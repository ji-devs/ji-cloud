use super::ParseUrlExt;
use anyhow::Context;
use futures_signals::signal::Mutable;
use regex::Regex;
use shared::domain::module::body::_groups::design::{
    GoogleDocId, GoogleDocsEmbed as RawGoogleDocsEmbed,
};
use utils::unwrap::UnwrapJiExt;

#[derive(Clone, Debug, Default)]
pub struct PartialGoogleDocsEmbed {
    pub url: Mutable<Option<GoogleDocId>>,
}
impl PartialGoogleDocsEmbed {
    pub fn full(&self) -> anyhow::Result<GoogleDocsEmbed> {
        Ok(GoogleDocsEmbed {
            url: Mutable::new(self.url.get_cloned().context("")?),
        })
    }
}

#[derive(Clone, Debug)]
pub struct GoogleDocsEmbed {
    pub url: Mutable<GoogleDocId>,
}
impl GoogleDocsEmbed {
    pub fn partial(&self) -> PartialGoogleDocsEmbed {
        PartialGoogleDocsEmbed {
            url: Mutable::new(Some(self.url.get_cloned())),
        }
    }
    pub fn update_from_partial(&self, partial: &PartialGoogleDocsEmbed) -> anyhow::Result<()> {
        self.url.set_neq(partial.url.get_cloned().context("")?);
        Ok(())
    }
}
impl From<RawGoogleDocsEmbed> for GoogleDocsEmbed {
    fn from(value: RawGoogleDocsEmbed) -> Self {
        Self {
            url: Mutable::new(value.url),
        }
    }
}
impl From<&GoogleDocsEmbed> for RawGoogleDocsEmbed {
    fn from(value: &GoogleDocsEmbed) -> Self {
        Self {
            url: value.url.get_cloned(),
        }
    }
}

impl ParseUrlExt for GoogleDocId {
    fn try_parse(text: String) -> anyhow::Result<Self> {
        match get_id_from_url(&text) {
            Some(_) => Ok(Self(text)),
            None => Err(anyhow::anyhow!("")),
        }
    }

    fn get_id(&self) -> &str {
        get_id_from_url(&self.0).expect_ji("Not a valid Google Doc url")
    }
}

const SHARE_URL_BASE: &str = "https://docs.google.com/document/d/e/";
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
    use shared::domain::module::body::_groups::design::GoogleDocId;

    use crate::stickers::embed::types::ParseUrlExt;

    #[test]
    fn can_get_id_from_url() {
        let valid_url_vec = vec![
            r#"<iframe src="https://docs.google.com/document/d/e/2PACX-1vRrrIu56i3JhBHBGDsiWfBNwJT5mMpy4Cv45rxa_jt9qMJvMFSldyeEboMXYaniqNfAdU93rUhVKXQL/pub?embedded=true"></iframe>"#,
            "https://docs.google.com/document/d/e/2PACX-1vRrrIu56i3JhBHBGDsiWfBNwJT5mMpy4Cv45rxa_jt9qMJvMFSldyeEboMXYaniqNfAdU93rUhVKXQL/pub",
            "2PACX-1vRrrIu56i3JhBHBGDsiWfBNwJT5mMpy4Cv45rxa_jt9qMJvMFSldyeEboMXYaniqNfAdU93rUhVKXQL",
        ];

        for url in valid_url_vec {
            let id = GoogleDocId::try_parse(url.to_string());

            assert!(id.is_ok());
        }
    }
}
