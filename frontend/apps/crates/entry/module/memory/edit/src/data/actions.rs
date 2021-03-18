use super::{state::*, history::*, raw};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use dominator::clone;
use components::module::history::state::HistoryState;

impl State {
    pub fn add_card(&self) {
        let game_mode = self.game_mode.get().unwrap_throw();
        //TODO - add pair, and to history too
    }

    pub fn change_step(&self, next:Step) {
        let prev = self.step.get();
        self.step.set(next);
        if prev != Step::Four {
            let mut completed = self.steps_completed.lock_mut();
            completed.insert(prev);
        }
    }

    pub fn change_mode(&self, mode: GameMode) {
        self.game_mode.set(Some(mode));
        self.pairs.lock_mut().clear();
    }


    pub fn set_from_history(&self, history:Option<History>) {
        match history {
            Some(history) => {

                self.pairs.lock_mut().replace_cloned(
                    history.pairs
                        .into_iter()
                        .map(|pair| (pair.0.into(), pair.1.into()))
                        .collect()
                );
            },
            None => {
                self.pairs.lock_mut().clear();
            }
        }
    }

    pub fn replace_single_list(&self, list: Vec<String>) {
        let game_mode = self.game_mode.get().unwrap_throw();

        match game_mode {
            GameMode::Duplicate => {
                let pairs:Vec<(Card, Card)> =
                    list
                        .into_iter()
                        .map(|word| {
                            (
                                Card::new_with_data(CardMode::Text, word.clone()),
                                Card::new_with_data(CardMode::Text, word),
                            )
                        })
                        .collect();
                self.replace_pairs(pairs);

            },
            _ => unimplemented!("can't replace single list in this mode!")
        }

    }

    pub fn replace_card_value(&self, card:&Card, pair_index: usize, side: Side, value: String) {
        card.data.set(Some(value.clone()));
        self.history.push_mix(|history| {
            let card = {
                let pair = &mut history.pairs[pair_index];

                match side {
                    Side::Left => &mut pair.0,
                    Side::Right => &mut pair.1
                }
            };
            *card = raw::Card::Text(value);
        });
    }

    pub fn delete_pair(&self, pair_index: usize) {
        self.pairs.lock_mut().remove(pair_index);
        self.history.push_mix(|history| {
            history.pairs.remove(pair_index);
        });
    }

    //internal only
    fn replace_pairs(&self, pairs:Vec<(Card, Card)>) {
        self.pairs.lock_mut().replace_cloned(pairs.clone());
        self.history.push_mix(move |last| {
            last.pairs = 
                pairs
                    .into_iter()
                    .map(|pair| (pair.0.into(), pair.1.into()))
                    .collect();
        });
    }


}
