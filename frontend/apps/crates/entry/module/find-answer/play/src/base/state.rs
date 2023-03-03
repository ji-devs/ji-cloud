use components::module::_common::play::prelude::*;
use once_cell::sync::OnceCell;
use rand::prelude::*;
use shared::domain::{
    asset::{Asset, AssetId},
    jig::player::{ModuleConfig, PlayerNavigationHandler, Seconds},
    module::{
        body::{
            _groups::design::{Backgrounds, Sticker},
            find_answer::{
                Mode, ModuleData as RawData, Ordering, PlaySettings, Question, QuestionField, Step,
            },
            ModuleAssist, ModuleAssistType,
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
    pub current_question: Mutable<Option<(usize, Rc<Question>)>>,
    /// List of references to sticker elements. This is used primarily for finding the WYSIWYG renderer for text stickers.
    pub sticker_refs: Vec<(OnceCell<HtmlElement>, OnceCell<HtmlElement>)>,
    pub question_field: QuestionField,
    pub module_phase: Mutable<ModulePlayPhase>,
    /// Custom instructions player so that we can handle the on_ended event.
    pub instructions: ModuleAssist,
    /// Feedback to play when the activity ends
    pub feedback: ModuleAssist,
    pub feedback_signal: Mutable<Option<ModuleAssist>>,
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
            .map(|_| (OnceCell::default(), OnceCell::default()))
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
            current_question: Mutable::new(None),
            sticker_refs,
            question_field: content.question_field,
            module_phase: init_args.play_phase,
            instructions: content.base.instructions.always_show(),
            feedback: content.base.feedback,
            feedback_signal: Mutable::new(None),
        });

        *base_ref.borrow_mut() = Some(base.clone());

        base
    }

    pub fn move_to_question(&self, index: usize) {
        if let Some(next_question) = self.questions.get(index) {
            self.current_question
                .set(Some((index, next_question.clone())));

            let module_config = self.get_module_config();
            if let Some(seconds) = module_config.timer {
                IframeAction::new(ModuleToJigPlayerMessage::ResetTimer(seconds))
                    .try_post_message_to_player()
                    .unwrap_ji();
            }
        }
    }
}

impl BaseExt for Base {
    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }

    fn get_module_assist(&self) -> Option<ModuleAssist> {
        Some(self.instructions.clone())
    }

    fn get_feedback(&self) -> ReadOnlyMutable<Option<ModuleAssist>> {
        self.feedback_signal.read_only()
    }

    fn handle_module_assist_ended(&self, module_assist_type: ModuleAssistType) {
        if let ModuleAssistType::Feedback = module_assist_type {
            self.set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Next)))
        }
    }

    fn handle_navigation(&self, message: JigToModulePlayerMessage) {
        match message {
            JigToModulePlayerMessage::Previous => {
                if let Some((idx, _)) = self.current_question.get_cloned() {
                    if idx > 0 {
                        self.move_to_question(idx - 1);
                    } else {
                        IframeAction::new(ModuleToJigPlayerMessage::Previous)
                            .try_post_message_to_player()
                            .unwrap_ji();
                    }
                }
            }
            JigToModulePlayerMessage::Next => {
                if let Some((idx, _)) = self.current_question.get_cloned() {
                    if idx < self.questions.len() - 1 {
                        self.move_to_question(idx + 1);
                    } else {
                        IframeAction::new(ModuleToJigPlayerMessage::Next)
                            .try_post_message_to_player()
                            .unwrap_ji();
                    }
                }
            }
            _ => {}
        }
    }

    fn get_module_config(&self) -> ModuleConfig {
        ModuleConfig {
            navigation_handler: PlayerNavigationHandler::Module,
            timer: self.settings.time_limit.map(|t| Seconds(t)),
        }
    }
}
