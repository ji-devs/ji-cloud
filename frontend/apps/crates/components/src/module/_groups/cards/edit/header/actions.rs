use super::state::*;
use crate::module::_groups::cards::edit::state::*;
use shared::domain::jig::module::body::_groups::cards::{CardPair as RawCardPair, Card as RawCard, Mode};

impl<RawData: RawDataExt, E: ExtraExt> Header<RawData, E> {
    pub fn add_pair(&self) {
        let pair = match self.base.mode {
            Mode::WordsAndImages => {
                (Card::new_text("".to_string()), Card::new_image(None))
            },
            _ => {
                (Card::new_text("".to_string()), Card::new_text("".to_string()))
            }
        };


        self.base.pairs.lock_mut().push_cloned(pair.clone());


        self.base.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.get_content_mut() {
                content.pairs.push(RawCardPair(pair.0.into(), pair.1.into()));
            }
        });
    }
}
