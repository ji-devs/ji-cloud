use regex::Regex;
use shared::domain::jig::module::body::{
    Transform,
    _groups::design::{Video, VideoHost, YoutubeUrl},
};
use utils::prelude::*;

pub trait VideoExt {
    fn new(value: VideoHost) -> Self;
}

impl VideoExt for Video {
    /// Create a new Video
    fn new(host: VideoHost) -> Self {
        Self {
            host,
            transform: Transform::identity(),
        }
    }
}

const REGULAR_URL_BASE: &str = "https://www.youtube.com/watch?v=";
const SHARE_URL_BASE: &str = "https://youtu.be/";
const EMBED_IFRAME_BASE: &str = "<iframe ";
const EMBED_URL_BASE: &str = "https://www.youtube.com/embed/";
const LEGACY_WATCH_STRING_BASE: &str = "watch?v=";

fn get_id_from_url(url: &str) -> Result<&str, ()> {
    let id;
    if is_id(url) {
        return Ok(url);
    } else if url.starts_with(REGULAR_URL_BASE) {
        id = extract_id_regular(url);
    } else if url.starts_with(SHARE_URL_BASE) {
        id = extract_id_share(url);
    } else if url.starts_with(EMBED_IFRAME_BASE) || url.starts_with(EMBED_URL_BASE) {
        id = extract_id_iframe(url);
    } else if url.find(LEGACY_WATCH_STRING_BASE).is_some() {
        id = extract_legacy_watch(url);
    } else {
        return Err(());
    };

    if is_id(id) {
        Ok(id)
    } else {
        Err(())
    }
}

fn extract_legacy_watch(url: &str) -> &str {
    let start_bytes = url.find(LEGACY_WATCH_STRING_BASE).unwrap_or(0) + LEGACY_WATCH_STRING_BASE.len();
    let end_bytes = url.find("&").unwrap_or(url.len());
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
    let regex = Regex::new(r"^[\w|_|\-]{11}$").unwrap();
    regex.is_match(id)
}

pub trait YoutubeUrlExt {
    fn try_parse(text: String) -> Result<YoutubeUrl, ()>;
    fn get_id(&self) -> &str;
    fn has_id(url: &str) -> bool;
}

impl YoutubeUrlExt for YoutubeUrl {
    fn try_parse(text: String) -> Result<Self, ()> {
        match get_id_from_url(&text) {
            Ok(_) => Ok(Self(text)),
            Err(_) => Err(()),
        }
    }

    fn get_id(&self) -> &str {
        get_id_from_url(&self.0).expect_ji("Not a valid YouTube url")
    }

    fn has_id(url: &str) -> bool {
        get_id_from_url(url).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use shared::domain::jig::module::body::_groups::design::YoutubeUrl;

    use crate::stickers::video::ext::YoutubeUrlExt;

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
        ];

        for url in valid_url_vec {
            let id = YoutubeUrl::try_parse(url.to_string());

            assert!(id.is_ok());
        }
    }
}
