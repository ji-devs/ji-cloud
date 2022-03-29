use crate::module::_groups::cards::edit::state::*;
use shared::domain::jig::module::body::_groups::cards::{CardPair as RawCardPair, Mode};
use shared::domain::jig::module::body::{Background, ThemeId};
use unicode_segmentation::UnicodeSegmentation;
use utils::prelude::*;

pub fn limit_text(max_len: usize, text: String) -> String {
    let len = text.graphemes(true).count();

    if len > max_len {
        let cutoff_grapheme_byte = text.grapheme_indices(true).nth(max_len).unwrap_ji().0;

        text[..cutoff_grapheme_byte].to_string()
    } else {
        text
    }
}

impl<RawData: RawDataExt, E: ExtraExt> CardsBase<RawData, E> {
    pub fn clear_all(&self) {
        self.pairs.lock_mut().clear();
        self.history.push_modify(|raw| {
            if let Some(content) = raw.get_content_mut() {
                content.pairs.clear();
            }
        });
    }

    pub fn replace_single_list(&self, list: Vec<String>) {
        let mode = self.mode;

        match mode {
            Mode::Duplicate | Mode::Lettering => {
                let pairs: Vec<(Card, Card)> = list
                    .into_iter()
                    .map(|word| (Card::new_text(word.clone()), Card::new_text(word)))
                    .collect();
                self.replace_pairs(pairs);
            }
            Mode::WordsAndImages => {
                let pairs: Vec<(Card, Card)> = list
                    .into_iter()
                    .map(|word| (Card::new_text(word), Card::new_image(None)))
                    .collect();
                self.replace_pairs(pairs);
            }
            _ => unimplemented!("can't replace single list in this mode!"),
        }
    }

    pub fn replace_dual_list(&self, list: Vec<(String, String)>) {
        let pairs: Vec<(Card, Card)> = list
            .into_iter()
            .map(|(word_1, word_2)| (Card::new_text(word_1), Card::new_text(word_2)))
            .collect();
        self.replace_pairs(pairs);
    }

    fn replace_pairs(&self, pairs: Vec<(Card, Card)>) {
        self.pairs.lock_mut().replace_cloned(pairs.clone());
        self.history.push_modify(move |raw| {
            if let Some(content) = raw.get_content_mut() {
                content.pairs = pairs
                    .into_iter()
                    .map(|pair| RawCardPair(pair.0.into(), pair.1.into()))
                    .collect();
            }
        });
    }

    /// Replace a pair at `idx` with the pair returned by `f`
    ///
    /// `f` is passed the pair originally at `idx` in the list of pairs.
    pub fn replace_pair<F>(&self, idx: usize, f: F)
    where
        F: FnOnce((Card, Card)) -> (Card, Card),
    {
        let mut pairs = self.pairs.lock_mut();
        let pair = f(pairs.remove(idx));
        pairs.insert_cloned(idx, pair.clone());

        self.history.push_modify(move |raw| {
            if let Some(content) = raw.get_content_mut() {
                content.pairs.remove(idx);
                content
                    .pairs
                    .insert(idx, RawCardPair(pair.0.into(), pair.1.into()))
            }
        });
    }

    pub fn add_pair(&self) {
        let pair = match self.mode {
            Mode::WordsAndImages => (Card::new_text("".to_string()), Card::new_image(None)),
            Mode::Images => (Card::new_image(None), Card::new_image(None)),
            _ => (
                Card::new_text("".to_string()),
                Card::new_text("".to_string()),
            ),
        };

        self.pairs.lock_mut().push_cloned(pair.clone());

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.get_content_mut() {
                content
                    .pairs
                    .push(RawCardPair(pair.0.into(), pair.1.into()));
            }
        });
    }

    pub fn set_theme(&self, theme: ThemeId) {
        self.theme_id.set_neq(theme);

        self.history.push_modify(move |raw| {
            if let Some(content) = raw.get_content_mut() {
                content.theme = theme;
            }
        });
    }

    pub fn set_bg(&self, background: Background) {
        let bg = Some(background);
        self.background.set(bg.clone());

        self.history.push_modify(move |raw| {
            if let Some(content) = raw.get_content_mut() {
                content.background = bg;
            }
        });
    }
}
