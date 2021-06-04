use unicode_segmentation::UnicodeSegmentation;
use utils::prelude::*;
use crate::{state::*, base::state::*};
use shared::domain::jig::module::body::memory::{
    CardPair as RawCardPair
};
use super::state::*;

impl MainPair {
    pub fn delete_pair(&self, pair_index: usize) {
        self.base.pairs.lock_mut().remove(pair_index);
        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                content.pairs.remove(pair_index);
            }
        });
    }
}
