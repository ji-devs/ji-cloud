use anyhow::Context;
use futures_signals::signal::Mutable;
use regex::Regex;
use shared::domain::module::body::_groups::design::{
    DoneAction, YoutubeEmbed as RawYoutubeEmbed, YoutubeUrl,
};
use url::Url;
use utils::prelude::*;

use super::ParseUrlExt;

#[derive(Clone, Default, Debug)]
pub struct PartialYoutubeEmbed {
    pub url: Mutable<Option<YoutubeUrl>>,
    pub clip: Mutable<bool>,
    pub start_at: Mutable<Option<u32>>,
    pub end_at: Mutable<Option<u32>>,
    pub captions: Mutable<bool>,
    pub muted: Mutable<bool>,
    pub autoplay: Mutable<bool>,
    pub done_action: Mutable<Option<DoneAction>>,
}
impl PartialYoutubeEmbed {
    pub fn full(&self) -> anyhow::Result<YoutubeEmbed> {
        Ok(YoutubeEmbed {
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
pub struct YoutubeEmbed {
    pub url: Mutable<YoutubeUrl>,
    pub clip: Mutable<bool>,
    pub start_at: Mutable<Option<u32>>,
    pub end_at: Mutable<Option<u32>>,
    pub captions: Mutable<bool>,
    pub muted: Mutable<bool>,
    pub autoplay: Mutable<bool>,
    pub done_action: Mutable<Option<DoneAction>>,
}
impl YoutubeEmbed {
    pub fn partial(&self) -> PartialYoutubeEmbed {
        PartialYoutubeEmbed {
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
    pub fn update_from_partial(&self, partial: &PartialYoutubeEmbed) -> anyhow::Result<()> {
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
impl From<RawYoutubeEmbed> for YoutubeEmbed {
    fn from(value: RawYoutubeEmbed) -> Self {
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
impl From<&YoutubeEmbed> for RawYoutubeEmbed {
    fn from(value: &YoutubeEmbed) -> Self {
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

impl ParseUrlExt for YoutubeUrl {
    fn try_parse(text: String) -> anyhow::Result<Self> {
        match get_id_from_url(&text) {
            Ok(_) => Ok(Self(text)),
            Err(_) => Err(anyhow::anyhow!("")),
        }
    }

    fn get_id(&self) -> &str {
        get_id_from_url(&self.0).expect_ji("Not a valid YouTube url")
    }
}

const ANY_YOUTUBE_DOMAIN: &str = "youtube.com";
const REGULAR_URL_BASE: &str = "https://www.youtube.com/watch?v=";
const SHARE_URL_BASE: &str = "https://youtu.be/";
const EMBED_IFRAME_BASE: &str = "<iframe ";
const EMBED_URL_BASE: &str = "https://www.youtube.com/embed/";

fn get_id_from_url(url: &str) -> Result<&str, ()> {
    let id;
    //when is_id passes all tests, this can be removed
    let mut check_extracted_id = true;
    if is_id(url) {
        return Ok(url);
    } else if url.starts_with(REGULAR_URL_BASE) {
        id = extract_id_regular(url);
    } else if url.starts_with(SHARE_URL_BASE) {
        id = extract_id_share(url);
    } else if url.starts_with(EMBED_IFRAME_BASE) || url.starts_with(EMBED_URL_BASE) {
        id = extract_id_iframe(url);
    } else if url.contains(ANY_YOUTUBE_DOMAIN) {
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
                None => return Err(()),
            },
            _ => return Err(()),
        }
    } else {
        return Err(());
    };

    if !check_extracted_id || is_id(id) {
        Ok(id)
    } else {
        Err(())
    }
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=814bb22fa870f8132b2995c6e99a5221
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

fn extract_id_share(url: &str) -> &str {
    let base_length = SHARE_URL_BASE.len();
    &url[base_length..(base_length + 11)]
}

fn extract_id_iframe(code: &str) -> &str {
    let id_index = code.find(EMBED_URL_BASE).unwrap_ji() + EMBED_URL_BASE.len();
    &code[id_index..(id_index + 11)]
}

fn is_id(id: &str) -> bool {
    let regex = Regex::new(r"^[\w|_|\-]{11}$").unwrap_ji();
    regex.is_match(id)
}

#[cfg(test)]
mod tests {
    use shared::domain::module::body::_groups::design::YoutubeUrl;

    use crate::stickers::embed::types::ParseUrlExt;

    #[test]
    fn can_get_id_from_url() {
        let valid_url_vec = vec![
            "https://www.youtube.com/watch?v=UQosz5VNsjY",
            "https://www.youtube.com/watch?v=UQosz5VNsjY&t=55s",
            "https://youtu.be/UQosz5VNsjY",
            "https://youtu.be/UQosz5VNsjY?t=58",
            r#"<iframe width="560" height="315" src="https://www.youtube.com/embed/UQosz5VNsjY?start=58" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>"#,
            r#"<iframe width='560' height='315' src='https://www.youtube.com/embed/UQosz5VNsjY?start=58' title='YouTube video player' frameborder='0' allow='accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture' allowfullscreen></iframe>"#,
            r#"<iframe src="https://www.youtube.com/embed/UQosz5VNsjY"></iframe>"#,
            "https://www.youtube.com/embed/UQosz5VNsjY",
            "UQosz5VNsjY",
            "https://www.youtube.com/watch?app=desktop&feature=youtu.be&v=7BvLp4VdW1A",
            "https://m.youtube.com/watch?v=0op1YgeiDl8-",
            "חשבון פשוטhttps://www.youtube.com/watch?v=lgZtuGgAZvo",
        ];

        for url in valid_url_vec {
            let id = YoutubeUrl::try_parse(url.to_string());

            assert!(id.is_ok());
        }
    }
}
