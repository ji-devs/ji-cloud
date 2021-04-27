/*
 * note that history actions are done imperatively
 * usually via push_modify
 */

use super::{state::*, raw};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use dominator::clone;
use components::module::history::state::HistoryState;
use futures_signals::signal::Mutable;
use utils::prelude::*;
use dominator_helpers::futures::AsyncLoader;
use unicode_segmentation::UnicodeSegmentation;
use shared::domain::jig::module::body::{Audio, Instructions};
pub type HistoryChangeFn = impl Fn(Option<raw::ModuleData>);
pub type HistoryUndoRedoFn = impl Fn(Option<raw::ModuleData>);
use shared::{
    api::endpoints::{ApiEndpoint, self, jig::module::*}, 
    domain::{
        image::ImageId,
        audio::AudioId, 
        jig::{*, module::*}
    }, 
    error::{EmptyError, MetadataNotFound},
    media::MediaLibrary
};

impl State {
    pub fn add_card(&self) {
        let mode = self.mode.get().unwrap_ji();
        let raw_pair = match mode {
            Mode::WordsAndImages => {
                (
                    raw::Card::Text("".to_string()),
                    raw::Card::Image(None)
                )
            },
            _ => {
                (
                    raw::Card::Text("".to_string()),
                    raw::Card::Text("".to_string()),
                )
            },
        };

        let pair:(Card, Card) = (
            raw_pair.0.clone().into(),
            raw_pair.1.clone().into()
        );

        self.pairs.lock_mut().push_cloned(pair);

        self.get_history().push_modify(move |game_data| {
            game_data.pairs.push(raw::CardPair(raw_pair.0.into(), raw_pair.1.into()));
        });
    }

    pub fn change_theme_id(&self, theme_id:ThemeId) {
        self.theme_id.set_neq(theme_id);
        self.get_history().push_modify(move |game_data| {
            game_data.theme_id = theme_id;
        });
    }

    pub fn change_step(&self, next:Step) {
        let prev = self.step.get();
        self.step.set(next);
        if prev != Step::Four {
            let mut completed = self.steps_completed.lock_mut();
            completed.insert(prev);
        }
    }

    pub fn change_mode(&self, mode: Mode) {
        let game_data = raw::ModuleData::new(
            mode,
            ThemeId::None, 
            Instructions::default(), 
            Vec::<(&str, &str)>::new()
        );

        self.get_history().push_modify(clone!(game_data => move |history_game_data| {
            *history_game_data = game_data;
        }));

        self.set_from_raw(game_data);
    }



