#![feature(type_alias_impl_trait)]
#![feature(min_type_alias_impl_trait)]
use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable,  SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::debug;
use super::{actions, raw};
use itertools::Itertools;
use std::fmt::Write;
use serde::Deserialize;
use std::collections::HashSet;
use components::{
    module::{page::ModulePageKind, history::state::HistoryState},
    text_editor::state::State as TextEditorState
};
use shared::{domain::{
    jig::{JigId, module::{body::{Audio, Instructions, ThemeOrImage}, ModuleId}},
    audio::AudioId
}, media::MediaLibrary};
use dominator_helpers::futures::AsyncLoader;
use wasm_bindgen_futures::spawn_local;
use utils::prelude::*;
use super::actions::{HistoryChangeFn, HistoryUndoRedoFn};
use crate::{
    steps::main::renderables::state::Renderables,
    overlay::state::State as OverlayState,
};


//See: https://users.rust-lang.org/t/eli5-existential/57780/16?u=dakom
//
//Basically, the type of these callbacks are closures created from *inside*
//Since we don't have the actual type here on the *outside* we can't define it
//However, we do know something about the type - namely, that it will *exist*
//Hence, the so-called "existential" type
//
//For this to be true it must actually be defined eventually though
//so that the compiler can kinda figure it out and fill the type in backwards
pub type HistoryStateImpl = HistoryState<raw::ModuleData, HistoryChangeFn, HistoryUndoRedoFn>;

pub struct State {
    //Common state stuff
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub step: Mutable<Step>,
    pub steps_completed: Mutable<HashSet<Step>>,
    pub theme_id: Mutable<ThemeId>,
    pub instructions: Mutable<Instructions>,
    pub save_loader: Rc<AsyncLoader>,
    pub overlay: OverlayState,
    history: RefCell<Option<Rc<HistoryStateImpl>>>,

    //Poster-specific
    pub bg: Mutable<Option<ThemeOrImage>>, 
    pub fg: Mutable<Option<ThemeOrImage>>, 
    pub renderables: Rc<Renderables>, 
    pub text_editor: Rc<TextEditorState>,
}

impl State {
    pub fn new(jig_id: JigId, module_id: ModuleId, raw_data:raw::ModuleData) -> Rc<Self> {


        let theme_id = raw_data.theme_id;

        let instructions = Mutable::new(raw_data.instructions.clone());

        let step = Mutable::new(match debug::settings().step.as_ref() {
            Some(step) => step.clone(),
            None => Step::One
        });

        let save_loader = Rc::new(AsyncLoader::new());


        let _self_for_text:Rc<RefCell<Option<Rc<Self>>>> = Rc::new(RefCell::new(None));
        let on_text_change = Box::new(clone!(_self_for_text => move |value:&str| {
            if let Some(_self) = _self_for_text.borrow().as_ref() {
                _self.change_text(value.to_string());
            }
        }));

        let _self = Rc::new(Self {
            jig_id,
            module_id,
            step,
            steps_completed: Mutable::new(HashSet::new()),
            theme_id: Mutable::new(theme_id),
            history: RefCell::new(None),
            save_loader,
            instructions,
            overlay: OverlayState::new(),

            bg: Mutable::new(raw_data.bg.clone()),
            fg: Mutable::new(raw_data.fg.clone()),
            renderables: Rc::new(Renderables::new(&raw_data.renderables,None)),
            text_editor: TextEditorState::new(theme_id, None, on_text_change)

        });

        *_self.renderables.on_updated.borrow_mut() = Some(Box::new(clone!(_self => move |renderables| {
           _self.replace_renderables(renderables); 
        })));

        *_self_for_text.borrow_mut() = Some(_self.clone());

        let history = Rc::new(HistoryState::new(
            raw_data,
            actions::history_on_change(_self.clone()),
            actions::history_on_undoredo(_self.clone()),
        ));

        *_self.history.borrow_mut() = Some(history);

        if let Some(index) = crate::debug::settings().selected_index {
            _self.select_renderable(index);
        }
        _self
    }

    pub fn get_history(&self) -> Rc<HistoryStateImpl> {
        self.history.borrow().as_ref().unwrap_ji().clone()
    }
    pub fn theme_id_str_signal(&self) -> impl Signal<Item = &'static str> {
        self.theme_id.signal_ref(|id| id.as_str_id())
    }



    pub fn page_kind_signal(&self) -> impl Signal<Item = ModulePageKind> {
        self.step.signal()
            .map(|step| {
                if step == Step::Four {
                    ModulePageKind::GridResizePreview
                } else {
                    ModulePageKind::GridResize
                }
            })
    }


    pub fn step_ready_signal(&self) -> impl Signal<Item = bool> {
        self.step.signal().map(|step| true)
    }



}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Step {
    One,
    Two,
    Three,
    Four
}

impl Step {
    pub fn label(&self) -> &'static str {
        match self {
            Step::One => crate::strings::steps_nav::STR_THEMES,
            Step::Two => crate::strings::steps_nav::STR_BACKGROUND,
            Step::Three => crate::strings::steps_nav::STR_CONTENT,
            Step::Four => crate::strings::steps_nav::STR_PREVIEW,
        }
    }

    pub fn number(&self) -> u8 {
        match self {
            Step::One => 1, 
            Step::Two => 2, 
            Step::Three => 3, 
            Step::Four => 4 
        }
    }
}

