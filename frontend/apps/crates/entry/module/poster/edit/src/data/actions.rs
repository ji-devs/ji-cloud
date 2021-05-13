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
use crate::steps::main::renderables::state::Renderable;
use shared::domain::jig::module::body::Renderable as RawRenderable;
use super::state::*;


impl State {



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

    fn set_from_raw(&self, game_data:raw::ModuleData) {
        self.theme_id.set_neq(game_data.theme_id);
        self.instructions.set(game_data.instructions);
    }

    pub fn next_step(&self) {
        self.step.replace_with(|step| match step {
            Step::One => Step::Two,
            Step::Two => Step::Three,
            Step::Three => Step::Four,
            Step::Four => unimplemented!("nothing after step 4!")
        });
    }


    /// Poster specific
    ///

    //Just for pushing history/save
    //the actual modifications are in renderables actions
    //because we want to be able to use that everywhere
    pub fn replace_renderables(&self, renderables:Vec<RawRenderable>) {

        log::info!("TODO - replace renderables!");
        /*
        self.get_history().push_modify(|game_data| {
            game_data.pairs.remove(pair_index);
        });
        */
    }
    pub fn select_renderable(&self, index:usize) {
        self.renderables.selected_index.set(Some(index));


        if let Some(item) = self.renderables.get(index) {
            match item {
                Renderable::Text(text) => {
                    let value = text.value.get_cloned();

                    /*
                    if value.is_empty() {
                        self.text_editor.value.set(None);
                    } else {
                        self.text_editor.value.set(Some(value));
                    }
                    */

                },
                _ => {
                }
            }
        }

    }

    pub fn change_text(&self, value:String) {
        if let Some(text) = self.renderables.get_current_as_text() {
            text.set_value(value);
        }
    }

    pub fn deselect_renderable(&self) {
        self.renderables.selected_index.set(None);
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
            let body = shared::domain::jig::module::ModuleBody::Poster(data);
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

