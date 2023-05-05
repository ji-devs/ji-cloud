use anyhow::Context;
use futures_signals::signal::Mutable;
use regex::Regex;
use shared::domain::module::body::_groups::design::{
    DoneAction, VimeoEmbed as RawVimeoEmbed, VimeoUrl,
};
use url::Url;
use utils::prelude::*;

use super::ParseUrlExt;

#[derive(Clone, Default, Debug)]
pub struct PartialVimeoEmbed {
    pub url: Mutable<Option<VimeoUrl>>,
    pub clip: Mutable<bool>,
    pub start_at: Mutable<Option<u32>>,
    pub end_at: Mutable<Option<u32>>,
    pub captions: Mutable<bool>,
    pub muted: Mutable<bool>,
    pub autoplay: Mutable<bool>,
    pub done_action: Mutable<Option<DoneAction>>,
}
impl PartialVimeoEmbed {
    pub fn full(&self) -> anyhow::Result<VimeoEmbed> {
        Ok(VimeoEmbed {
            url: Mutable::new(self.url.get_cloned().context("")?),
            clip: self.clip.clone(),
            start_at: self.start_at.clone(),
            end_at: self.end_at.clone(),
            captions: self.captions.clone(),
            muted: self.muted.clone(),
            autoplay: self.autoplay.clone(),
            done_action: self.done_action.clone(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct VimeoEmbed {
    pub url: Mutable<VimeoUrl>,
    pub clip: Mutable<bool>,
    pub start_at: Mutable<Option<u32>>,
    pub end_at: Mutable<Option<u32>>,
    pub captions: Mutable<bool>,
    pub muted: Mutable<bool>,
    pub autoplay: Mutable<bool>,
    pub done_action: Mutable<Option<DoneAction>>,
}
impl VimeoEmbed {
    pub fn partial(&self) -> PartialVimeoEmbed {
        PartialVimeoEmbed {
            url: Mutable::new(Some(self.url.get_cloned())),
            clip: self.clip.clone(),
            start_at: self.start_at.clone(),
            end_at: self.end_at.clone(),
            captions: self.captions.clone(),
            muted: self.muted.clone(),
            autoplay: self.autoplay.clone(),
            done_action: self.done_action.clone(),
        }
    }
    pub fn update_from_partial(&self, partial: &PartialVimeoEmbed) -> anyhow::Result<()> {
        self.clip.set_neq(partial.clip.get());
        self.start_at.set_neq(partial.start_at.get());
        self.end_at.set_neq(partial.end_at.get());
        self.captions.set_neq(partial.captions.get());
        self.muted.set_neq(partial.muted.get());
        self.autoplay.set_neq(partial.autoplay.get());
        self.done_action.set_neq(partial.done_action.get());
        self.url.set_neq(partial.url.get_cloned().context("")?);
        Ok(())
    }
}
impl From<RawVimeoEmbed> for VimeoEmbed {
    fn from(value: RawVimeoEmbed) -> Self {
        Self {
            url: Mutable::new(value.url),
            clip: Mutable::new(value.start_at.is_some() || value.end_at.is_some()),
            start_at: Mutable::new(value.start_at),
            end_at: Mutable::new(value.end_at),
            captions: Mutable::new(value.captions),
            muted: Mutable::new(value.muted),
            autoplay: Mutable::new(value.autoplay),
            done_action: Mutable::new(value.done_action),
        }
    }
}
impl From<&VimeoEmbed> for RawVimeoEmbed {
    fn from(value: &VimeoEmbed) -> Self {
        Self {
            url: value.url.get_cloned(),
            start_at: value.start_at.get(),
            end_at: value.end_at.get(),
            captions: value.captions.get(),
            muted: value.muted.get(),
            autoplay: value.autoplay.get(),
            done_action: value.done_action.get(),
        }
    }
}

impl ParseUrlExt for VimeoUrl {
    fn try_parse(text: String) -> anyhow::Result<Self> {
        match get_id_from_url(&text) {
            Ok(_) => Ok(Self(text)),
            Err(_) => Err(anyhow::anyhow!("")),
        }
    }

    fn get_id(&self) -> &str {
        get_id_from_url(&self.0).expect_ji("Not a valid Vimeo url")
    }
}

const ANY_VIMEO_DOMAIN: &str = "youtube.com";
const REGULAR_URL_BASE: &str = "https://www.vimeo.com/";
const EMBED_IFRAME_BASE: &str = "<iframe ";
const EMBED_URL_BASE: &str = "https://player.vimeo.com/video/";
const ID_LENGTH: usize = 9;

fn get_id_from_url(url: &str) -> anyhow::Result<&str> {
    let id;
    //when is_id passes all tests, this can be removed
    let mut check_extracted_id = true;
    if is_id(url) {
        return Ok(url);
    } else if url.starts_with(REGULAR_URL_BASE) {
        id = extract_id_regular(url);
    } else if url.starts_with(EMBED_IFRAME_BASE) && url.len() >= EMBED_IFRAME_BASE.len() + ID_LENGTH
    {
        id = extract_id_iframe(url).context("")?;
    } else if url.starts_with(EMBED_URL_BASE) && url.len() >= EMBED_URL_BASE.len() + ID_LENGTH {
        log::info!("herer");
        id = extract_id_iframe(url).context("")?;
    } else if url.contains(ANY_VIMEO_DOMAIN) {
        let url = match url.find("http") {
            Some(pos) => &url[pos..],
            None => url,
        };

        match Url::parse(url) {
            Ok(real_url) => match real_url.query_pairs().find(|pair| pair.0 == "v") {
                Some(_) => {
                    id = extract_any_v(url);
                    check_extracted_id = false;
                }
                None => anyhow::bail!(""),
            },
            _ => anyhow::bail!(""),
        }
    } else {
        anyhow::bail!("");
    };

    if !check_extracted_id || is_id(id) {
        Ok(id)
    } else {
        anyhow::bail!("")
    }
}

fn extract_any_v(url: &str) -> &str {
    let start_bytes = url.find("v=").unwrap_or(0) + 2;
    let rest = &url[start_bytes..];
    let end_bytes = start_bytes + rest.find('&').unwrap_or(rest.len());
    &url[start_bytes..end_bytes]
}

fn extract_id_regular(url: &str) -> &str {
    let base_length = REGULAR_URL_BASE.len();
    &url[base_length..(base_length + 11)]
}

fn extract_id_iframe(code: &str) -> Option<&str> {
    let id_index = code.find(EMBED_URL_BASE).unwrap_ji() + EMBED_URL_BASE.len();
    code.get(id_index..(id_index + 11))
}

fn is_id(id: &str) -> bool {
    let regex = Regex::new(&format!("^[0-9]{{{ID_LENGTH}}}$")).unwrap_ji();
    regex.is_match(id)
}

#[cfg(test)]
mod tests {
    use shared::domain::module::body::_groups::design::VimeoUrl;

    use crate::stickers::embed::types::ParseUrlExt;

    #[test]
    fn can_get_id_from_url() {
        let valid_url_vec = vec![
            "https://vimeo.com/317852618",
            "https://vimeo.com/317852618#t=60s",
            r#"<iframe src="https://player.vimeo.com/video/317852618?h=7b3f53e9a8" width="640" height="360" frameborder="0" allow="autoplay; fullscreen; picture-in-picture" allowfullscreen></iframe>"#,
            r#"<iframe src="https://player.vimeo.com/video/317852618?h=7b3f53e9a8" width="640" height="360" frameborder="0" allow="autoplay; fullscreen; picture-in-picture" allowfullscreen></iframe>
            <p><a href="https://vimeo.com/317852618">Rust Compiler Source History Visualization</a> from <a href="https://vimeo.com/user1428435">j27</a> on <a href="https://vimeo.com">Vimeo</a>.</p>"#,
            r#"<iframe src="https://player.vimeo.com/video/317852618?h=7b3f53e9a8"></iframe>"#,
            "https://player.vimeo.com/video/317852618?h=7b3f53e9a8",
            "317852618",
        ];

        for url in valid_url_vec {
            let id = VimeoUrl::try_parse(url.to_string());

            assert!(id.is_ok());
        }
    }
}
