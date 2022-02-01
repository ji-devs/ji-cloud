use crate::module::_groups::cards::{edit::state::*, lookup::Side};
use dominator::clone;
use shared::domain::jig::module::body::{
    Image,
    _groups::cards::{BaseContent as RawBaseContent, Card as RawCard, CardContent as RawCardContent, Mode},
};
use utils::prelude::*;

use super::state::*;

impl<RawData: RawDataExt, E: ExtraExt> MainCard<RawData, E> {
    pub fn replace_card_text(&self, pair_index: usize, side: Side, text: String) {
        self.with_pair(
            pair_index,
            side,
            clone!(text => move |mode, card, other| {
                if mode == Mode::Duplicate {
                    other.as_text_mutable().set_neq(text.clone());
                }
                card.as_text_mutable().set_neq(text);
            }),
        );

        self.base.history.push_modify(|raw| {
            if let Some(content) = raw.get_content_mut() {
                with_raw_pair(
                    content,
                    pair_index,
                    side,
                    clone!(text => move |mode, card, other| {
                        if mode == Mode::Duplicate {
                            other.card_content = RawCardContent::Text(text.clone());
                        }
                        card.card_content = RawCardContent::Text(text);
                    }),
                );
            }
        });
    }

    pub fn replace_card_image(&self, pair_index: usize, side: Side, data: Image) {
        self.with_pair(
            pair_index,
            side,
            clone!(data => move |_mode, card, _other| {
                card.as_image_mutable().set_neq(Some(data));
            }),
        );

        self.base.history.push_modify(|raw| {
            if let Some(content) = raw.get_content_mut() {
                with_raw_pair(
                    content,
                    pair_index,
                    side,
                    clone!(data => move |_mode, card, _other| {
                        card.card_content = RawCardContent::Image(Some(data));
                    }),
                );
            }
        });
    }

    pub fn remove_card_image(&self, pair_index: usize, side: Side) {
        self.with_pair(
            pair_index,
            side,
            move |_mode, card, _other| {
                card.as_image_mutable().set_neq(None);
            },
        );

        self.base.history.push_modify(|raw| {
            if let Some(content) = raw.get_content_mut() {
                with_raw_pair(
                    content,
                    pair_index,
                    side,
                    move |_mode, card, _other| {
                        card.card_content = RawCardContent::Image(None);
                    },
                );
            }
        });
    }

    fn with_pair<A, F: FnOnce(Mode, &Card, &Card) -> A>(
        &self,
        pair_index: usize,
        main_side: Side,
        f: F,
    ) -> A {
        let mode = self.base.mode;
        let pair = self.base.pairs.lock_ref();
        let pair = pair.get(pair_index).unwrap_ji();
        match main_side {
            Side::Left => f(mode, &pair.0, &pair.1),
            Side::Right => f(mode, &pair.1, &pair.0),
        }
    }

    pub fn delete_pair(&self, pair_index: usize) {
        self.base.pairs.lock_mut().remove(pair_index);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.get_content_mut() {
                content.pairs.remove(pair_index);
            }
        });
    }

    pub fn close_menu(&self) {
        // Close the menu
        self.menu_open.set_neq(false);
        // And then remove the reference to the popup container element
        self.menu_container_elem.set_neq(None);
    }

    /// Toggles whether this card is selected
    ///
    /// Rules:
    ///
    /// 1. If no cards are selected, select the card at the current index
    /// 1. If one card is selected at the current index, deselect it
    /// 1. If both cards are selected at the current index, select only the other card
    /// 1. If there is a selection at another index, deselect it
    pub fn toggle_selection(&self) {
        if !self.can_select {
            return;
        }

        let current_idx = self.index.get_cloned().unwrap_or_default();
        let selected_pair = self.base.selected_pair.get_cloned();

        match selected_pair {
            Some((selected_idx, selected_side)) => {
                let correct_idx = current_idx == selected_idx;

                match (correct_idx, selected_side) {
                    (true, SelectedSide::One(side)) => {
                        if side == self.side {
                            // If toggling for the current side and index, deselect cards.
                            self.base.selected_pair.set(None);
                        } else {
                            // If toggling on the current index but a different card, select both.
                            self.base.selected_pair.set(Some((selected_idx, SelectedSide::Both)));
                        }
                    },
                    (true, SelectedSide::Both) => {
                        // if toggling when both are selected, deselect the current card.
                        self.base.selected_pair.set(Some((selected_idx, SelectedSide::One(self.side.negate()))));
                    },
                    _ => {
                        self.base.selected_pair.set(None);
                        // Alternatively, we could select cards on a new idx instead of resetting
                        // the selection.
                    }
                }
            },
            None => {
                self.base.selected_pair.set(Some((current_idx, SelectedSide::One(self.side))));
            }
        }
    }
}

fn with_raw_pair<A, F: FnOnce(Mode, &mut RawCard, &mut RawCard) -> A>(
    raw: &mut RawBaseContent,
    pair_index: usize,
    main_side: Side,
    f: F,
) -> A {
    let mode = raw.mode;
    let pair = raw.pairs.get_mut(pair_index).unwrap_ji();
    match main_side {
        Side::Left => f(mode, &mut pair.0, &mut pair.1),
        Side::Right => f(mode, &mut pair.1, &mut pair.0),
    }
}
