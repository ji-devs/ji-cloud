use components::module::_common::play::prelude::*;
use once_cell::sync::OnceCell;
use rand::prelude::*;
use shared::domain::{
    asset::{Asset, AssetId},
    module::{
        body::{
            _groups::design::{Backgrounds, Sticker},
            find_answer::{
                Mode, ModuleData as RawData, Next, Ordering, PlaySettings, Question, QuestionField,
                Step,
            },
            Instructions, InstructionsType,
        },
        ModuleId,
    },
};
use utils::prelude::*;

use futures_signals::signal::{Mutable, ReadOnlyMutable};
use std::{cell::RefCell, rc::Rc};
use web_sys::HtmlElement;

pub struct Base {
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    pub asset: Asset,
    pub theme_id: ThemeId,
    pub settings: PlaySettings,
    pub backgrounds: Backgrounds,
    pub stickers: Vec<Sticker>,
    pub questions: Vec<Rc<Question>>,
    /// List of references to sticker elements. This is used primarily for finding the WYSIWYG renderer for text stickers.
    pub sticker_refs: Vec<OnceCell<HtmlElement>>,
    pub question_field: QuestionField,
    pub module_phase: Mutable<ModulePlayPhase>,
    /// Custom instructions player so that we can handle the on_ended event.
    pub instructions: Instructions,
    /// Feedback to play when the activity ends
    pub feedback: Instructions,
    pub feedback_signal: Mutable<Option<Instructions>>,
}

impl Base {
    pub async fn new(init_args: InitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {
        let InitFromRawArgs {
            asset_id,
            module_id,
            asset,
            raw,
            theme_id,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();

        // Initially we fill this list with `None`. Once we start rendering stickers, we will update the individual items with their relevant refs.
        let sticker_refs = (0..content.base.stickers.len())
            .map(|_| OnceCell::default())
            .collect();

        let base_ref: Rc<RefCell<Option<Rc<Self>>>> = Rc::new(RefCell::new(None));

        let mut questions: Vec<Rc<Question>> = content
            .questions
            .into_iter()
            .map(|question| Rc::new(question))
            .collect();

        if let Ordering::Randomize = content.play_settings.ordering {
            let mut rng = thread_rng();
            questions.shuffle(&mut rng);
        }

        let base = Rc::new(Self {
            asset_id,
            module_id,
            asset,
            theme_id,
            settings: content.play_settings,
            backgrounds: content.base.backgrounds,
            stickers: content.base.stickers,
            questions,
            sticker_refs,
            question_field: content.question_field,
            module_phase: init_args.play_phase,
            instructions: content.base.instructions,
            feedback: content.base.feedback,
            feedback_signal: Mutable::new(None),
        });

        *base_ref.borrow_mut() = Some(base.clone());

        base
    }
}

impl BaseExt for Base {
    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }

    fn get_instructions(&self) -> Option<Instructions> {
        Some(self.instructions.clone())
    }

    fn get_feedback(&self) -> ReadOnlyMutable<Option<Instructions>> {
        self.feedback_signal.read_only()
    }

    fn handle_instructions_ended(&self, instructions_type: InstructionsType) {
        if let InstructionsType::Feedback = instructions_type {
            match self.settings.next {
                Next::SelectAll | Next::SelectSome(_) => {
                    self.set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Next)));
                }
                _ => self.set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Positive))),
            }
        }
    }

    fn get_timer_minutes(&self) -> Option<u32> {
        self.settings.time_limit
    }
}
