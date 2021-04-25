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
    pub fn change_step(&self, next:Step) {
        let prev = self.step.get();
        self.step.set(next);
        if prev != Step::Four {
            let mut completed = self.steps_completed.lock_mut();
            completed.insert(prev);
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
        self.instructions.audio_id.set_neq(game_data.instructions.audio_id);
        self.instructions.text.set_neq(game_data.instructions.text);
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

