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
            memory::{
                Mode, 
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
    pub jig: Option<Jig>,
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub step: ReadOnlyMutable<Step>,
    pub theme: Mutable<ThemeChoice>,
    pub instructions: Mutable<Instructions>,
    pub audio_mixer: AudioMixer,

    pub mode: Mode,
    pub tooltips: Tooltips,

    //memory
    pub pairs: MutableVec<(Card, Card)>,
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
    pub async fn new(
        audio_mixer: AudioMixer,
        jig_id: JigId,
        module_id: ModuleId,
        jig: Option<Jig>,
        raw:RawData, 
        step: ReadOnlyMutable<Step>,
        history: Rc<HistoryStateImpl<RawData>>
    ) -> Rc<Self> {

        let content = raw.content.unwrap_ji();

        let pairs:Vec<(Card, Card)> = content.pairs
            .iter()
            .map(|pair| {
                (pair.0.clone().into(), pair.1.clone().into())
            })
            .collect();

        let theme = Mutable::new(content.theme);
        let mode = content.mode.into();
        let instructions = Mutable::new(content.instructions);

        let _self = Rc::new(Self {
            jig_id,
            module_id,
            jig,
            history,
            step,
            theme,
            instructions,
            audio_mixer,
            mode,
            tooltips: Tooltips::new(),
            pairs: MutableVec::new_with_values(pairs)
        });

        _self
    }
    pub fn get_theme_id(&self) -> ThemeId {
        match self.theme.get_cloned() {
            ThemeChoice::Jig => {
                // self.jig.as_ref().unwrap_ji().theme_id.clone()
                log::warn!("waiting on jig settings");
                ThemeId::Chalkboard
            },
            ThemeChoice::Override(theme_id) => theme_id
        }
    }
    pub fn theme_id_signal(&self) -> impl Signal<Item = ThemeId> {
        self.theme.signal_cloned()
            .map(|theme| match theme {
                ThemeChoice::Jig => {
                    // self.jig.as_ref().unwrap_ji().theme_id.clone()
                    log::warn!("waiting on jig settings");
                    ThemeId::Chalkboard
                },
                ThemeChoice::Override(theme_id) => theme_id
            })
    }

    pub fn theme_id_str_signal(&self) -> impl Signal<Item = &'static str> {
        self.theme_id_signal().map(|id| id.as_str_id())
    }

    pub fn pairs_len_signal(&self) -> impl Signal<Item = usize> {
        self.pairs.signal_vec_cloned().len()
    }

    pub fn is_empty_signal(&self) -> impl Signal<Item = bool> {
        self.pairs_len_signal()
            .map(|len| len <= 0)
            .dedupe()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Step {
    One,
    Two, 
    Three,
    Four,
}

impl Default for Step {
    fn default() -> Self {
        Self::One
    }
}

impl StepExt for Step {
    fn next(&self) -> Option<Self> {
        match self {
            Self::One => Some(Self::Two),
            Self::Two => Some(Self::Three),
            Self::Three => Some(Self::Four),
            Self::Four => None,
        }
    }

    fn as_number(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::One => crate::strings::steps_nav::STR_CONTENT,
            Self::Two => crate::strings::steps_nav::STR_DESIGN,
            Self::Three => crate::strings::steps_nav::STR_SETTINGS,
            Self::Four => crate::strings::steps_nav::STR_PREVIEW,
        }
    }

    fn get_list() -> Vec<Self> {
        vec![
            Self::One,
            Self::Two,
            Self::Three,
            Self::Four,
        ]
    }
    fn get_preview() -> Self {
        Self::Four
    }
}
