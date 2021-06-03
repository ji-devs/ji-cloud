use dominator::clone;
use utils::prelude::*;
use crate::{state::*, steps::state::*};
use shared::domain::jig::module::body::{
    Image,
    memory::{
        ModuleData as RawData,
        Content as RawContent,
        Mode as RawMode,
        Card as RawCard,
        CardPair as RawCardPair
    }
};
use super::state::*;

impl MainCard {
    pub fn replace_card_text(&self, pair_index: usize, side: Side, text: String) {

        self.with_pair(pair_index, side, clone!(text => move |mode, card, other| {
            if mode == Mode::Duplicate {
                other.as_text_mutable().set_neq(text.clone());
            }
            card.as_text_mutable().set_neq(text);
        }));

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                with_raw_pair(content, pair_index, side, clone!(text => move |mode, card, other| {
                    if mode == RawMode::Duplicate {
                        *other = RawCard::Text(text.clone());
                    }
                    *card = RawCard::Text(text.clone());
                }));
            }
        });
    }

    pub fn replace_card_image(&self, pair_index: usize, side: Side, data: Image) {
        self.with_pair(pair_index, side, clone!(data => move |mode, card, other| {
            card.as_image_mutable().set_neq(Some(data));
        }));

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                with_raw_pair(content, pair_index, side, clone!(data => move |mode, card, other| {
                    *card = RawCard::Image(Some(data));
                }));
            }
        });
    }

    fn with_pair<A, F: FnOnce(Mode, &Card, &Card) -> A>(&self, pair_index: usize, main_side: Side, f: F) -> A {
        let mode = self.base.mode;
        let pair = self.base.pairs.lock_ref();
        let pair = pair.get(pair_index).unwrap_ji();
        match main_side {
            Side::Left => {
                f(mode, &pair.0, &pair.1)
            },
            Side::Right => {
                f(mode, &pair.1, &pair.0)
            }
        }
    }
}

fn with_raw_pair<A, F: FnOnce(RawMode, &mut RawCard, &mut RawCard) -> A>(raw: &mut RawContent, pair_index: usize, main_side: Side, f: F) -> A {
    let mode = raw.mode;
    let pair = raw.pairs.get_mut(pair_index).unwrap_ji();
    match main_side {
        Side::Left => {
            f(mode, &mut pair.0, &mut pair.1)
        },
        Side::Right => {
            f(mode, &mut pair.1, &mut pair.0)
        }
    }
}
