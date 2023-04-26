use super::ParseUrlExt;
use anyhow::Context;
use futures_signals::signal::Mutable;
use regex::Regex;
use shared::domain::module::body::_groups::design::{QuizletEmbed as RawQuizletEmbed, QuizletId};
use utils::unwrap::UnwrapJiExt;

#[derive(Clone, Debug, Default)]
pub struct PartialQuizletEmbed {
    pub url: Mutable<Option<QuizletId>>,
}
impl PartialQuizletEmbed {
    pub fn full(&self) -> anyhow::Result<QuizletEmbed> {
        Ok(QuizletEmbed {
            url: Mutable::new(self.url.get_cloned().context("")?),
        })
    }
}

#[derive(Clone, Debug)]
pub struct QuizletEmbed {
    pub url: Mutable<QuizletId>,
}
impl QuizletEmbed {
    pub fn partial(&self) -> PartialQuizletEmbed {
        PartialQuizletEmbed {
            url: Mutable::new(Some(self.url.get_cloned())),
        }
    }
    pub fn update_from_partial(&self, partial: &PartialQuizletEmbed) -> anyhow::Result<()> {
        self.url.set_neq(partial.url.get_cloned().context("")?);
        Ok(())
    }
}
impl From<RawQuizletEmbed> for QuizletEmbed {
    fn from(value: RawQuizletEmbed) -> Self {
        Self {
            url: Mutable::new(value.url),
        }
    }
}
impl From<&QuizletEmbed> for RawQuizletEmbed {
    fn from(value: &QuizletEmbed) -> Self {
        Self {
            url: value.url.get_cloned(),
        }
    }
}

impl ParseUrlExt for QuizletId {
    fn try_parse(text: String) -> anyhow::Result<Self> {
        match get_id_from_url(&text) {
            Ok(_) => Ok(Self(text)),
            Err(_) => Err(anyhow::anyhow!("")),
        }
    }

    fn get_id(&self) -> &str {
        get_id_from_url(&self.0).expect_ji("Not a valid Quizlet url")
    }
}

const SHARE_URL_BASE: &str = "https://quizlet.com/";
const EMBED_IFRAME_BASE: &str = "<iframe ";
const ID_LENGTH: usize = 9;

fn get_id_from_url(url: &str) -> Result<&str, ()> {
    let id;

    if is_id(url) {
        return Ok(url);
    } else if url.starts_with(SHARE_URL_BASE) {
        id = extract_id_share(url);
    } else if url.starts_with(EMBED_IFRAME_BASE) {
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
    let regex = Regex::new(&format!("^[0-9]{{{ID_LENGTH}}}$")).unwrap_ji();

    regex.is_match(id)
}

#[cfg(test)]
mod tests {
    use shared::domain::module::body::_groups::design::QuizletId;

    use crate::stickers::embed::types::ParseUrlExt;

    #[test]
    fn can_get_id_from_url() {
        let valid_url_vec = vec![
            r#"<iframe src="https://quizlet.com/603057919/flashcards/embed?i=3rxkfr&x=1jj1" width="1200" height="780" ></iframe>"#,
            "https://quizlet.com/603057919/flashcards/embed?i=3rxkfr&x=1jj1",
            "603057919",
        ];

        for url in valid_url_vec {
            let id = QuizletId::try_parse(url.to_string());

            assert!(id.is_ok());
        }
    }
}