    pub fn replace_single_list(&self, list: Vec<String>) {
        let mode = self.mode.get().unwrap_ji();

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
    pub fn replace_card_text(&self, pair_index: usize, side: Side, text: String) {

        self.with_pair(pair_index, side, clone!(text => move |mode, card, other| {
            if mode == Mode::Duplicate {
                other.as_text_mutable().set_neq(text.clone());
            }
            card.as_text_mutable().set_neq(text);
        }));

        self.get_history().push_modify(|game_data| {
            with_raw_pair(game_data, pair_index, side, clone!(text => move |mode, card, other| {
                if mode == Mode::Duplicate {
                    *other = raw::Card::Text(text.clone());
                }
                *card = raw::Card::Text(text.clone());
            }));
        });
    }

    pub fn replace_card_image(&self, pair_index: usize, side: Side, data: (ImageId, MediaLibrary)) {
        self.with_pair(pair_index, side, clone!(data => move |mode, card, other| {
            card.as_image_mutable().set_neq(Some(data));
        }));

        self.get_history().push_modify(|game_data| {
            with_raw_pair(game_data, pair_index, side, clone!(data => move |mode, card, other| {
                *card = raw::Card::Image(Some(data));
            }));
        });
    }


    pub fn delete_pair(&self, pair_index: usize) {
        self.pairs.lock_mut().remove(pair_index);
        self.get_history().push_modify(|game_data| {
            game_data.pairs.remove(pair_index);
        });
    }

    pub fn clear_all(&self) {
        self.pairs.lock_mut().clear();
        self.get_history().push_modify(|game_data| {
            game_data.pairs.clear();
        });
    }

    pub fn limit_text(&self, max_len: usize, text:String) -> String {
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

    pub fn save_instructions(&self, instructions: Instructions, also_history:bool) {
        if(also_history) {
            self.get_history().push_modify(|game_data| {
                game_data.instructions = instructions;
            });
        } else {
            self.save_without_history(|game_data| {
                game_data.instructions = instructions;
            })
        }
    }
    //Usually saving goes through the history mechanism. when it doesn't this can be used
    //It pulls from the latest history in order to mixin
    fn save_without_history(&self, f: impl FnOnce(&mut raw::ModuleData)) {
        let mut data = self.get_history().get_current();
        f(&mut data);

        save(
            self.save_loader.clone(), 
            self.jig_id.clone(), 
            self.module_id.clone(), 
            data
        );
    }
    fn with_pair<A, F: FnOnce(Mode, &Card, &Card) -> A>(&self, pair_index: usize, main_side: Side, f: F) -> A {
        let mode = self.mode.get().unwrap_ji();
        let pair = self.pairs.lock_ref();
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

    fn replace_pairs(&self, pairs:Vec<(Card, Card)>) {
        self.pairs.lock_mut().replace_cloned(pairs.clone());
        self.get_history().push_modify(move |game_data| {
            game_data.pairs = 
                pairs
                    .into_iter()
                    .map(|pair| raw::CardPair(pair.0.into(), pair.1.into()))
                    .collect();
        });
    }


    fn set_from_raw(&self, game_data:raw::ModuleData) {
        match game_data.mode {
            Some(mode) => {
                self.pairs.lock_mut().replace_cloned(
                    game_data.pairs
                        .into_iter()
                        .map(|pair| (pair.0.into(), pair.1.into()))
                        .collect()
                );
                self.mode.set_neq(Some(mode));
                self.theme_id.set_neq(game_data.theme_id);
                self.instructions.set(game_data.instructions);
            },
            None => {
                self.pairs.lock_mut().clear();
                self.mode.set_neq(None);
                self.theme_id.set_neq(ThemeId::None);
                self.instructions.set(Instructions::default());
            }
        }

    }

    pub fn next_step(&self) {
        self.step.replace_with(|step| match step {
            Step::One => Step::Two,
            Step::Two => Step::Three,
            Step::Three => Step::Four,
            Step::Four => unimplemented!("nothing after step 4!")
        });
    }

}

pub fn history_on_change(state: Rc<State>) -> HistoryChangeFn {
    move |game_data:Option<raw::ModuleData>| {
        save(state.save_loader.clone(), state.jig_id.clone(), state.module_id.clone(), game_data.unwrap_or_default());
    }
}
//Does not update history or save
//Saving happens like any other onchange
pub fn history_on_undoredo(state: Rc<State>) -> HistoryUndoRedoFn {
    move |game_data:Option<raw::ModuleData>| {
        state.set_from_raw(game_data.unwrap_or_default());
    }
}
pub fn save(save_loader: Rc<AsyncLoader>, jig_id: JigId, module_id: ModuleId, data: raw::ModuleData) {
    if crate::debug::settings().live_save {
        save_loader.load(async move {
            let body = shared::domain::jig::module::ModuleBody::MemoryGame(data);
            log::info!("SAVING...");
            let path = Update::PATH
                .replace("{id}",&jig_id.0.to_string())
                .replace("{module_id}",&module_id.0.to_string());

            let req = Some(ModuleUpdateRequest {
                index: None,
                body: Some(body), 
            });
            api_with_auth_empty::<EmptyError, _>(&path, Update::METHOD, req).await; //.expect_ji("error saving module!");
            log::info!("SAVED!");
        });
    } else {
        //log::info!("SKIPPING SAVE - DEBUG!");
    }
}

//internal only
fn with_raw_pair<A, F: FnOnce(raw::Mode, &mut raw::Card, &mut raw::Card) -> A>(game_data: &mut raw::ModuleData, pair_index: usize, main_side: Side, f: F) -> A {
    let mode = game_data.mode.unwrap_ji();
    let pair = game_data.pairs.get_mut(pair_index).unwrap_ji();
    match main_side {
        Side::Left => {
            f(mode, &mut pair.0, &mut pair.1)
        },
        Side::Right => {
            f(mode, &mut pair.1, &mut pair.0)
        }
    }
}
