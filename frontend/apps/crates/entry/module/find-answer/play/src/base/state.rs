use components::module::_common::play::prelude::*;
use once_cell::sync::OnceCell;
use shared::domain::{
    asset::{Asset, AssetId},
    module::{
        body::{
            _groups::design::{Backgrounds, Sticker},
            find_answer::{
                Mode, ModuleData as RawData, PlaySettings, Question, QuestionField, Step,
            },
            Instructions,
        },
        ModuleId,
    },
};
use utils::prelude::*;

use futures_signals::signal::Mutable;
use std::rc::Rc;
use web_sys::HtmlElement;

pub struct Base {
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    pub asset: Asset,
    pub theme_id: ThemeId,
    pub instructions: Instructions,
    pub settings: PlaySettings,
    pub backgrounds: Backgrounds,
    pub stickers: Vec<Sticker>,
    pub questions: Vec<Rc<Question>>,
    /// List of references to sticker elements. This is used primarily for finding the WYSIWYG renderer for text stickers.
    pub sticker_refs: Vec<OnceCell<HtmlElement>>,
    pub question_field: QuestionField,
    pub module_phase: Mutable<ModulePlayPhase>,
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

        Rc::new(Self {
            asset_id,
            module_id,
            asset,
            theme_id,
            instructions: content.base.instructions,
            settings: content.play_settings,
            backgrounds: content.base.backgrounds,
            stickers: content.base.stickers,
            questions: content
                .questions
                .into_iter()
                .map(|question| Rc::new(question))
                .collect(),
            sticker_refs,
            question_field: content.question_field,
            module_phase: init_args.play_phase,
        })
    }
}

impl BaseExt for Base {
    fn get_instructions(&self) -> Option<Instructions> {
        Some(self.instructions.clone())
    }

    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }
}
