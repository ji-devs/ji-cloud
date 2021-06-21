mod card;
pub use card::*;

use components::module::edit::prelude::*;
use web_sys::AudioContext;
use std::rc::Rc;
use shared::domain::jig::{
    JigId, 
    Jig,
    module::{
        ModuleId, 
        body::{
            ThemeChoice,
            Audio,
            Instructions,
            Background,
            memory::{
                Mode, 
                Step,
                Content as RawContent, 
                ModuleData as RawData
            }
        }
    }
};
use futures_signals::{
    map_ref,
    signal::{ReadOnlyMutable, Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
use utils::prelude::*;
use components::{
    audio_mixer::AudioMixer,
    tooltip::state::State as TooltipState
};
use dominator::clone;
use std::cell::RefCell;

pub struct Base {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig_theme_id: Mutable<ThemeId>,
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub step: ReadOnlyMutable<Step>,
    pub theme_choice: Mutable<ThemeChoice>,
    pub theme_id: ReadOnlyMutable<ThemeId>,
    pub instructions: Mutable<Instructions>,
    pub audio_mixer: AudioMixer,

    pub mode: Mode,
    pub tooltips: Tooltips,

    //memory
    pub pairs: MutableVec<(Card, Card)>,
    pub background: Mutable<Option<Background>>,
}

pub struct Tooltips {
    pub delete: Mutable<Option<Rc<TooltipState>>>,
    pub list_error: Mutable<Option<Rc<TooltipState>>>
}
impl Tooltips {
    pub fn new() -> Self {
        Self {
            delete: Mutable::new(None),
            list_error: Mutable::new(None),
        }
    }
}
impl Base {
    pub async fn new(init_args: BaseInitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {

        let BaseInitFromRawArgs { 
            raw,
            jig_id,
            module_id,
            jig_theme_id,
            theme_id,
            history,
            step,
            theme_choice,
            audio_mixer,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();

        let pairs:Vec<(Card, Card)> = content.pairs
            .iter()
            .map(|pair| {
                (pair.0.clone().into(), pair.1.clone().into())
            })
            .collect();


        let mode = content.mode.into();
        let instructions = Mutable::new(content.instructions);

        let background = Mutable::new(content.background);

        let _self = Rc::new(Self {
            jig_id,
            module_id,
            jig_theme_id,
            history,
            step: step.read_only(),
            theme_choice,
            theme_id,
            instructions,
            audio_mixer,
            mode,
            tooltips: Tooltips::new(),
            pairs: MutableVec::new_with_values(pairs),
            background,
        });

        _self
    }

    pub fn pairs_len_signal(&self) -> impl Signal<Item = usize> {
        self.pairs.signal_vec_cloned().len()
    }

    pub fn is_empty_signal(&self) -> impl Signal<Item = bool> {
        self.pairs_len_signal()
            .map(|len| len <= 0)
            .dedupe()
    }

    pub fn theme_id_str_signal(&self) -> impl Signal<Item = &'static str> { 
        self.theme_id.signal().map(|id| id.as_str_id())
    }
}

//the requirement for this indirection might be a compiler bug...
//I couldn't reproduce it on playground
//here was the latest attempt: https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=75e158fa8d226b8fdc505ec8551ca259

impl BaseExt<Step> for Base {
    type NextStepAllowedSignal = impl Signal<Item = bool>;

    fn allowed_step_change(&self, from:Step, to:Step) -> bool {
        if self.pairs.lock_ref().len() >= 2 {
            true
        } else {
            false
        }
    }

    fn next_step_allowed_signal(&self) -> Self::NextStepAllowedSignal {
        map_ref! {
            let pairs_len = self.pairs_len_signal(),
            let step = self.step.signal()
                => {
                    if *pairs_len >= 2 {
                        true
                    } else {
                        false
                    }
                }
        }
    }
    

}
