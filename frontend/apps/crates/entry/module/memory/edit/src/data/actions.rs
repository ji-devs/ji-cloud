/*
 * note that history actions are done imperatively
 * usually via push_modify
 */

use super::{state::*, history::History, raw};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use dominator::clone;
use components::module::history::state::HistoryState;
use futures_signals::signal::Mutable;
use utils::prelude::*;
use dominator_helpers::futures::AsyncLoader;

use shared::{
    api::endpoints::{ApiEndpoint, self, module::*}, 
    domain::{
        audio::AudioId, 
        jig::{*, module::*}
    }, 
    error::{EmptyError, MetadataNotFound},
    media::MediaLibrary
};

impl State {
    pub fn add_card(&self) {
        let game_mode = self.game_mode.get().unwrap_ji();
        let raw_pair = match game_mode {
            GameMode::Duplicate => {
                (
                    raw::Card::Text("".to_string()),
                    raw::Card::Text("".to_string()),
                )
            },
            _ => unimplemented!("unknown!")
        };

        let pair:(Card, Card) = (
            raw_pair.0.clone().into(),
            raw_pair.1.clone().into()
        );

        self.pairs.lock_mut().push_cloned(pair);

        self.history.push_modify(move |history| {
            if let Some(game_data) = &mut history.game_data {
                game_data.pairs.push((raw_pair.0.into(), raw_pair.1.into()));
            }
        });
    }

    pub fn change_theme(&self, theme:String) {
        self.theme.set_neq(theme.clone());
        self.history.push_modify(move |history| {
            if let Some(game_data) = &mut history.game_data {
                game_data.theme = theme;
            }
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

    pub fn change_mode(&self, mode: GameMode) {
        self.history.push_modify(move |history| {
            match mode {
                GameMode::Duplicate => {
                    history.game_data = Some(raw::GameData::new(
                        mode,
                        crate::config::get_themes_cloned()[0].clone(),
                        raw::Instructions::new(), 
                        Vec::<(&str, &str)>::new()
                    ));
                },
                _ => unimplemented!("TODO - change mode")
            };
        });

        self.set_from_history(Some(self.history.get_current()));
    }



    pub fn replace_single_list(&self, list: Vec<String>) {
        let game_mode = self.game_mode.get().unwrap_ji();

        match game_mode {
            GameMode::Duplicate | GameMode::Lettering => {
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
            _ => unimplemented!("can't replace single list in this mode!")
        }

    }


    pub fn replace_card_text(&self, pair_index: usize, side: Side, text: String) {

        self.with_pair(pair_index, side, clone!(text => move |game_mode, card, other| {
            if game_mode == GameMode::Duplicate {
                other.as_text_mutable().set_neq(text.clone());
            }
            card.as_text_mutable().set_neq(text);
        }));

        self.history.push_modify(|history| {
            if let Some(game_data) = &mut history.game_data {
                with_raw_pair(game_data, pair_index, side, clone!(text => move |game_mode, card, other| {
                    if game_mode == GameMode::Duplicate {
                        *other = raw::Card::Text(text.clone());
                    }
                    *card = raw::Card::Text(text.clone());
                }));
            }
        });
    }

    pub fn change_instructions_text(&self, text: Option<String>) {
        self.instructions.text.set_neq(text.clone());
        self.history.push_modify(move |history| {
            if let Some(game_data) = &mut history.game_data {
                game_data.instructions.text = text;
            }
        });
    }

    pub fn change_instructions_audio(&self, audio_id: Option<AudioId>) {
        log::info!("CHANGING INSTRUCTIONS AUDIO!!!!");
        self.instructions.audio_id.set_neq(audio_id.clone());

        self.history.push_modify(move |history| {
            if let Some(game_data) = &mut history.game_data {
                game_data.instructions.audio_id = audio_id;
            }
        });
    }
    pub fn delete_pair(&self, pair_index: usize) {
        self.pairs.lock_mut().remove(pair_index);
        self.history.push_modify(|history| {
            if let Some(game_data) = &mut history.game_data {
                game_data.pairs.remove(pair_index);
            }
        });
    }

    fn with_pair<A, F: FnOnce(GameMode, &Card, &Card) -> A>(&self, pair_index: usize, main_side: Side, f: F) -> A {
        let game_mode = self.game_mode.get().unwrap_ji();
        let pair = self.pairs.lock_ref();
        let pair = pair.get(pair_index).unwrap_ji();
        match main_side {
            Side::Left => {
                f(game_mode, &pair.0, &pair.1)
            },
            Side::Right => {
                f(game_mode, &pair.1, &pair.0)
            }
        }
    }

    fn replace_pairs(&self, pairs:Vec<(Card, Card)>) {
        self.pairs.lock_mut().replace_cloned(pairs.clone());
        self.history.push_modify(move |last| {
            if let Some(game_data) = &mut last.game_data {
                game_data.pairs = 
                    pairs
                        .into_iter()
                        .map(|pair| (pair.0.into(), pair.1.into()))
                        .collect();
            }
        });
    }

    //Doesn't update history of course
    pub fn set_from_history(&self, history:Option<History>) {
        match history {
            Some(history) => {
                self.set_from_raw(history.game_data);
            },
            None => {
                self.set_from_raw(None);
            }
        }
    }

    fn set_from_raw(&self, game_data:Option<raw::GameData>) {
        match game_data {
            Some(game_data) => {
                self.pairs.lock_mut().replace_cloned(
                    game_data.pairs
                        .into_iter()
                        .map(|pair| (pair.0.into(), pair.1.into()))
                        .collect()
                );
                self.game_mode.set_neq(Some(game_data.mode));
                self.theme.set_neq(game_data.theme);
                self.instructions.audio_id.set_neq(game_data.instructions.audio_id);
                self.instructions.text.set_neq(game_data.instructions.text);
            },
            None => {
                self.pairs.lock_mut().clear();
                self.game_mode.set_neq(None);
                self.theme.set_neq("".to_string());
                self.instructions.audio_id.set_neq(None);
                self.instructions.text.set_neq(None);
            }
        }

    }

}

pub fn save(save_loader: Rc<AsyncLoader>, module_id: ModuleId, data: Option<raw::GameData>) {

    //Note - there's currently no way to save a None... 
    if let Some(value) = data.map(|data| serde_json::to_value(&data).unwrap_ji()) {

        if crate::debug::settings().live_save {
            save_loader.load(async move {
                log::info!("SAVING...");
                let path = Update::PATH.replace("{id}",&module_id.0.to_string());

                let req = Some(ModuleUpdateRequest {
                    kind: None,
                    body: Some(value), 
                });
                api_with_auth_empty::<EmptyError, _>(&path, Update::METHOD, req).await; //.expect_ji("error saving module!");
                log::info!("SAVED!");
            });
        } else {
            //log::info!("SKIPPING SAVE - DEBUG!");
        }
    } else {
        log::info!("SKIPPING SAVE - NO DATA!");
    }
}

//internal only
fn with_raw_pair<A, F: FnOnce(raw::Mode, &mut raw::Card, &mut raw::Card) -> A>(game_data: &mut raw::GameData, pair_index: usize, main_side: Side, f: F) -> A {
    let game_mode = game_data.mode.clone();
    let pair = game_data.pairs.get_mut(pair_index).unwrap_ji();
    match main_side {
        Side::Left => {
            f(game_mode, &mut pair.0, &mut pair.1)
        },
        Side::Right => {
            f(game_mode, &mut pair.1, &mut pair.0)
        }
    }
}
