use components::{instructions::player::InstructionsPlayer, module::_common::play::prelude::*};
use dominator::clone;
use once_cell::sync::OnceCell;
use rand::prelude::*;
use shared::domain::{
    asset::{Asset, AssetId},
    module::{
        body::{
            _groups::design::{Backgrounds, Sticker},
            find_answer::{
                Mode, ModuleData as RawData, Ordering, PlaySettings, Question, QuestionField, Step,
            },
            Instructions,
        },
        ModuleId,
    },
};
use utils::prelude::*;

use futures_signals::signal::Mutable;
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
    pub instructions_player: Rc<InstructionsPlayer>,
    /// Whether the instructions player has completed playback.
    pub instructions_finished: Mutable<bool>,
    /// Feedback to play when the activity ends
    pub feedback: Instructions,
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
            instructions_player: InstructionsPlayer::new(
                content.base.instructions,
                Some(clone!(base_ref => move || {
                    if let Some(base_ref) = &*base_ref.borrow() {
                        base_ref.instructions_finished.set_neq(true);
                    }
                })),
            ),
            instructions_finished: Mutable::new(false),
            feedback: content.base.feedback,
        });

        *base_ref.borrow_mut() = Some(base.clone());

        base
    }
}

impl BaseExt for Base {
    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }

    fn get_timer_minutes(&self) -> Option<u32> {
        self.settings.time_limit
    }
}
