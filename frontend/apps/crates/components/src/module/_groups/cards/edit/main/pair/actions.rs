use unicode_segmentation::UnicodeSegmentation;
use utils::prelude::*;
use shared::domain::jig::module::body::_groups::cards::CardPair as RawCardPair;
use super::state::*;
use crate::module::_groups::cards::edit::state::*;

impl <RawData: RawDataExt, E: ExtraExt> MainPair <RawData, E> {
    pub fn delete_pair(&self, pair_index: usize) {
        self.base.pairs.lock_mut().remove(pair_index);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.get_content_mut() {
                content.pairs.remove(pair_index);
            }
        });
    }
}
