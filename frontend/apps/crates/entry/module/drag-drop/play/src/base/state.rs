use components::module::_common::play::prelude::*;
use shared::domain::{
    asset::{Asset, AssetId},
    module::{
        body::{
            Instructions,
            _groups::design::Backgrounds,
            drag_drop::{
                Item, Mode, ModuleData as RawData, PlaySettings, Step, TargetArea, TargetTransform,
            },
            InstructionsType,
        },
        ModuleId,
    },
};
use utils::prelude::*;

use futures_signals::signal::{Mutable, ReadOnlyMutable};
use std::rc::Rc;

pub struct Base {
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    pub asset: Asset,
    pub theme_id: ThemeId,
    pub instructions: Instructions,
    pub feedback: Instructions,
    pub feedback_signal: Mutable<Option<Instructions>>,
    pub settings: PlaySettings,
    pub backgrounds: Backgrounds,
    pub items: Vec<Item>,
    pub item_targets: Vec<TargetTransform>,
    pub target_areas: Vec<TargetArea>,
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

        Rc::new(Self {
            asset_id,
            module_id,
            asset,
            theme_id,
            instructions: content.instructions,
            feedback: content.feedback,
            feedback_signal: Mutable::new(None),
            settings: content.play_settings,
            backgrounds: content.backgrounds,
            items: content.items,
            item_targets: content.item_targets,
            target_areas: content.target_areas,
            module_phase: init_args.play_phase,
        })
    }
}

impl BaseExt for Base {
    fn get_instructions(&self) -> Option<Instructions> {
        Some(self.instructions.clone())
    }

    fn get_feedback(&self) -> ReadOnlyMutable<Option<Instructions>> {
        self.feedback_signal.read_only()
    }

    fn handle_instructions_ended(&self, instructions_type: InstructionsType) {
        if let InstructionsType::Feedback = instructions_type {
            self.set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Next)));
        }
    }

    fn get_timer_seconds(&self) -> Option<u32> {
        self.settings.time_limit
    }

    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }
}
