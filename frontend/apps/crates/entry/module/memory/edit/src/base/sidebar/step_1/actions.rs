use unicode_segmentation::UnicodeSegmentation;
use utils::prelude::*;
use crate::{state::*, base::state::*};
use shared::domain::jig::module::body::memory::{
    CardPair as RawCardPair
};
use shared::domain::jig::module::body::memory::Mode; 

pub fn limit_text(max_len: usize, text:String) -> String {
    let len = text.graphemes(true).count();

    if len > max_len {
        let cutoff_grapheme_byte = text
            .grapheme_indices(true)
            .nth(max_len)
            .unwrap_ji()
            .0;

        text[..cutoff_grapheme_byte].to_string()
    } else {
        text
    }
}

impl Base {
    pub fn clear_all(&self) {
        self.pairs.lock_mut().clear();
        self.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                content.pairs.clear();
            }
        });
    }

    pub fn replace_single_list(&self, list: Vec<String>) {
        let mode = self.mode;

        match mode {
            Mode::Duplicate | Mode::Lettering => {
                let pairs:Vec<(Card, Card)> =
                    list
                        .into_iter()
                        .map(|word| {
                            (
                                Card::new_text(word.clone()),
                                Card::new_text(word),
                            )
                        })
                        .collect();
                self.replace_pairs(pairs);

            },
            Mode::WordsAndImages => {
                let pairs:Vec<(Card, Card)> =
                    list
                        .into_iter()
                        .map(|word| {
                            (
                                Card::new_text(word),
                                Card::new_image(None)
                            )
                        })
                        .collect();
                self.replace_pairs(pairs);

            },
            _ => unimplemented!("can't replace single list in this mode!")
        }

    }


    pub fn replace_dual_list(&self, list: Vec<(String, String)>) {
        let pairs:Vec<(Card, Card)> =
            list
                .into_iter()
                .map(|(word_1, word_2)| {
                    (
                        Card::new_text(word_1),
                        Card::new_text(word_2),
                    )
                })
                .collect();
        self.replace_pairs(pairs);
    }

    fn replace_pairs(&self, pairs:Vec<(Card, Card)>) {
        self.pairs.lock_mut().replace_cloned(pairs.clone());
        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.pairs = 
                    pairs
                        .into_iter()
                        .map(|pair| RawCardPair(pair.0.into(), pair.1.into()))
                        .collect();
            }
        });
    }
}
